extern crate clap;
extern crate serde_json;

extern crate regex;
use regex::Regex;

use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use extractor::Extractor;
use profile::Profile;

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigurationSource {
    pub archive: Option<String>,
    pub config: Option<String>,
    pub data_dir: Option<String>,
    pub temp_dir: Option<String>,
    pub target_dir: Option<String>,
    pub extractor: Option<String>,
    pub profile: Option<String>,
    pub extractors: Option<HashMap<String, Extractor>>,
    pub profiles: Option<HashMap<String, Profile>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    pub archive: PathBuf,
    pub config: PathBuf,
    pub data_dir: PathBuf,
    pub temp_dir: PathBuf,
    pub target_dir: PathBuf,
    pub extractor: Option<String>,
    pub profile: Option<String>,
    pub extractors: HashMap<String, Extractor>,
    pub profiles: HashMap<String, Profile>,
}

impl Configuration {
    pub fn load(args: &clap::ArgMatches) -> Result<Configuration, Box<Error>> {
        let conf_environment =
            ConfigurationSource::environment().merge(ConfigurationSource::compiled());
        let conf_command_line = ConfigurationSource::command_line(args);
        let conf_filesystem: ConfigurationSource;

        {
            let config_file = conf_command_line.config.as_ref().or_else(|| {
                conf_environment.config.as_ref()
            });

            if config_file.is_some() {
                let config_file = PathBuf::from(ConfigurationSource::expand_evars(
                    config_file.unwrap().clone(),
                ));

                println!("config_file: {:#?}", config_file);
                conf_filesystem = ConfigurationSource::filesystem(&config_file)?;
            } else {
                return Err(Box::new(ConfigError::NoConfig));
            }
        }

        conf_command_line
            .merge(conf_filesystem)
            .merge(conf_environment)
            .validate()
    }

    pub fn get_extractor(&self) -> Option<&Extractor> {
        let mut extractor = if self.extractor.is_some() {
            self.extractors.get(self.extractor.as_ref().unwrap())
        } else {
            for extract in self.extractors.values() {
                if extract.can_extract(&self.archive) {
                    return Some(extract);
                }
            }

            None
        };

        extractor = extractor.or_else(|| self.extractors.get("fallback"));

        extractor
    }

    pub fn get_profile(&self) -> Option<&Profile> {
        if self.profile.is_some() {
            self.profiles.get(self.profile.as_ref().unwrap()).or_else(
                || {
                    self.profiles.get("fallback")
                },
            )
        } else {
            None
        }
    }
}

impl ConfigurationSource {
    pub fn compiled() -> ConfigurationSource {
        ConfigurationSource {
            archive: None,
            config: option_env!("RXR_CONFIG").map(String::from),
            data_dir: option_env!("RXR_DATA_DIR").map(String::from),
            temp_dir: option_env!("RXR_TEMP_DIR").map(String::from),
            target_dir: None,
            extractor: None,
            profile: None,
            extractors: None,
            profiles: None,
        }
    }

    pub fn command_line(args: &clap::ArgMatches) -> ConfigurationSource {
        ConfigurationSource {
            archive: args.value_of("archive").map(String::from),
            config: args.value_of("config").map(String::from),
            data_dir: args.value_of("data_dir").map(String::from),
            temp_dir: args.value_of("temp_dir").map(String::from),
            target_dir: args.value_of("target_dir").map(String::from),
            extractor: args.value_of("extractor").map(String::from),
            profile: args.value_of("profile").map(String::from),
            extractors: None,
            profiles: None,
        }
    }

    pub fn environment() -> ConfigurationSource {
        ConfigurationSource {
            archive: None,

            config: ConfigurationSource::get_evar(&["RXR_CONFIG"]),

            data_dir: ConfigurationSource::get_evar(&["RXR_DATA_DIR", "XDG_DATA_HOME"]),

            temp_dir: ConfigurationSource::get_evar(&["RXR_TEMP_DIR"]),

            target_dir: ConfigurationSource::get_evar(&["RXR_TARGET_DIR"]),

            extractor: ConfigurationSource::get_evar(&["RXR_EXTRACTOR"]),

            profile: ConfigurationSource::get_evar(&["RXR_PROFILE"]),

            extractors: None,

            profiles: None,
        }
    }

    pub fn filesystem(path: &PathBuf) -> Result<ConfigurationSource, Box<Error>> {
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

    pub fn expand_evars(s: String) -> String {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\$\{?([_a-zA-Z0-9]+)\}?").unwrap();
        }

        let expanded = RE.captures_iter(&s)
            .map(|m| {
                (
                    m.get(0).unwrap().as_str(),
                    env::var(m.get(1).unwrap().as_str()).unwrap_or(String::new()),
                )
            })
            .fold(s.clone(), |e, (key, val)| e.replace(key, &val));

        if expanded != s {
            return ConfigurationSource::expand_evars(expanded);
        }

        expanded
    }

    pub fn merge(self, other: ConfigurationSource) -> ConfigurationSource {
        ConfigurationSource {
            archive: self.archive.or(other.archive),
            config: self.config.or(other.config),
            data_dir: self.data_dir.or(other.data_dir),
            temp_dir: self.temp_dir.or(other.temp_dir),
            target_dir: self.target_dir.or(other.target_dir),
            extractor: self.extractor.or(other.extractor),
            profile: self.profile.or(other.profile),
            extractors: self.extractors.or(other.extractors),
            profiles: self.profiles.or(other.profiles),
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

        let archive = self.archive.map(PathBuf::from).unwrap();

        let mut temp_dir = self.temp_dir.map(ConfigurationSource::expand_evars).map(
            PathBuf::from,
        );

        let mut target_dir = self.target_dir.map(ConfigurationSource::expand_evars).map(
            PathBuf::from,
        );

        if temp_dir.is_none() {
            temp_dir = Some(PathBuf::from(
                target_dir.as_ref().unwrap().as_path().parent().unwrap(),
            ));
        } else {
            let mut path = temp_dir.as_ref().unwrap().clone();
            path.push(archive.as_path().file_name().unwrap());

            target_dir = Some(path);
        }

        Ok(Configuration {
            archive: archive,

            config: self.config
                .map(ConfigurationSource::expand_evars)
                .map(PathBuf::from)
                .unwrap(),

            data_dir: self.data_dir
                .map(ConfigurationSource::expand_evars)
                .map(PathBuf::from)
                .unwrap(),

            temp_dir: temp_dir.unwrap(),

            target_dir: target_dir.unwrap(),

            extractor: self.extractor,

            profile: self.profile,

            extractors: self.extractors.unwrap(),

            profiles: self.profiles.unwrap(),
        })
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
            ConfigError::NoTemp => {
                write!(
                    f,
                    "no temp or target directory was provided in the config file"
                )
            }
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
