use std::error::Error;

use clap::{crate_authors, crate_version, App, AppSettings, Arg, SubCommand};

mod backends;
mod config;
mod error;
mod types;
mod utils;

use crate::backends::{BackendConfig, BACKENDS_INFO};
use crate::config::{choose_config_file, read_config, Config};
use crate::utils::{read_stdin, write_hist};

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

fn do_paste(config: Config, mut server_args: Vec<String>) -> Result<(), Box<dyn Error>> {
    // sanity checking
    if config.servers.is_empty() {
        return Err(r#"No servers defined in configuration!
Define one in the config file like:

    [servers.rs]
    backend = "paste_rs"
    url = "https://paste.rs/""#
            .into());
    }

    // config file if set, otherwise arbitrary server
    let server_choice: String = config
        .main
        .server
        .clone()
        .unwrap_or_else(|| config.servers.keys().next().unwrap().to_owned());

    let backend_config: BackendConfig = match config.servers.get(&server_choice) {
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

    server_args.insert(0, server_choice.clone());

    let mut backend = backend_config.clone().extract_backend();

    match backend.apply_args(server_args) {
        Err(e) => {
            match e.kind {
                clap::ErrorKind::HelpDisplayed => {
                    eprintln!(
                        "[servers.{}]\n{}---\n",
                        server_choice,
                        toml::to_string(&backend_config).expect("must be valid")
                    );
                }
                _ => {}
            }
            e.exit();
        }
        Ok(_) => {}
    };

    let data = read_stdin()?;
    let paste_url = backend.paste(data)?;

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

fn run() -> Result<(), Box<dyn Error>> {
    let app = App::new("pc")
        .version(crate_version!())
        .author(crate_authors!())
        .setting(AppSettings::AllowExternalSubcommands)
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Set a custom config file. \"NONE\" forces use of default")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("histfile")
                .short("H")
                .long("histfile")
                .value_name("FILE")
                .help("Set a custom file to log to. \"NONE\" disables")
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
            let ext_args: Vec<String> = match ext_m.values_of("") {
                Some(values) => values.map(|s| s.to_owned()).collect(),
                None => vec![],
            };

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

    match opt.op {
        Op::Paste {
            server,
            server_args,
        } => {
            let config = config
                .with_server_override(server)
                .with_histfile_override(opt.histfile);
            do_paste(config, server_args)
        }
        Op::DumpConfig => {
            println!("{}", toml::to_string(&config)?);
            Ok(())
        }
        Op::List => {
            for (key, backend_config) in config.servers.into_iter() {
                println!(
                    "{0} => {1}{2}",
                    key,
                    backend_config.extract_backend(),
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
            let mut names = BACKENDS_INFO.keys().collect::<Vec<&&str>>();
            names.sort();
            for name in names {
                println!("{}", name);
            }
            Ok(())
        }
        Op::ShowBackend(name) => match BACKENDS_INFO.get(name.as_str()) {
            Some(s) => {
                println!("{}", s);
                Ok(())
            }
            None => Err(format!("{} is not a valid backend", name).into()),
        },
    }
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
