use base64::{engine::general_purpose::STANDARD, Engine};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub fn serialize<S, T>(bytes: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: AsRef<[u8]>,
{
    STANDARD.encode(bytes.as_ref()).serialize(serializer)
}

pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: TryFrom<Vec<u8>>,
    T::Error: std::fmt::Debug,
{
    let s = String::deserialize(deserializer)?;
    let bytes = STANDARD.decode(s).map_err(serde::de::Error::custom)?;
    T::try_from(bytes).map_err(|e| serde::de::Error::custom(format!("{:?}", e)))
}
