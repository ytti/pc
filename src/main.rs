use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

use clap::{App, AppSettings, Arg, SubCommand};
use serde::{Deserialize, Serialize};
use url::Url;

use pc::{backends, build_client, BackendConfig};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Config {
    main: MainConfig,
    servers: HashMap<String, BackendConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct MainConfig {
    server: Option<String>,
    histfile: Option<String>,
}

impl Config {
    fn with_histfile_override(self, new_histfile: Option<String>) -> Self {
        Config {
            main: MainConfig {
                histfile: match new_histfile {
                    Some(ref c) if c.as_str() == "NONE" => None,
                    _ => new_histfile.or(self.main.histfile),
                },
                ..self.main
            },
            ..self
        }
    }
}

impl std::default::Default for Config {
    fn default() -> Self {
        Config {
            main: MainConfig {
                server: Some("paste_rs".to_owned()),
                histfile: None,
            },
            servers: {
                let mut servers = HashMap::new();
                servers.insert(
                    "paste_rs".to_owned(),
                    BackendConfig::Generic(backends::generic::Config {
                        url: Url::parse("https://paste.rs/").unwrap(),
                    }),
                );
                servers.insert(
                    "vpaste".to_owned(),
                    BackendConfig::Vpaste(backends::vpaste::Config {
                        url: Url::parse("http://vpaste.net/").unwrap(),
                    }),
                );
                servers
            },
        }
    }
}

fn read_file(fname: &str) -> io::Result<String> {
    let mut file = File::open(fname)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn choose_config_file(file_override: &Option<String>) -> Result<Option<String>, Box<dyn Error>> {
    match file_override {
        Some(s) => {
            // file override, use if exists, else err
            if s == "NONE" {
                Ok(None)
            } else {
                if Path::new(s).exists() {
                    Ok(Some(s.to_owned()))
                } else {
                    Err(format!("config file not found: {:?}", s).into())
                }
            }
        }
        None => {
            // no file override; find a file in the default locations
            let config_dir = match env::var("XDG_CONFIG_HOME") {
                Ok(val) => val,
                Err(_) => format!("{}/.config", env::var("HOME")?),
            };

            let config_file = format!("{}/pc/config.toml", config_dir);

            if Path::new(&config_file).exists() {
                Ok(Some(config_file))
            } else {
                Ok(None)
            }
        }
    }
}

fn read_stdin() -> io::Result<String> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer)?;

    Ok(buffer)
}

fn do_paste(config: Config) -> Result<(), Box<dyn Error>> {
    // sanity checking
    if config.servers.is_empty() {
        return Err(r#"No servers defined in configuration!
Define one in the config file like:

    [servers.rs]
    backend = "generic"
    url = "https://paste.rs/""#
            .into());
    }

    // -s cli arg > config file > random server
    let server_choice: String = config
        .main
        .server
        .clone()
        .unwrap_or_else(|| config.servers.keys().next().unwrap().to_owned());

    // we're removing from the config here because we want an owned object, not a reference
    let client_config: BackendConfig = match config.servers.get(&server_choice) {
        Some(choice) => choice.to_owned(),
        None => {
            return Err(format!(
                r#"No corresponding server config for {0}.
To use this, add a server block under the heading [servers.{0}] in the config toml file."#,
                server_choice
            )
            .into());
        }
    };

    let data = read_stdin()?;

    let client = build_client(client_config);
    let paste_url = client.paste(data)?;

    // send the url to stdout!
    println!("{}", paste_url);

    if let Some(ref path) = config.main.histfile {
        match write_hist(paste_url, path) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("error writing to histfile: {}", path);
                return Err(e);
            }
        }
    }

    Ok(())
}

fn write_hist(paste_url: Url, path: &str) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new().append(true).create(true).open(path)?;
    file.write(format!("{}\n", paste_url).as_bytes())?;
    Ok(())
}

