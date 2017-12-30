use regex::Regex;

use deserialisers;

use std::ffi::OsStr;

#[derive(Deserialize, Debug)]
pub struct Filters {
    #[serde(deserialize_with = "deserialisers::regex_array")]
    #[serde(default)]
    pub includes: Vec<Regex>,

    #[serde(deserialize_with = "deserialisers::regex_array")]
    #[serde(default)]
    pub excludes: Vec<Regex>,
}

impl Filters {
    pub fn filter<'a, T>(&self, items: &'a [T]) -> Vec<&'a T>
    where
        T: AsRef<OsStr>,
    {
        items
            .iter()
            .filter(|i| {
                let i = i.as_ref().to_str().unwrap();
                self.excludes.iter().all(|excl| !excl.is_match(i))
                    && self.includes.iter().any(|incl| incl.is_match(i))
            })
            .collect()
    }
}
