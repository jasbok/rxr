extern crate serde_json;

extern crate regex;
use regex::Regex;
use regex::RegexBuilder;

use std::collections::HashMap;
use std::path::Path;
use std::process::Command;

use mappings::Mappings;

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    pub cmd: String,
    pub args: Vec<String>,
    evars: Option<HashMap<String, String>>,
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

    pub fn command(&self, working_dir: &Path, mappings: &Mappings) -> Command {
        let mut cmd = Command::new(&self.cmd);

        let mut args = self.args.clone();
        let mut evars = self.evars.clone().unwrap_or(HashMap::new());

        for arg in &mut args {
            *arg = mappings.replace_all(arg);
        }
        for evar in evars.values_mut() {
            *evar = mappings.replace_all(evar);
        }

        cmd.current_dir(working_dir).args(args).envs(evars);

        cmd
    }
}
