use std::collections::HashMap;
use std::error::Error;
use std::process;

use mappings::Mappings;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Command {
    pub cmd: String,
    pub args: Vec<String>,
    #[serde(default = "Command::evars_default")]
    pub evars: HashMap<String, String>,
    #[serde(default = "Command::wd_default")]
    pub wd: String,
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

        println!("Executing command => {:#?}: {:?}", self.wd, cmd);

        let output = cmd.output();

        match output {
            Ok(output) => {
                if output.status.success() {
                    println!("Executed command successfully.");
                } else {
                    println!("Command failed.");
                }
                println!("[stdout] {:#?}", String::from_utf8_lossy(&output.stdout));
                println!("[stderr] {:#?}", String::from_utf8_lossy(&output.stderr));
            }
            Err(err) => println!("Failed to run command: {:#?}", err),
        }

        Ok(())
    }
}
