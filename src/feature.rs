extern crate regex;
use regex::Regex;
use regex::RegexBuilder;

use std::error::Error;
use std::fmt;


use serde::de::{self, Deserialize, Deserializer, Visitor, MapAccess};

#[derive(Debug)]
pub struct Feature {
    pattern: Regex,
    weight: usize,
}

impl Feature {
    pub fn new(pattern: &str, weight: usize) -> Result<Feature, Box<Error>> {
        Ok(Feature {
            pattern: RegexBuilder::new(pattern).case_insensitive(true).build()?,
            weight: weight,
        })
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

impl<'de> Deserialize<'de> for Feature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Pattern,
            Weight,
        };

        struct FeatureVisitor;

        impl<'de> Visitor<'de> for FeatureVisitor {
            type Value = Feature;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Feature")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Feature, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut pattern = None;
                let mut weight = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Pattern => {
                            if pattern.is_some() {
                                return Err(de::Error::duplicate_field("pattern"));
                            }
                            pattern = Some(map.next_value()?);
                        }
                        Field::Weight => {
                            if weight.is_some() {
                                return Err(de::Error::duplicate_field("weight"));
                            }
                            weight = Some(map.next_value()?);
                        }
                    }
                }
                let pattern: String = pattern.ok_or_else(|| de::Error::missing_field("pattern"))?;
                let weight = weight.unwrap_or(1);

                Feature::new(&pattern, weight).or_else(|err| {
                    Err(de::Error::custom(err.description()))
                })
            }
        }

        const FIELDS: &'static [&'static str] = &["pattern", "weight"];
        deserializer.deserialize_struct("Feature", FIELDS, FeatureVisitor)
    }
}