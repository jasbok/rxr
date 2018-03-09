use config::configuration::Configuration;
use config::environment::Environment;
use config::error::ConfigError;

use extractor::Extractor;
use profile::Profile;

use std::collections::HashMap;
use std::error::Error;

use std::path::Path;
use std::path::PathBuf;

extern crate difference;

#[derive(Deserialize, Debug, Default)]
pub struct Source {
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

impl Source {
    pub fn merge(self, other: Source) -> Source {
        Source {
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

        //let archives: Vec<PathBuf> = self.archives.unwrap().iter().map(PathBuf::from).collect();

        let mut temp_dir = self.temp_dir.as_ref().map(Environment::expand);

        let mut target_dir = self.target_dir.as_ref().map(Environment::expand);

        if temp_dir.is_none() {
            temp_dir = Some(
                PathBuf::from(target_dir.as_ref().unwrap())
                    .as_path()
                    .parent()
                    .unwrap()
                    .to_string_lossy()
                    .into_owned(),
            );
        } else {
            let targets: Vec<&str> = self.archives
                .as_ref()
                .unwrap()
                .iter()
                .map(|target| Path::new(target).file_stem().unwrap().to_str().unwrap())
                .collect();

            let mut target = String::from(targets[0]);

            if targets.len() > 1 {
                for archive in &targets {
                    let change_set = difference::Changeset::new(&target, archive, "");
                    target.clear();
                    for diff in change_set.diffs {
                        match diff {
                            difference::Difference::Same(diff)
                            | difference::Difference::Rem(diff) => {
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
            path.push('/');
            path.push_str(&target);

            println!("=====> Final Target Path: {:#?}", path);

            target_dir = Some(path);
        }

        Ok(Configuration {
            archives: self.archives.unwrap_or_default(),

            config: self.config.as_ref().map(Environment::expand).unwrap(),

            data_dir: self.data_dir.as_ref().map(Environment::expand).unwrap(),

            //temp_dir: self.temp_dir.unwrap_or_default(),
            temp_dir: temp_dir.unwrap_or_default(),

            target_dir: target_dir.unwrap_or_default(),

            extractor: self.extractor,

            profile: self.profile,

            extractors: self.extractors.unwrap(),

            profiles: self.profiles.unwrap(),
        })
    }
}
