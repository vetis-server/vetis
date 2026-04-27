use std::time::Duration;

use serde::{Deserialize, Deserializer};

pub(crate) fn deserialize_duration<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    parse_duration::parse(&s).map_err(serde::de::Error::custom)
}
