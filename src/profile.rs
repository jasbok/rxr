extern crate serde_json;

extern crate regex;
use regex::Regex;
use regex::RegexBuilder;

use std::error::Error;
use std::path::PathBuf;

use command::Command;
use mappings::Mappings;

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    pub command: Command,
    // pub cmd: String,
    // pub args: Vec<String>,
    // evars: Option<HashMap<String, String>>,
    executables: Option<Vec<String>>,
    features: Option<Vec<String>>,
}

impl Profile {
    pub fn executables(&self) -> Vec<Regex> {
        let mut regexes: Vec<Regex> = Vec::new();

        if let Some(ref execs) = self.executables {
            for exec in execs {
                regexes.push(
                    RegexBuilder::new(&exec)
                        .case_insensitive(true)
                        .build()
                        .unwrap(),
                );
            }
        }

        regexes
    }

    pub fn features(&self) -> Vec<Regex> {
        let mut regexes: Vec<Regex> = Vec::new();

        if let Some(ref execs) = self.executables {
            for exec in execs {
                regexes.push(
                    RegexBuilder::new(&exec)
                        .case_insensitive(true)
                        .build()
                        .unwrap(),
                );
            }
        }

        regexes
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
}
