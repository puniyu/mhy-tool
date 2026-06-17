pub mod gacha;

use serde::de::{self, Deserializer, Visitor};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub struct Uin(pub u64);

impl Uin {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}

impl From<u64> for Uin {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<Uin> for u64 {
    fn from(uin: Uin) -> Self {
        uin.0
    }
}

impl std::fmt::Display for Uin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for Uin {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UinVisitor;

        impl Visitor<'_> for UinVisitor {
            type Value = Uin;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a u64 or a string containing a u64")
            }

            fn visit_u64<E: de::Error>(self, v: u64) -> Result<Uin, E> {
                Ok(Uin(v))
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<Uin, E> {
                v.parse::<u64>()
                    .map(Uin)
                    .map_err(|_| de::Error::invalid_value(de::Unexpected::Str(v), &self))
            }
        }

        deserializer.deserialize_any(UinVisitor)
    }
}
