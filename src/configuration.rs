extern crate clap;
extern crate serde_json;

use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use extractor::Extractor;
use profile::Profile;
use mappings::Mappings;

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    pub archive: Option<PathBuf>,
    pub config: Option<PathBuf>,
    pub config_dir: Option<PathBuf>,
    pub data_dir: Option<PathBuf>,
    pub temp_dir: Option<PathBuf>,
    pub target_dir: Option<PathBuf>,
    pub extractor: Option<String>,
    pub profile: Option<String>,
    pub extractors: Option<HashMap<String, Extractor>>,
    pub profiles: Option<HashMap<String, Profile>>,
    pub mappings: Option<Mappings>,
}

impl Configuration {
    pub fn command_line(args: clap::ArgMatches) -> Configuration {
        Configuration {
            archive: args.value_of("archive").map(PathBuf::from),
            config: args.value_of("config").map(PathBuf::from),
            config_dir: args.value_of("config_dir").map(PathBuf::from),
            data_dir: args.value_of("data_dir").map(PathBuf::from),
            temp_dir: args.value_of("temp_dir").map(PathBuf::from),
            target_dir: args.value_of("target_dir").map(PathBuf::from),
            extractor: args.value_of("extractor").map(String::from),
            profile: args.value_of("profile").map(String::from),
            extractors: None,
            profiles: None,
            mappings: None,
        }
    }

    pub fn environment() -> Configuration {
        Configuration {
            archive: None,

            config: Configuration::get_evar(&vec!["RXR_CONFIG"]).map(PathBuf::from),

            config_dir: Configuration::get_evar(&vec!["RXR_CONFIG_DIR", "XDG_CONFIG_HOME"])
                .map(PathBuf::from),

            data_dir: Configuration::get_evar(&vec!["RXR_DATA_DIR", "XDG_DATA_HOME"])
                .map(PathBuf::from),

            temp_dir: Configuration::get_evar(&vec!["RXR_TEMP_DIR"]).map(PathBuf::from),

            target_dir: Configuration::get_evar(&vec!["RXR_TARGET_DIR"]).map(PathBuf::from),

            extractor: Configuration::get_evar(&vec!["RXR_EXTRACTOR"]),

            profile: Configuration::get_evar(&vec!["RXR_PROFILE"]),

            extractors: None,

            profiles: None,

            mappings: None,
        }
    }

    pub fn filesystem(path: &PathBuf) -> Result<Configuration, Box<Error>> {
        let mut json = String::new();
        File::open(&path)?.read_to_string(&mut json)?;
        Ok(serde_json::from_str(&json)?)
    }

    pub fn get_evar(keys: &[&str]) -> Option<String> {
        for key in keys {
            match env::var(key) {
                Ok(val) => return Some(val),
                Err(_) => continue,
            }
        }

        None
    }

    pub fn merge(self, other: Configuration) -> Configuration {
        Configuration {
            archive: self.archive.or(other.archive),
            config: self.config.or(other.config),
            config_dir: self.config_dir.or(other.config_dir),
            data_dir: self.data_dir.or(other.data_dir),
            temp_dir: self.temp_dir.or(other.temp_dir),
            target_dir: self.target_dir.or(other.target_dir),
            extractor: self.extractor.or(other.extractor),
            profile: self.profile.or(other.profile),
            extractors: self.extractors.or(other.extractors),
            profiles: self.profiles.or(other.profiles),
            mappings: self.mappings.or(other.mappings),
        }
    }

    pub fn validate(self) -> Result<Configuration, Box<Error>> {
        if self.extractors.is_none() {
            return Err(Box::new(ConfigError::NoExtractors));
        }
        if self.profiles.is_none() {
            return Err(Box::new(ConfigError::NoProfiles));
        }

        if self.temp_dir.is_none() && self.target_dir.is_none() {
            return Err(Box::new(ConfigError::NoTemp));
        }

        Ok(self)
    }

    pub fn set_defaults(&mut self) -> () {
        if self.config_dir.is_none() {
            self.config_dir = Some(PathBuf::from(
                self.config.as_ref().unwrap().as_path().parent().unwrap(),
            ));
        } else {
            let mut path = self.config_dir.as_ref().unwrap().clone();
            path.push("rxr.json");

            self.target_dir = Some(path);
        }

        if self.temp_dir.is_none() {
            self.temp_dir = Some(PathBuf::from(
                self.target_dir
                    .as_ref()
                    .unwrap()
                    .as_path()
                    .parent()
                    .unwrap(),
            ));
        } else {
            let mut path = self.temp_dir.as_ref().unwrap().clone();
            path.push(
                self.archive
                    .as_ref()
                    .unwrap()
                    .as_path()
                    .file_name()
                    .unwrap(),
            );

            self.target_dir = Some(path);
        }
    }

    pub fn new(args: clap::ArgMatches) -> Result<Configuration, Box<Error>> {
        let conf_environment = Configuration::environment();
        let conf_command_line = Configuration::command_line(args);
        let conf_filesystem: Configuration;

        {
            let config_file = conf_command_line
                .config
                .as_ref()
                .or(conf_environment.config.as_ref())
                .or_else(|| {
                    let config_dir = conf_command_line
                        .config_dir
                        .as_ref()
                        .or(conf_environment.config_dir.as_ref());
                    config_dir
                });

            if config_file.is_none() {
                return Err(Box::new(ConfigError::NoConfig));
            }

            conf_filesystem = Configuration::filesystem(&config_file.unwrap())?;
        }

        let conf = conf_command_line
            .merge(conf_filesystem)
            .merge(conf_environment)
            .validate();

        conf
    }

    pub fn get_extractor(&self) -> Option<&Extractor> {
        let mut extractor = None;

        if self.extractor.is_some() {
            extractor = self.extractors
                .as_ref()
                .unwrap()
                .get(self.extractor.as_ref().unwrap());
        } else {
            for extract in self.extractors.as_ref().unwrap().values() {
                if extract.can_extract(self.archive.as_ref().unwrap()) {
                    extractor = Some(extract);
                }
            }
        }

        extractor = extractor.or(self.extractors.as_ref().unwrap().get("fallback"));

        extractor
    }

    pub fn get_profile(&self) -> Option<&Profile> {
        let mut profile = None;

        if self.profile.is_some() {
            profile = self.profiles
                .as_ref()
                .unwrap()
                .get(self.profile.as_ref().unwrap());
        }

        profile.or(self.profiles.as_ref().unwrap().get("fallback"))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ConfigError {
    NoConfig,
    NoExtractors,
    NoProfiles,
    NoTemp,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfigError::NoConfig => write!(f, "no configuration file was provided"),
            ConfigError::NoExtractors => {
                write!(f, "no extractors where provided in the config file")
            }
            ConfigError::NoProfiles => write!(f, "no profiles where provided in the config file"),
            ConfigError::NoTemp => write!(
                f,
                "no temp or target directory was provided in the config file"
            ),
        }
    }
}

impl Error for ConfigError {
    fn description(&self) -> &str {
        match *self {
            ConfigError::NoConfig => "no configuration file was provided",
            ConfigError::NoExtractors => "no extractors where provided in the config file",
            ConfigError::NoProfiles => "no profiles where provided in the config file",
            ConfigError::NoTemp => "no temp or target directory was provided in the config file",
        }
    }
}
