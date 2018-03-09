use std::path::Path;
use std::error::Error;

use command::Command;
use mappings::Mappings;

#[derive(Serialize, Deserialize, Debug)]
pub struct Extractor {
    extensions: Vec<String>,
    command: Command,
}

impl Extractor {
    pub fn can_extract<T>(&self, path: &T) -> bool
    where
        T: AsRef<str>,
    {
        let extension = Path::new(path.as_ref())
            .extension()
            .unwrap_or_default()
            .to_string_lossy();

        self.extensions.contains(&extension.into_owned())
    }

    pub fn extract<T>(&self, archive: &T, target: &T) -> Result<(), Box<Error>>
    where
        T: AsRef<str>,
    {
        let mut mappings = Mappings::new();
        mappings.insert("archive", archive);
        mappings.insert("target", target);

        println!("Mappings: {:#?}", mappings);

        let mut command = self.command.clone();
        command.apply_mappings(&mappings);

        println!("Command: {:#?}", command);

        command.execute()?;

        Ok(())
    }
}
