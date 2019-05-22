use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

use serde::Deserialize;
use structopt::StructOpt;

use pc::{build_client, BackendConfig, PasteClient};

#[derive(Debug, StructOpt)]
/// Command line paste service client.
struct Opt {
    /// Use a custom config file
    #[structopt(long = "config", short = "c")]
    config_file: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
    main: MainConfig,
    servers: HashMap<String, BackendConfig>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct MainConfig {
    default: String,
}

impl std::default::Default for Config {
    fn default() -> Self {
        Config {
            main: MainConfig {
                default: "paste_rs".to_owned(),
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

fn read_config(fname: Option<String>) -> Result<Config, Box<dyn std::error::Error>> {
    match fname {
        Some(s) => {
            let data = read_file(&s)?;
            let config = toml::from_str(&data)?;
            Ok(config)
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

fn run(opt: Opt) -> Result<(), Box<dyn std::error::Error>> {
    dbg!(&opt);

    let config = read_config(opt.config_file)?;
    dbg!(&config);

    let server_choice: String = config.main.default;

    let client_config: &BackendConfig = match config.servers.get(&server_choice) {
        Some(choice) => choice,
        None => {
            return Err("helo".into());
        }
    };

    let data = read_stdin()?;

    let client = build_client(client_config);

    let paste_info = client.paste(data)?;

    println!("{}", paste_info);
    Ok(())
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
