extern crate serde_json;

extern crate regex;
use regex::Regex;
use regex::RegexBuilder;

use std::error::Error;
use std::fmt;
use std::path::PathBuf;

use command::Command;
use feature::Feature;
use mappings::Mappings;

#[derive(Deserialize, Debug)]
pub struct Profile {
    pub command: Command,

    #[serde(default = "Vec::new")]
    pub executables: Vec<String>,

    #[serde(default = "Vec::new")]
    pub features: Vec<Feature>,
}

impl Profile {
    pub fn executables_regex(&self) -> Result<Regex, Box<Error>> {
        let regex_str = self.executables.as_slice().join("|");

        match RegexBuilder::new(&regex_str).case_insensitive(true).build() {
            Ok(reg) => Ok(reg),
            Err(_) => Err(Box::new(ProfileError::CouldNotParseExecutableRegex)),
        }
    }

    pub fn run(&self, executable: &PathBuf, target: &PathBuf) -> Result<(), Box<Error>> {
        let mut mappings = Mappings::new();
        mappings.insert("executable", executable);
        mappings.insert("target", target);

        let mut command = self.command.clone();
        command.apply_mappings(&mappings);
        command.execute()?;

        Ok(())
    }

    pub fn feature_score(&self, items: &[&str]) -> usize {
        self.features.iter().fold(0, |sum, ref feature| {
            sum + feature.score_all(items)
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ProfileError {
    CouldNotParseExecutableRegex,
}

impl fmt::Display for ProfileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ProfileError::CouldNotParseExecutableRegex => {
                write!(f, "could not parse one of the executable regexes")
            }
        }
    }
}

impl Error for ProfileError {
    fn description(&self) -> &str {
        match *self {
            ProfileError::CouldNotParseExecutableRegex => {
                "could not parse one of the executable regexes"
            }
        }
    }
}