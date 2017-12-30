use std::collections::HashMap;
use std::error::Error;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use regex::Regex;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct DosboxConfig {
    pub autoexec: Vec<String>,
    pub settings: HashMap<String, HashMap<String, String>>,
}

impl DosboxConfig {
    pub fn new() -> DosboxConfig {
        DosboxConfig {
            autoexec: Vec::new(),
            settings: HashMap::new(),
        }
    }

    pub fn read(path: &PathBuf) -> Result<DosboxConfig, Box<Error>> {
        let lines: Vec<String> = BufReader::new(File::open(&path)?)
            .lines()
            .filter_map(|line| line.ok())
            .collect();

        DosboxConfig::parse(lines.as_slice())
    }

    pub fn parse<T>(lines: &[T]) -> Result<DosboxConfig, Box<Error>>
    where
        T: AsRef<str>,
    {
        lazy_static! {
            static ref SECTION_RE : Regex = Regex::new("\\[(.+)\\]").unwrap();
            static ref COMMENT_RE : Regex = Regex::new("#(.*)").unwrap();
        }

        let filtered = lines
            .iter()
            .map(|line| line.as_ref().trim())
            .filter(|line| !line.is_empty())
            .filter(|line| !COMMENT_RE.is_match(line));

        let mut section: Option<String> = None;
        let mut autoexec: Vec<String> = Vec::new();
        let mut settings: HashMap<String, HashMap<String, String>> = HashMap::new();

        for line in filtered {
            if let Some(caps) = SECTION_RE.captures(line) {
                section = Some(String::from(caps.get(1).unwrap().as_str()));
            } else if let Some(section) = section.as_ref() {
                match section.as_str() {
                    "autoexec" => autoexec.push(DosboxConfig::command_from(line)),
                    section => {
                        if let Some((key, val)) = DosboxConfig::setting_from(line) {
                            settings
                            .entry(String::from(section)) // TODO: Avoid multiple clones.
                            .or_insert(HashMap::new())
                            .insert(key, val);
                        }
                    }
                }
            }
        }

        Ok(DosboxConfig {
            autoexec: autoexec,
            settings: settings,
        })
    }

    fn command_from(line: &str) -> String {
        line.replace("\\", "/")
    }

    fn setting_from(line: &str) -> Option<(String, String)> {
        lazy_static! {
            static ref KEY_VAL_RE : Regex = Regex::new("(.+?)\\s*=\\s*(.+)").unwrap();
        }

        if let Some(caps) = KEY_VAL_RE.captures(&line) {
            return Some((
                String::from(caps.get(1).unwrap().as_str()),
                String::from(caps.get(2).unwrap().as_str()),
            ));
        }

        None
    }

    pub fn merge(&self, config_b: &DosboxConfig) -> DosboxConfig {
        let mut merged = self.clone();

        for line in &config_b.autoexec {
            println!("Adding autoexec line: {}", line);
            merged.autoexec.push(line.clone());
        }

        for (section, settings) in &config_b.settings {
            if let Some(msettings) = merged.settings.get_mut(section) {
                for (key, val) in settings {
                    msettings.insert(key.clone(), val.clone());
                }
                continue;
            }
            merged.settings.insert(section.clone(), settings.clone());
        }

        merged
    }

    pub fn write(&self, path: &PathBuf) -> Result<(), Box<Error>> {
        let mut file = File::create(path)?;

        for (section, settings) in &self.settings {
            file.write(format!("[{}]\n", section).as_bytes())?;

            for (key, val) in settings {
                file.write(format!("{}={}\n", key, val).as_bytes())?;
            }
        }

        file.write("[autoexec]\n".as_bytes())?;
        for line in &self.autoexec {
            file.write(format!("{}\n", line).as_bytes())?;
        }

        file.flush()?;
        Ok(())
    }
}