fn read_config(path: &str) -> Result<Config, Box<dyn Error>> {
    let data = read_file(path)?;
    let config = toml::from_str(&data)?;
    Ok(config)
}

fn run() -> Result<(), Box<dyn Error>> {
    let app = App::new("pc")
        .version("0.1.0")
        .author("author")
        .setting(AppSettings::AllowExternalSubcommands)
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Set a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("histfile")
                .short("H")
                .long("histfile")
                .value_name("FILE")
                .help("Set a custom file to log to")
                .takes_value(true),
        )
        .subcommand(SubCommand::with_name("list").about("List info about available server blocks"))
        .subcommand(SubCommand::with_name("list-backends").about("List available backends"))
        .subcommand(
            SubCommand::with_name("dump-config").about("Dump current config serialized as toml"),
        )
        .subcommand(
            SubCommand::with_name("show-backend")
                .arg(Arg::with_name("backend"))
                .about("Show information about a backend"),
        );

    let matches = app.get_matches();

    let op: Op = match matches.subcommand() {
        ("list", _m) => Op::List,
        ("dump-config", _m) => Op::DumpConfig,
        ("list-backends", _m) => Op::ListBackends,
        ("show-backend", Some(m)) => {
            Op::ShowBackend(m.value_of("backend").expect("required param").to_owned())
        }
        (external, Some(ext_m)) => {
            if matches.is_present("op") {
                return Err("Extra commands can't be used when in paste mode"
                    .to_owned()
                    .into());
            }
            let ext_args: Vec<String> =
                ext_m.values_of("").unwrap().map(|s| s.to_owned()).collect();

            Op::Paste {
                server: Some(external.to_owned()),
                server_args: ext_args,
            }
        }
        ("", None) => Op::Paste {
            server: None,
            server_args: vec![],
        },
        _ => unreachable!(),
    };

    let opt = Opt {
        histfile: matches.value_of("histfile").map(|s| s.to_owned()),
        config_file: matches.value_of("config").map(|s| s.to_owned()),
        op,
    };

    let fname: Option<String> = choose_config_file(&opt.config_file)?;
    let config = match fname {
        Some(path) => match read_config(&path) {
            Ok(config) => config,
            Err(e) => {
                eprintln!("error with config file: {}", path);
                return Err(e);
            }
        },
        None => Config::default(),
    };

    let config = config.with_histfile_override(opt.histfile);

    match opt.op {
        Op::Paste {
            server,
            server_args,
        } => {
            // TODO: merge config and paste args and send to do_paste.
            // need more complex stuff to allow each backend to provide its own clap app which we
            // can pass the server_args to.
            do_paste(config)
        },
        Op::DumpConfig => {
            println!("{}", toml::to_string(&config)?);
            Ok(())
        }
        Op::List => {
            for (key, server) in config.servers.iter() {
                println!(
                    "{0} => {1}{2}",
                    key,
                    server,
                    if &config.main.server == &Some(key.to_owned()) {
                        " [default]"
                    } else {
                        ""
                    }
                );
            }
            Ok(())
        }
        Op::ListBackends => {
            for name in backends::BACKEND_NAMES {
                println!("{}", name);
            }
            Ok(())
        }
        Op::ShowBackend(name) => match backends::info_from_str(&name) {
            Ok(s) => {
                println!("{}", s);
                Ok(())
            }
            Err(s) => Err(s.into()),
        },
    }
}

#[derive(Debug, Clone)]
struct Opt {
    config_file: Option<String>,
    op: Op,
    histfile: Option<String>,
}

#[derive(Debug, Clone)]
enum Op {
    Paste {
        server: Option<String>,
        server_args: Vec<String>,
    },
    List,
    ShowBackend(String),
    ListBackends,
    DumpConfig,
}

fn main() {
    std::process::exit(match run() {
        Err(err) => {
            eprintln!("{}", err);
            1
        }
        Ok(_) => 0,
    });
}
