#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![feature(plugin)]
#![plugin(error_def)]


#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;

extern crate clap;
extern crate eval;
extern crate regex;
extern crate serde;
extern crate serde_json;

use std::error::Error;
use std::fs;
use std::path::PathBuf;

mod command;
mod configuration;
mod deserialisers;
pub mod dosbox_config;
mod extractor;
mod feature;
pub mod filters;
mod mappings;
mod menu;
mod profile;
pub mod template;
mod utils;

use configuration::Configuration;

fn extract(config: &Configuration) -> Result<(), Box<Error>> {
    if !config.target_dir.as_path().exists() {
        let extractor = config.get_extractor().unwrap();
        println!("Extractor: {:#?}", extractor);

        fs::create_dir_all(&config.target_dir)?;

        for archive in &config.archives {
            extractor.extract(archive, &config.target_dir)?;
        }
    }

    Ok(())
}

fn determine_executor(config: &Configuration) -> Result<&profile::Profile, Box<Error>> {
    let files = utils::recursive_find_all(&config.target_dir)?;

    let file_paths: Vec<&str> = files
        .iter()
        .map(|file| file.as_path().to_str().unwrap())
        .collect();

    let (profile, score) = config
        .profiles
        .iter()
        .map(|(key, profile)| (key, profile.feature_score(&file_paths)))
        .max_by_key(|&(_, score)| score)
        .unwrap();

    println!("Determined the following profile: {} ({})", profile, score);

    Ok(&config.profiles[profile])
}

fn execute(config: &Configuration) -> Result<(), Box<Error>> {
    let executor = config.get_profile().unwrap_or(determine_executor(config)?);

    let mut executables =
        utils::recursive_find(&config.target_dir, executor.executables.as_slice())?;
    executables.sort();
    utils::strip_prefix(&mut executables, &config.target_dir)?;

    if executables.len() > 1 {
        let menu = menu::Menu::from(&executables);
        executor.run(&PathBuf::from(menu.display()), &config.target_dir)?;
    } else if executables.len() == 1 {
        executor.run(&PathBuf::from(&executables[0]), &config.target_dir)?;
    } else {
        println!("Could not find any suitable executables.");
    }

    Ok(())
}

pub fn run(args: &clap::ArgMatches) -> Result<(), Box<Error>> {
    let config = Configuration::load(args)?;
    extract(&config)?;
    execute(&config)?;
    Ok(())
}
