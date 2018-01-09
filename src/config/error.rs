use std::fmt;
use std::error::Error;

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
