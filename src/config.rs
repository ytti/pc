use std::collections::HashMap;
use std::default::Default;
use std::env;
use std::error::Error;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::backends::BackendConfig;
use crate::utils::read_file;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub main: MainConfig,
    pub servers: HashMap<String, BackendConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MainConfig {
    pub server: Option<String>,
    pub histfile: Option<String>,
}

impl Config {
    pub fn with_server_override(self, new_server: Option<String>) -> Self {
        Config {
            main: MainConfig {
                server: match new_server {
                    Some(ref c) if c.as_str() == "NONE" => None,
                    _ => new_server.or(self.main.server),
                },
                ..self.main
            },
            ..self
        }
    }

    pub fn with_histfile_override(self, new_histfile: Option<String>) -> Self {
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

impl Default for Config {
    fn default() -> Self {
        toml::from_str(include_str!("../default_config.toml"))
            .expect("default config should be correct")
    }
}

pub fn choose_config_file(
    file_override: &Option<String>,
) -> Result<Option<String>, Box<dyn Error>> {
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

pub fn read_config(path: &str) -> Result<Config, Box<dyn Error>> {
    let data = read_file(path)?;
    let config = toml::from_str(&data)?;
    Ok(config)
}
