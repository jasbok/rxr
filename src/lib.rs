#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

extern crate clap;
extern crate regex;
extern crate serde;
extern crate serde_json;

use std::error::Error;
use std::fs;
use std::path::PathBuf;

mod command;
mod configuration;
mod extractor;
mod mappings;
mod menu;
mod profile;
mod utils;

use configuration::ConfigurationSource;
use configuration::Configuration;

fn extract(config: &Configuration) -> Result<(), Box<Error>> {
    if !config.target_dir.as_path().exists() {
        let extractor = config.get_extractor().unwrap();
        println!("Extractor: {:#?}", extractor);

        fs::create_dir_all(&config.target_dir)?;
        extractor.extract(&config.archive, &config.target_dir)?;
    }

    Ok(())
}

fn execute(config: &Configuration) -> Result<(), Box<Error>> {
    let executor = config.get_profile().unwrap();
    println!("Executor: {:#?}", executor);

    let executable_regex = &executor.executables_regex()?;

    println!("Executables Regex: {:#?}", executable_regex);

    let mut executables = utils::recursive_find(&config.target_dir, executable_regex)?;
    executables.sort();
    utils::strip_prefix(&mut executables, &config.target_dir)?;

    println!("Executables: {:#?}", executables);

    if executables.len() > 1 {
        let menu = menu::Menu::from(&executables);
        executor.run(
            &PathBuf::from(menu.display()),
            &config.target_dir,
        )?;
    } else if executables.len() == 1 {
        executor.run(
            &PathBuf::from(&executables[0]),
            &config.target_dir,
        )?;
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
