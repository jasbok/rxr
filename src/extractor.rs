use std::path::PathBuf;
use std::error::Error;

use command::Command;
use mappings::Mappings;

#[derive(Serialize, Deserialize, Debug)]
pub struct Extractor {
    extensions: Vec<String>,
    command: Command,
}

impl Extractor {
    pub fn apply_mappings(&mut self, mappings: &Mappings) {
        self.command.apply_mappings(mappings);
    }

    pub fn can_extract(&self, path: &PathBuf) -> bool {
        self.extensions.contains(&String::from(
            path.as_path().extension().unwrap().to_str().unwrap(),
        ))
    }

    pub fn extract(&self) -> Result<(), Box<Error>> {
        self.command.execute()?;
        Ok(())
    }
}
