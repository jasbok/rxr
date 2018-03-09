use config::commandline::CommandLine;
use config::compiled::Compiled;
use config::error::ConfigError;
use config::environment::Environment;
use config::persisted::Persisted;
use config::source::Source;

use extractor::Extractor;
use profile::Profile;

use std::collections::HashMap;
use std::error::Error;

#[derive(Deserialize, Debug)]
pub struct Configuration {
    pub archives: Vec<String>,
    pub config: String,
    pub data_dir: String,
    pub temp_dir: String,
    pub target_dir: String,
    pub extractor: Option<String>,
    pub profile: Option<String>,
    pub extractors: HashMap<String, Extractor>,
    pub profiles: HashMap<String, Profile>,
}

impl Configuration {
    pub fn load() -> Result<Configuration, Box<Error>> {
        let args = CommandLine::current();
        let environment = Environment::current();
        let persisted: Persisted;
        let compiled = Compiled::new();

        {
            let persisted_path = args.config
                .as_ref()
                .or_else(|| environment.get_config())
                .or_else(|| compiled.config.as_ref());

            if let Some(path) = persisted_path {
                persisted = Persisted::read(path)?;
            } else {
                return Err(Box::new(ConfigError::NoConfig));
            }
        }

        Source::from(args)
            .merge(Source::from(environment))
            .merge(Source::from(persisted))
            .merge(Source::from(compiled))
            .validate()
    }

    pub fn get_extractor(&self) -> Option<&Extractor> {
        let mut extractor = if self.extractor.is_some() {
            self.extractors.get(self.extractor.as_ref().unwrap())
        } else {
            for extract in self.extractors.values() {
                if extract.can_extract(&self.archives[0]) {
                    return Some(extract);
                }
            }

            None
        };

        extractor = extractor.or_else(|| self.extractors.get("fallback"));

        extractor
    }

    pub fn get_profile(&self) -> Option<&Profile> {
        if self.profile.is_some() {
            self.profiles
                .get(self.profile.as_ref().unwrap())
                .or_else(|| self.profiles.get("fallback"))
        } else {
            None
        }
    }
}
