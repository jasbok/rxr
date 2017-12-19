use std::error::Error;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use regex::Regex;

pub fn recursive_find(dir: &Path, regexes: &Vec<Regex>) -> Result<Vec<PathBuf>, Box<Error>> {
    let mut results: Vec<PathBuf> = Vec::new();

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let mut dir_paths = recursive_find(&path, &regexes)?;
                results.append(&mut dir_paths);
            } else {
                for re in regexes {
                    if re.is_match(path.to_str().unwrap()) {
                        results.push(path);
                        break;
                    }
                }
            }
        }
    }

    Ok(results)
}

pub fn strip_prefix(paths: &mut Vec<PathBuf>, prefix: &PathBuf) -> Result<(), Box<Error>> {
    for path in paths {
        *path = PathBuf::from(path.as_path().strip_prefix(&prefix)?.to_str().unwrap());
    }

    Ok(())
}
