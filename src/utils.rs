use std::error::Error;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use regex::Regex;

pub fn recursive_find_all(dir: &Path) -> Result<Vec<PathBuf>, Box<Error>> {
    if dir.is_dir() {
        let (dirs, files): (Vec<PathBuf>, Vec<PathBuf>) = fs::read_dir(dir)?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .partition(|p| p.is_dir());

        let mut results: Vec<PathBuf> = files.to_vec();

        for dir in dirs {
            let mut dir_paths = recursive_find_all(&dir)?;
            results.append(&mut dir_paths);
        }

        return Ok(results);
    }

    Ok(Vec::new())
}

pub fn recursive_find(dir: &Path, regexes: &[Regex]) -> Result<Vec<PathBuf>, Box<Error>> {
    if dir.is_dir() {
        let (dirs, files): (Vec<PathBuf>, Vec<PathBuf>) = fs::read_dir(dir)?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .partition(|p| p.is_dir());

        let mut results: Vec<PathBuf> = files
            .iter()
            .filter(|f| {
                regexes.iter().any(
                    |regex| regex.is_match(f.to_str().unwrap()),
                )
            })
            .cloned()
            .collect();

        for dir in dirs {
            let mut dir_paths = recursive_find(&dir, regexes)?;
            results.append(&mut dir_paths);
        }

        return Ok(results);
    }

    Ok(Vec::new())
}

pub fn strip_prefix(paths: &mut Vec<PathBuf>, prefix: &PathBuf) -> Result<(), Box<Error>> {
    for path in paths {
        *path = PathBuf::from(path.as_path().strip_prefix(&prefix)?.to_str().unwrap());
    }

    Ok(())
}