#[macro_use]
extern crate serde_derive;

extern crate regex;
extern crate serde;
extern crate serde_json;

use std::error::Error;
use std::fs;
use std::path::PathBuf;

pub mod args;
mod command;
mod config;
mod extractor;
mod mappings;
mod menu;
mod paths;
mod profile;
mod utils;

use args::Args;
use config::Config;
use mappings::Mappings;

fn read_config(path: &PathBuf) -> Result<Config, Box<Error>> {
    let conf = match Config::open(&path) {
        Ok(conf) => Ok(conf),
        Err(ref err) if err.is::<serde_json::Error>() => {
            Err(format!("Could not parse config: {})", err))
        }
        Err(err) => Err(format!("Could not load config: {})", err)),
    }?;

    Ok(conf)
}

pub fn run(args: Args) -> Result<(), Box<Error>> {
    let mut mappings = Mappings::new();
    mappings.insert("archive", &args.archive.to_str().unwrap());
    mappings.insert(
        "archive.name",
        &args.archive
            .as_path()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap(),
    );

    let mut config = read_config(&args.config)?;
    config.apply_mappings(&mut mappings);

    println!("mappings: {:#?}", mappings);
    println!("Archive: {:#?}", &config.paths.target);
    println!("Archive Name: {}", mappings.get("archive.name").unwrap());

    fs::create_dir_all(&config.paths.target)?;

    match config.get_extractor(&args.archive) {
        Some(extractor) => {
            extractor.extract()?;
        }
        None => {
            fs::copy(
                &args.archive,
                format!(
                    "{}/{}",
                    &config.paths.target.to_str().unwrap(),
                    mappings.get("archive.name").unwrap()
                ),
            )?;
        }
    };

    let active_profile = config.profiles.get_mut(&args.profile).unwrap();

    let mut executables =
        utils::recursive_find(&config.paths.target, &active_profile.executables())?;
    utils::strip_prefix(&mut executables, &config.paths.target)?;

    let menu = menu::Menu::from(&executables);

    let selection = menu.display();
    mappings.insert("executable", selection);

    active_profile.run(&mappings)?;

    Ok(())
}
