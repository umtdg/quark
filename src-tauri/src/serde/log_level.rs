use log::LevelFilter;
use serde::{Deserialize, Deserializer, Serializer};

pub fn serialize<S>(level: &LevelFilter, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match level {
        LevelFilter::Off => serializer.serialize_str("off"),
        LevelFilter::Trace => serializer.serialize_str("trace"),
        LevelFilter::Debug => serializer.serialize_str("debug"),
        LevelFilter::Info => serializer.serialize_str("info"),
        LevelFilter::Warn => serializer.serialize_str("warn"),
        LevelFilter::Error => serializer.serialize_str("error"),
    }
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<LevelFilter, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?.to_lowercase();
    match s.as_str() {
        "off" => Ok(LevelFilter::Off),
        "trace" => Ok(LevelFilter::Trace),
        "debug" => Ok(LevelFilter::Debug),
        "info" => Ok(LevelFilter::Info),
        "warn" => Ok(LevelFilter::Warn),
        "error" => Ok(LevelFilter::Error),
        _ => Err(serde::de::Error::custom(format!(
            "Unsuppored log level {:?}",
            s
        ))),
    }
}
