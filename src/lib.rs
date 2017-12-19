#[macro_use]
extern crate serde_derive;

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

use configuration::Configuration;

fn extract(config: &Configuration) -> Result<(), Box<Error>> {
    if !config.target_dir.as_ref().unwrap().as_path().exists() {
        let extractor = config.get_extractor().unwrap();
        println!("Extractor: {:#?}", extractor);

        fs::create_dir_all(&config.target_dir.as_ref().unwrap())?;
        extractor.extract(
            config.archive.as_ref().unwrap(),
            config.target_dir.as_ref().unwrap(),
        )?;
    }

    Ok(())
}

fn execute(config: &Configuration) -> Result<(), Box<Error>> {
    let executor = config.get_profile().unwrap();
    println!("Executor: {:#?}", executor);

    let mut executables =
        utils::recursive_find(config.target_dir.as_ref().unwrap(), &executor.executables())?;
    utils::strip_prefix(&mut executables, config.target_dir.as_ref().unwrap())?;

    if executables.len() > 1 {
        let menu = menu::Menu::from(&executables);
        executor.run(
            &PathBuf::from(menu.display()),
            config.target_dir.as_ref().unwrap(),
        )?;
    } else if executables.len() == 1 {
        executor.run(
            &PathBuf::from(&executables[0]),
            config.target_dir.as_ref().unwrap(),
        )?;
    } else {
        println!("Could not find any suitable executables.");
    }

    Ok(())
}

pub fn run(args: clap::ArgMatches) -> Result<(), Box<Error>> {
    let mut config = Configuration::new(args)?;
    config.set_defaults();

    extract(&config)?;
    execute(&config)?;

    Ok(())
}
