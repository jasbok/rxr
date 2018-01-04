extern crate regex;
use regex::Regex;

use std::collections::HashMap;
use std::env;

pub struct Environment(HashMap<String, String>);

impl Environment {
    pub fn new() -> Environment {
        Environment { 0: HashMap::new() }
    }

    pub fn current() -> Environment {
        Environment {
            0: env::vars()
                .map(|(k, v)| (k, Environment::expand(&v)))
                .collect(),
        }
    }

    pub fn variable(&self, key: &str) -> Option<&String> {
        self.0.get(key)
    }

    pub fn expand<T>(target: &T) -> String
    where
        T: AsRef<str>,
    {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\$\{?([_a-zA-Z0-9]+)\}?").unwrap();
        }

        RE.captures_iter(target.as_ref())
            .filter_map(|rmatch| {
                if let Ok(val) = env::var(&rmatch[1]) {
                    return Some((rmatch, val));
                }
                None
            })
            .fold(String::from(target.as_ref()), |e, (rmatch, val)| {
                e.replace(&rmatch[0], &val)
            })
    }

    pub fn get_config(&self) -> Option<&String> {
        self.variable("RXR_CONFIG")
    }

    pub fn get_data_dir(&self) -> Option<&String> {
        self.0.get("RXR_DATA_DIR").or(self.0.get("XDG_DATA_HOME"))
    }

    pub fn get_temp_dir(&self) -> Option<&String> {
        self.0.get("RXR_TEMP_DIR")
    }

    pub fn get_target_dir(&self) -> Option<&String> {
        self.0.get("RXR_TARGET_DIR")
    }

    pub fn get_extractor(&self) -> Option<&String> {
        self.0.get("RXR_EXTRACTOR_DIR")
    }

    pub fn get_profile(&self) -> Option<&String> {
        self.0.get("RXR_TEMP_DIR")
    }
}
