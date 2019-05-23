use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

use serde::{Deserialize, Serialize};
use structopt::StructOpt;
use url::Url;

use pc::{backends, build_client, BackendConfig};

#[derive(Debug, StructOpt)]
/// Command line paste service client.
struct Opt {
    /// Use a custom config file
    #[structopt(long = "config", short = "c")]
    config_file: Option<String>,

    /// Select which user-defined server to use
    #[structopt(long = "server", short = "s")]
    server: Option<String>,

    #[structopt(subcommand)]
    cmd: Option<OptCommand>,
}

#[derive(Debug, StructOpt)]
enum OptCommand {
    #[structopt(name = "dump-config")]
    /// Print the configuration as currently used.
    DumpConfig,
    #[structopt(name = "list-servers")]
    /// List the available configured servers
    ListServers,
    #[structopt(name = "list-backends")]
    /// List the available backends
    ListBackends,
    #[structopt(name = "backend-info")]
    /// Print more information about a backend
    BackendInfo { name: String },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Config {
    main: MainConfig,
    servers: HashMap<String, BackendConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct MainConfig {
    default: Option<String>,
}

impl std::default::Default for Config {
    fn default() -> Self {
        Config {
            main: MainConfig {
                default: Some("paste_rs".to_owned()),
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

fn do_paste(opt: Opt, config: Config) -> Result<(), Box<dyn Error>> {
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
    let server_choice: String = opt.server.unwrap_or_else(|| {
        config
            .main
            .default
            .clone()
            .unwrap_or_else(|| config.servers.keys().next().unwrap().to_owned())
    });

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
    Ok(())
}

fn read_config(path: &str) -> Result<Config, Box<dyn Error>> {
    let data = read_file(path)?;
    let config = toml::from_str(&data)?;
    Ok(config)
}

fn run(opt: Opt) -> Result<(), Box<dyn Error>> {
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

    match opt.cmd {
        None => do_paste(opt, config),
        Some(OptCommand::DumpConfig) => {
            println!("{}", toml::to_string(&config)?);
            Ok(())
        }
        Some(OptCommand::ListServers) => {
            for (key, server) in config.servers.iter() {
                println!("{} => {}", key, server);
            }
            Ok(())
        }
        Some(OptCommand::ListBackends) => {
            for name in backends::BACKEND_NAMES {
                println!("{}", name);
            }
            Ok(())
        }
        Some(OptCommand::BackendInfo { name }) => match backends::info_from_str(&name) {
            Ok(s) => {
                println!("{}", s);
                Ok(())
            }
            Err(s) => Err(s.into()),
        },
    }
}

fn main() {
    let opt = Opt::from_args();

    std::process::exit(match run(opt) {
        Err(err) => {
            eprintln!("{}", err);
            1
        }
        Ok(_) => 0,
    });
}
