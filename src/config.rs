extern crate serde_json;

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::string::String;

use extractor::Extractor;
use mappings::Mappings;
use profile::Profile;
use paths::Paths;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub profiles: HashMap<String, Profile>,
    #[serde(default = "Paths::new")] pub paths: Paths,
    pub extractors: HashMap<String, Extractor>,
}

impl Config {
    pub fn open(path: &PathBuf) -> Result<Config, Box<Error>> {
        let mut json = String::new();
        File::open(&path)?.read_to_string(&mut json)?;
        Ok(serde_json::from_str(&json)?)
    }

    pub fn apply_mappings(&mut self, mappings: &mut Mappings) -> () {
        self.paths.apply_mappings(mappings);

        for extractor in self.extractors.values_mut() {
            extractor.apply_mappings(mappings);
        }
    }

    pub fn get_extractor(&self, archive: &PathBuf) -> Option<&Extractor> {
        let mut extractor = None;

        for extract in self.extractors.values() {
            if extract.can_extract(&archive) {
                extractor = Some(extract);
            }
        }

        extractor
    }
}
