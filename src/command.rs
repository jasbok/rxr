use std::collections::HashMap;
use std::error::Error;
use std::process;

use mappings::Mappings;

#[derive(Serialize, Deserialize, Debug)]
pub struct Command {
    pub cmd: String,
    pub args: Vec<String>,
    #[serde(default = "Command::evars_default")] pub evars: HashMap<String, String>,
    #[serde(default = "Command::wd_default")] pub wd: String,
}

impl Command {
    pub fn evars_default() -> HashMap<String, String> {
        HashMap::new()
    }

    pub fn wd_default() -> String {
        String::from("{target}")
    }

    pub fn apply_mappings(&mut self, mappings: &Mappings) {
        mappings.replace(&mut self.wd);
        mappings.replace_vec(&mut self.args);
        mappings.replace_map(&mut self.evars);
    }

    pub fn execute(&self) -> Result<(), Box<Error>> {
        let mut cmd = process::Command::new(&self.cmd);
        cmd.args(&self.args).envs(&self.evars).current_dir(&self.wd);

        match cmd.output() {
            Ok(_) => {}
            Err(err) => println!("Failed to run command: {:#?}", err),
        }

        Ok(())
    }
}
