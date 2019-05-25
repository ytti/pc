use std::default::Default;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::utils::BackendConfig;

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

