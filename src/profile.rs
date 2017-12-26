extern crate serde_json;

extern crate regex;
use regex::Regex;

use std::error::Error;
use std::path::PathBuf;

use command::Command;
use feature::Feature;
use mappings::Mappings;

use deserialisers;

#[derive(Deserialize, Debug)]
pub struct Profile {
    pub command: Command,

    #[serde(default = "Vec::new")]
    #[serde(deserialize_with = "deserialisers::regex_array")]
    pub executables: Vec<Regex>,

    #[serde(default = "Vec::new")]
    pub features: Vec<Feature>,
}

impl Profile {
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
        self.features.iter().fold(0, |sum, feature| {
            sum + feature.score_all(items)
        })
    }
}