#[macro_use]
extern crate serde_derive;

extern crate regex;
extern crate serde;
extern crate serde_json;

use std::error::Error;
use std::fmt;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use regex::Regex;


mod archive;
mod config;
mod mappings;
mod menu;
mod paths;
mod profile;
mod utils;

use archive::Archive;
use config::Config;
use mappings::Mappings;

pub struct Args<'a> {
    profile: &'a str,
    archive: &'a str,
    config: &'a str,
}

impl<'a> Args<'a> {
    pub fn new(args: &'a [String]) -> Result<Args<'a>, &'static str> {
        if args.len() < 4 {
            return Err("Need to provide at least profile, archive and config arguments.");
        }

        Ok(Args::<'a> {
            profile: &args[1],
            archive: &args[2],
            config: &args[3],
        })
    }
}


impl<'a> fmt::Display for Args<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{ profile: {}, archive: {}, config: {} }}",
            self.profile, self.archive, self.config
        )
    }
}

fn read_config(path: &str) -> Result<Config, Box<Error>> {
    let conf = match Config::open(&path) {
        Ok(conf) => Ok(conf),
        Err(ref err) if err.is::<serde_json::Error>() => {
            Err(format!("Could not parse config: {})", err))
        }
        Err(err) => Err(format!("Could not load config: {})", err)),
    }?;

    Ok(conf)
}

pub fn extract_archive(archive: &Archive, target: &PathBuf) -> Result<(), Box<Error>> {
    if fs::read_dir(&target).is_err() {
        println!("Extracting archive: {}", &target.to_str().unwrap());

        fs::create_dir_all(&target)?;

        let output = archive.extract(&target)?;

        if !output.status.success() {
            println!(
                "Archive stderr: {}",
                String::from_utf8(output.stderr).unwrap()
            );
        }
    } else {
        println!("Archive already extracted: {}", &target.to_str().unwrap());
    }

    Ok(())
}

pub fn run(args: Args) -> Result<(), Box<Error>> {
    let archive = Archive::from(&args.archive);

    let mut mappings = Mappings::new();
    mappings.insert("archive", &archive.name());

    let mut config = read_config(args.config)?;
    config.apply_mappings(&mut mappings);

    extract_archive(&archive, &config.paths.target)?;

    let active_profile = config.profiles.get(args.profile).unwrap();

    let mut executables =
        utils::recursive_find(&config.paths.target, &active_profile.executables())?;
    utils::strip_prefix(&mut executables, &config.paths.target)?;

    let menu = menu::Menu::from(&executables);

    let selection = menu.display();
    mappings.insert("executable", &executables[selection]);

    active_profile
        .command(&config.paths.target, &mappings)
        .spawn()?;

    Ok(())
}
