use std::{path::Path};

use clap::{App, AppSettings, Arg, SubCommand};

mod config;
use config::*;

fn main() {
    let matches = App::new("azalea")
        .version("1.0.0")
        .about("Package manager for minecraft")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("init")
            .about("Initializes azalea in this folder")
            .long_about("Initializes azalea in this directory. The directory should be a world's saves folder.")
            .arg(
                Arg::with_name("name")
                    .help("The name of the package")
                    .required(true)
            )
            .arg(
                Arg::with_name("folder")
                    .help("The folder to initialize in")
            )
        )
        .subcommand(
            SubCommand::with_name("add")
                .about("Adds a package")
                .arg(
                    Arg::with_name("package")
                        .required(true)
                )
        )
        .subcommand(
            SubCommand::with_name("info")
                .about("Displays info about the package")
        )
        .get_matches();
    if let Some(matches) = matches.subcommand_matches("init") {
        let folder = match matches.value_of("folder") {
            Some(folder) => {
                println!("Initializing azalea in {}", folder);
                Path::new(folder)
            }
            None => {
                println!("Initializing azalea in this directory");
                Path::new(".")
            }
        };
        let config_path = folder.join("azalea.json");
        let config = Config {
            name: matches.value_of("name").unwrap().into(),
            version: "1.0.0".into(),
            packages: vec![]
        };
        serde_json::to_writer_pretty(
            std::fs::OpenOptions::new().write(true).create(true).open(config_path).expect("Could not open azalea.json"),
            &config
        ).expect("Could not write to azalea.json");
    } else {
        // Read config
        let mut config = serde_json::from_reader::<_, Config>(
            std::fs::OpenOptions::new().read(true).open(Path::new("azalea.json")).expect("Could not find azalea.json")
        ).expect("Could not parse azalea.json");
        // Match command
        match matches.subcommand() {
            ("add", Some(matches)) => {
                let package = matches.value_of("package").unwrap();
                println!("Adding package {}", package);
                config.packages.push(Package {
                    name: package.into(),
                    version: "1.0.0".into()
                });
            }
            ("info", _) => {
                println!(
r#"Package {}
Version: {}
Dependencies:"#,
                    config.name, config.version
                );
                for p in &config.packages {
                    println!("\t- {} ({})", p.name, p.version)
                }
            }
            _ => unreachable!()
        }
        // Write config
        serde_json::to_writer_pretty(
            std::fs::OpenOptions::new().write(true).open("azalea.json").expect("Could not open azalea.json"),
            &config
        ).expect("Could not write to azalea.json");
    }
}
