use std::iter::FromIterator;

use serde::de::{Deserialize, Deserializer};

#[allow(dead_code)]
pub fn string_comma_list<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromIterator<String>,
{
    let s = <&str>::deserialize(deserializer)?;

    Ok(s.split(',').map(|s| s.to_owned()).collect())
}
