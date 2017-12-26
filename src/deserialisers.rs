use regex::Regex;
use regex::RegexBuilder;

use serde::de;
use serde::Deserialize;
use serde::Deserializer;

use std::error::Error;

pub fn regex<'de, D>(deserialiser: D) -> Result<Regex, D::Error>
where
    D: Deserializer<'de>,
{
    let input = Deserialize::deserialize(deserialiser).map(
        |val: String| val,
    )?;

    RegexBuilder::new(&input)
        .case_insensitive(true)
        .build()
        .or_else(|err| {
            Err(de::Error::custom(format!(
                "Failed deserialising regex: '{}'",
                err.description()
            )))
        })
}

pub fn regex_array<'de, D>(deserialiser: D) -> Result<Vec<Regex>, D::Error>
where
    D: Deserializer<'de>,
{
    let vec = Deserialize::deserialize(deserialiser).map(
        |val: Vec<String>| val,
    )?;

    let mut output: Vec<Regex> = Vec::new();

    for input in &vec {
        output.push(RegexBuilder::new(input)
            .case_insensitive(true)
            .build()
            .or_else(|err| {
                Err(de::Error::custom(format!(
                    "Failed deserialising regex: '{}'",
                    err.description()
                )))
            })?);
    }

    Ok(output)
}