extern crate clap;
extern crate serde_json;

extern crate difference;

use std::collections::HashMap;

use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use extractor::Extractor;
use profile::Profile;

mod environment;
use self::environment::Environment;

#[derive(Deserialize, Debug)]
pub struct ConfigurationSource {
    pub archives: Option<Vec<String>>,
    pub config: Option<String>,
    pub data_dir: Option<String>,
    pub temp_dir: Option<String>,
    pub target_dir: Option<String>,
    pub extractor: Option<String>,
    pub profile: Option<String>,
    pub extractors: Option<HashMap<String, Extractor>>,
    pub profiles: Option<HashMap<String, Profile>>,
}

#[derive(Deserialize, Debug)]
pub struct Configuration {
    pub archives: Vec<PathBuf>,
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
        let conf_environment = ConfigurationSource::from(Environment::current())
            .merge(ConfigurationSource::compiled());
        let conf_command_line = ConfigurationSource::command_line(args);
        let conf_filesystem: ConfigurationSource;

        {
            let config_file = conf_command_line
                .config
                .as_ref()
                .or_else(|| conf_environment.config.as_ref());

            if config_file.is_some() {
                let config_file = PathBuf::from(Environment::expand(config_file.unwrap()));

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
                if extract.can_extract(&self.archives[0]) {
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
            self.profiles
                .get(self.profile.as_ref().unwrap())
                .or_else(|| self.profiles.get("fallback"))
        } else {
            None
        }
    }
}

impl ConfigurationSource {
    pub fn compiled() -> ConfigurationSource {
        ConfigurationSource {
            archives: None,
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
            archives: Some(
                args.values_of("archives")
                    .unwrap()
                    .map(String::from)
                    .collect(),
            ),
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

    pub fn filesystem(path: &PathBuf) -> Result<ConfigurationSource, Box<Error>> {
        let mut json = String::new();
        File::open(&path)?.read_to_string(&mut json)?;
        Ok(serde_json::from_str(&json)?)
    }

    pub fn merge(self, other: ConfigurationSource) -> ConfigurationSource {
        ConfigurationSource {
            archives: self.archives.or(other.archives),
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

        let archives: Vec<PathBuf> = self.archives.unwrap().iter().map(PathBuf::from).collect();

        let mut temp_dir = self.temp_dir
            .as_ref()
            .map(Environment::expand)
            .map(PathBuf::from);

        let mut target_dir = self.target_dir
            .as_ref()
            .map(Environment::expand)
            .map(PathBuf::from);

        if temp_dir.is_none() {
            temp_dir = Some(PathBuf::from(
                target_dir.as_ref().unwrap().as_path().parent().unwrap(),
            ));
        } else {
            let targets: Vec<&str> = archives
                .iter()
                .map(|target| target.as_path().file_stem().unwrap().to_str().unwrap())
                .collect();

            let mut target = String::from(targets[0]);

            if targets.len() > 1 {
                for archive in &targets {
                    let change_set = difference::Changeset::new(&target, archive, "");
                    target.clear();
                    for diff in change_set.diffs {
                        match diff {
                            difference::Difference::Same(diff) => {
                                target.push_str(&diff);
                            }
                            difference::Difference::Rem(diff) => {
                                target.push_str(&diff);
                            }
                            difference::Difference::Add(diff) => {
                                target.push_str("_");
                                target.push_str(&diff);
                            }
                        }
                    }
                }
            }

            let mut path = temp_dir.as_ref().unwrap().clone();
            path.push(&target);

            println!("=====> Final Target Path: {:#?}", path);

            target_dir = Some(path);
        }

        Ok(Configuration {
            archives: archives,

            config: self.config
                .as_ref()
                .map(Environment::expand)
                .map(PathBuf::from)
                .unwrap(),

            data_dir: self.data_dir
                .as_ref()
                .map(Environment::expand)
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

impl From<Environment> for ConfigurationSource {
    fn from(environment: Environment) -> ConfigurationSource {
        ConfigurationSource {
            archives: None,

            config: environment.get_config().map(String::clone),

            data_dir: environment.get_data_dir().map(String::clone),

            temp_dir: environment.get_temp_dir().map(String::clone),

            target_dir: environment.get_target_dir().map(String::clone),

            extractor: environment.get_extractor().map(String::clone),

            profile: environment.get_profile().map(String::clone),

            extractors: None,

            profiles: None,
        }
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