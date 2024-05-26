use std::str::FromStr;

use anyhow::Result;
use csv::ReaderBuilder;
use serde::{de::DeserializeOwned, Deserialize, Deserializer};

pub fn from_sequence<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr + Default,
{
    let sequence = String::deserialize(deserializer)?;
    match sequence.len() {
        0 => Ok(vec![]),
        _ => Ok(sequence
            .split(",")
            .map(|s| s.parse::<T>().unwrap_or_default())
            .collect()),
    }
}

pub fn from_str<'de, T>(data: &str) -> Result<Vec<T>, csv::Error>
where
    T: DeserializeOwned,
{
    let mut reader = ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(data.as_bytes());

    reader.deserialize::<T>().collect()
}
