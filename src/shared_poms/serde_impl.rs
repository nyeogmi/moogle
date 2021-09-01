use super::Pom;
use serde::{Serialize, Deserialize};

use crate::RawPom;

impl<T: Serialize> Serialize for Pom<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            self.elements.serialize(serializer)
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Pom<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        Ok(Pom::from_raw(RawPom::deserialize(deserializer)?))
    }
}