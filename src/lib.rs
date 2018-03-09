#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![feature(plugin)]

extern crate failure;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate failure_derive;

extern crate clap;
extern crate eval;
extern crate regex;
extern crate serde;
extern crate serde_json;

use std::error::Error;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

mod command;
mod config;
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

use config::Configuration;

fn extract(config: &Configuration) -> Result<(), Box<Error>> {
    let target_path = Path::new(&config.target_dir);
    if !target_path.exists() {
        fs::create_dir_all(target_path)?;

        if let Some(extractor) = config.get_extractor() {
            println!("Extractor: {:#?}", extractor);

            for archive in &config.archives {
                extractor.extract(archive, &config.target_dir)?;
            }
        } else {
            //return Err(Box::new(Error()));
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

    let target_dir = PathBuf::from(&config.target_dir);

    let mut executables =
        utils::recursive_find(&config.target_dir, executor.executables.as_slice())?;
    executables.sort();
    utils::strip_prefix(&mut executables, &target_dir)?;

    if executables.len() > 1 {
        let mut menu = menu::Menu::from(&executables);
        menu.display();
        let selected: Vec<&usize> = menu.get_selected().iter().collect();
        let executable = &executables[*selected[0]];
        executor.run(executable, &target_dir)?;
    } else if executables.len() == 1 {
        executor.run(&PathBuf::from(&executables[0]), &target_dir)?;
    } else {
        println!("Could not find any suitable executables.");
    }

    Ok(())
}

pub fn run() -> Result<(), Box<Error>> {
    let config = Configuration::load()?;
    extract(&config)?;
    execute(&config)?;
    Ok(())
}
