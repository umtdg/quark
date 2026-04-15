use std::{collections::HashMap, fmt::Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Shortcut string is empty")]
    Empty,

    #[error("Shortcut has empty key or no key at all")]
    EmptyKey,

    #[error("Duplicate modifier {0}")]
    DuplicateModifier(String),

    #[error("Invalid shortcut modifier {0}")]
    InvalidModifier(String),
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Modifiers {
    ctrl: bool,
    alt: bool,
    meta: bool,
    shift: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Shortcut {
    modifiers: Modifiers,
    key: String,
}

impl Display for Shortcut {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut parts = Vec::with_capacity(5);
        if self.modifiers.ctrl {
            parts.push("ctrl");
        }

        if self.modifiers.alt {
            parts.push("alt");
        }

        if self.modifiers.meta {
            parts.push("meta");
        }

        if self.modifiers.shift {
            parts.push("shift");
        }

        parts.push(&self.key);
        write!(f, "{}", parts.join("-"))
    }
}

impl FromStr for Shortcut {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split("-").collect();
        if parts.is_empty() {
            return Err(Error::Empty);
        }

        let mut modifiers = Modifiers::default();
        let mut key: Option<&str> = None;

        for (i, part) in parts.iter().enumerate() {
            match part.to_lowercase().as_str() {
                "ctrl" | "control" => {
                    if modifiers.ctrl {
                        return Err(Error::DuplicateModifier("ctrl".into()));
                    }

                    modifiers.ctrl = true;
                }
                "alt" | "option" => {
                    if modifiers.alt {
                        return Err(Error::DuplicateModifier("alt".into()));
                    }

                    modifiers.alt = true;
                }
                "meta" | "super" | "win" | "cmd" => {
                    if modifiers.meta {
                        return Err(Error::DuplicateModifier("meta".into()));
                    }

                    modifiers.meta = true;
                }
                "shift" => {
                    if modifiers.shift {
                        return Err(Error::DuplicateModifier("shift".into()));
                    }

                    modifiers.shift = true;
                }
                _ => {
                    if i != parts.len() - 1 {
                        return Err(Error::InvalidModifier(part.to_string()));
                    }

                    if part.is_empty() {
                        return Err(Error::EmptyKey);
                    }

                    key = Some(part);
                }
            }
        }

        let key = key.ok_or(Error::EmptyKey)?;

        Ok(Shortcut {
            modifiers,
            key: key.to_lowercase(),
        })
    }
}

impl Serialize for Shortcut {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Shortcut {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Shortcut::from_str(&s).map_err(serde::de::Error::custom)
    }
}

pub type ShortcutMap<T> = HashMap<Shortcut, T>;
