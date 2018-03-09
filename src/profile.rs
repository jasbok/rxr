extern crate serde_json;

extern crate regex;
use regex::Regex;

use std::error::Error;
use std::path::Path;
use std::path::PathBuf;
use std::ffi::OsStr;

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
    pub fn run<T>(&self, executable: &T, target: &T) -> Result<(), Box<Error>>
    where
        T: AsRef<OsStr>,
    {
        let executable = executable.as_ref().to_string_lossy().into_owned();
        let target = target.as_ref().to_string_lossy().into_owned();

        let mut mappings = Mappings::new();

        mappings.insert("executable", &executable);
        mappings.insert(
            "executable_dir",
            &PathBuf::from(&executable)
                .parent()
                .unwrap_or_else(|| Path::new("."))
                .to_string_lossy(),
        );
        mappings.insert("target", &target);

        let mut command = self.command.clone();
        command.apply_mappings(&mappings);
        command.execute()?;

        Ok(())
    }

    pub fn feature_score(&self, items: &[&str]) -> usize {
        self.features
            .iter()
            .fold(0, |sum, feature| sum + feature.score_all(items))
    }
}
