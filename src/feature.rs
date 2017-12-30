extern crate regex;
use regex::Regex;

use deserialisers;


#[derive(Deserialize, Debug)]
pub struct Feature {
    #[serde(deserialize_with = "deserialisers::regex")] pattern: Regex,

    #[serde(default = "Feature::default_weight")] weight: usize,
}

impl Feature {
    pub fn default_weight() -> usize {
        1
    }

    pub fn score(&self, item: &str) -> usize {
        if self.pattern.is_match(item) {
            return self.weight;
        }

        0
    }

    pub fn score_all(&self, items: &[&str]) -> usize {
        items.iter().fold(0, |sum, &item| sum + self.score(item))
    }
}
