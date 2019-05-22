use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

use serde::{Deserialize, Serialize};
use structopt::StructOpt;

use pc::{build_client, BackendConfig, PasteClient};

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
                    BackendConfig::Generic {
                        url: "https://paste.rs/".to_owned(),
                    },
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

fn read_config(fname: &Option<String>) -> Result<Config, Box<dyn Error>> {
    match fname {
        Some(s) => {
            if s.as_str() == "NONE" {
                Ok(Config::default())
            } else {
                let data = read_file(&s)?;
                let config = toml::from_str(&data)?;
                Ok(config)
            }
        }
        None => {
            let config_dir = match env::var("XDG_CONFIG_HOME") {
                Ok(val) => val,
                Err(_) => match env::var("HOME") {
                    Ok(val) => format!("{}/.config", val),
                    Err(e) => {
                        return Err(Box::new(e));
                    }
                },
            };

            let config_file = format!("{}/pc/config.toml", config_dir);

            if Path::new(&config_file).exists() {
                let data = read_file(&config_file)?;
                let config = toml::from_str(&data)?;
                Ok(config)
            } else {
                Ok(Config::default())
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
    if config.servers.len() < 1 {
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

    let client_config: &BackendConfig = match config.servers.get(&server_choice) {
        Some(choice) => choice,
        None => {
            // TODO: more helpful error message
            return Err(format!("No corresponding server config for {}", server_choice).into());
        }
    };

    let data = read_stdin()?;
    let client = build_client(client_config);
    let paste_url = client.paste(data)?;

    // send the url to stdout!
    println!("{}", paste_url);
    Ok(())
}

fn run(opt: Opt) -> Result<(), Box<dyn Error>> {
    let config = read_config(&opt.config_file)?;

    match opt.cmd {
        None => do_paste(opt, config),
        Some(OptCommand::DumpConfig) => {
            println!("{}", toml::to_string(&config)?);
            Ok(())
        }
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
