extern crate rxr;
use rxr::filters::Filters;

extern crate regex;
use regex::Regex;

use std::path::PathBuf;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_strs() {
        let filters = Filters {
            includes: vec![Regex::new(".*\\.(exe|bat)$").unwrap()],
            excludes: vec![
                Regex::new(".*setup.*").unwrap(),
                Regex::new("docs.bat").unwrap(),
                Regex::new("docs.com").unwrap(),
            ],
        };

        let items = vec![
            "run.exe",
            "run.bat",
            "run.com",
            "setup.exe",
            "setup.bat",
            "setup.com",
            "docs.exe",
            "docs.bat",
            "docs.com",
        ];

        let filtered = filters.filter(&items);
        let expected: Vec<&&str> = vec![&"run.exe", &"run.bat", &"docs.exe"];

        assert_eq!(filtered, expected);
    }

    #[test]
    fn filter_paths() {
        let filters = Filters {
            includes: vec![Regex::new(".*\\.(exe|bat)$").unwrap()],
            excludes: vec![
                Regex::new(".*setup.*").unwrap(),
                Regex::new("docs.bat").unwrap(),
                Regex::new("docs.com").unwrap(),
            ],
        };

        let items = vec![
            PathBuf::from("run.exe"),
            PathBuf::from("run.bat"),
            PathBuf::from("run.com"),
            PathBuf::from("setup.exe"),
            PathBuf::from("setup.bat"),
            PathBuf::from("setup.com"),
            PathBuf::from("docs.exe"),
            PathBuf::from("docs.bat"),
            PathBuf::from("docs.com"),
        ];

        let filtered = filters.filter(&items);
        let expected = vec![
            PathBuf::from("run.exe"),
            PathBuf::from("run.bat"),
            PathBuf::from("docs.exe"),
        ];
        let expected_ref: Vec<_> = expected.iter().collect();

        assert_eq!(filtered, expected_ref);
    }
}
