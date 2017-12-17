extern crate serde_json;

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::string::String;

use mappings::Mappings;
use profile::Profile;
use paths::Paths;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub profiles: HashMap<String, Profile>,
    #[serde(default = "Paths::new")] pub paths: Paths,
}

impl Config {
    pub fn open(path: &str) -> Result<Config, Box<Error>> {
        let mut json = String::new();
        File::open(&path)?.read_to_string(&mut json)?;
        Ok(serde_json::from_str(&json)?)
    }

    pub fn apply_mappings(&mut self, mappings: &mut Mappings) -> () {
        self.paths.apply_mappings(mappings);
    }
}
