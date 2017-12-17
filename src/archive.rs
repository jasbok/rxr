use std::error::Error;
use std::ffi::OsString;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::Output;

pub struct Archive {
    path: PathBuf,
}

impl Archive {
    pub fn from(path: &str) -> Archive {
        let mut archive = Archive {
            path: PathBuf::new(),
        };

        archive.path.push(Path::new(path));

        archive
    }

    pub fn name(&self) -> String {
        OsString::from(self.path.file_name().unwrap())
            .into_string()
            .unwrap()
    }

    pub fn extension(&self) -> String {
        OsString::from(self.path.extension().unwrap())
            .into_string()
            .unwrap()
    }

    pub fn extract(&self, target: &Path) -> Result<(Output), Box<Error>> {
        let output = Command::new("unzip")
            .args(&[self.path.to_str().unwrap(), "-d", target.to_str().unwrap()])
            .output()?;

        Ok(output)
    }
}
