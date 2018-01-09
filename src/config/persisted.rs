use super::source::Source;

extern crate serde_json;

use extractor::Extractor;
use profile::Profile;

use std::collections::HashMap;
use std::error::Error;
use std::io::prelude::*;
use std::path::Path;

#[derive(Deserialize, Debug, Default)]
pub struct Persisted {
    pub extractors: Option<HashMap<String, Extractor>>,
    pub profiles: Option<HashMap<String, Profile>>,
}

impl Persisted {
    pub fn read<T>(path: &T) -> Result<Persisted, Box<Error>>
    where
        T: AsRef<Path>,
    {
        let mut json = String::new();
        ::fs::File::open(&path)?.read_to_string(&mut json)?;
        Ok(serde_json::from_str(&json)?)
    }
}

impl From<Persisted> for Source {
    fn from(persisted: Persisted) -> Source {
        Source {
            extractors: persisted.extractors,
            profiles: persisted.profiles,
            ..Default::default()
        }
    }
}
