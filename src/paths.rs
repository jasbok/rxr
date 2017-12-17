use std::path::PathBuf;

use mappings::Mappings;

#[derive(Serialize, Deserialize, Debug)]
pub struct Paths {
    #[serde(default = "Paths::tmp_default")] pub tmp: PathBuf,
    #[serde(default = "Paths::target_default")] pub target: PathBuf,
    #[serde(default = "Paths::stdout_default")] pub stdout: PathBuf,
    #[serde(default = "Paths::stderr_default")] pub stderr: PathBuf,
}

impl Paths {
    pub fn new() -> Paths {
        Paths {
            tmp: Paths::tmp_default(),
            target: Paths::target_default(),
            stdout: Paths::stdout_default(),
            stderr: Paths::stderr_default(),
        }
    }

    pub fn tmp_default() -> PathBuf {
        PathBuf::from("/tmp/rxr")
    }

    pub fn target_default() -> PathBuf {
        PathBuf::from("{tmp}/{archive.name}")
    }

    pub fn stdout_default() -> PathBuf {
        PathBuf::from("{target}/stdout")
    }

    pub fn stderr_default() -> PathBuf {
        PathBuf::from("{target}/stderr")
    }

    pub fn apply_mappings(&mut self, mappings: &mut Mappings) {
        let tmp = mappings.replace_all(&self.tmp.to_str().unwrap());
        mappings.insert("tmp", &tmp);
        self.tmp = PathBuf::from(&tmp);

        let target = mappings.replace_all(&self.target.to_str().unwrap());
        mappings.insert("target", &target);
        self.target = PathBuf::from(&target);

        let stdout = mappings.replace_all(&self.stdout.to_str().unwrap());
        mappings.insert("stdout", &stdout);
        self.stdout = PathBuf::from(&stdout);

        let stderr = mappings.replace_all(&self.stderr.to_str().unwrap());
        mappings.insert("stderr", &stderr);
        self.stderr = PathBuf::from(&stderr);
    }
}
