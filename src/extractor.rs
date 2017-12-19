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
    pub fn can_extract(&self, path: &PathBuf) -> bool {
        self.extensions.contains(&String::from(
            path.as_path().extension().unwrap().to_str().unwrap(),
        ))
    }

    pub fn extract(&self, archive: &PathBuf, target: &PathBuf) -> Result<(), Box<Error>> {
        let mut mappings = Mappings::new();
        mappings.insert("archive", archive);
        mappings.insert("target", target);

        let mut command = self.command.clone();
        command.apply_mappings(&mappings);
        command.execute()?;

        Ok(())
    }
}
