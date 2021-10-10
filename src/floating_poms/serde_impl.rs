use super::{FloatingPom, floating::Floating};
use serde::{Serialize, Deserialize};

use crate::RawPom;

impl<T: Serialize> Serialize for FloatingPom<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            self.elements.borrow().serialize(serializer)
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for FloatingPom<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        Ok(FloatingPom::from_raw(RawPom::deserialize(deserializer)?))
    }
}

impl<T: Serialize> Serialize for Floating<'static, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            self.borrow().serialize(serializer)
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Floating<'static, T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
            Ok(Self::new(T::deserialize(deserializer)?))
    }
}