use super::{Set, ToMany, ToOne};
use serde::{Serialize, Deserialize};

use crate::{IdLike, RawSet, RawToMany, RawToOne};

impl<T: IdLike+Serialize> Serialize for Set<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            self.raw.borrow().serialize(serializer)
    }
}

impl<'de, T: IdLike+Deserialize<'de>> Deserialize<'de> for Set<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
            let mut xs = Set::new();
            let raw = xs.raw();
            let mut des = RawSet::<T>::deserialize(deserializer)?;
            std::mem::swap(raw, &mut des);
            Ok(xs)
    }
}

impl<K: IdLike+Serialize, V: Serialize+IdLike> Serialize for ToOne<K, V> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            self.raw.borrow().serialize(serializer)
    }
}

impl<'de, K: IdLike+Deserialize<'de>, V: IdLike+Deserialize<'de>> Deserialize<'de> for ToOne<K, V> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
            let mut xs = ToOne::new();
            let raw = xs.raw();
            let mut des = RawToOne::<K, V>::deserialize(deserializer)?;
            std::mem::swap(raw, &mut des);
            Ok(xs)
    }
}

impl<K: IdLike+Serialize, V: IdLike+Serialize> Serialize for ToMany<K, V> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            self.raw.borrow().serialize(serializer)
    }
}

impl<'de, K: IdLike+Deserialize<'de>, V: IdLike+Deserialize<'de>> Deserialize<'de> for ToMany<K, V> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
            let mut xs = ToMany::new();
            let raw = xs.raw();
            let mut des = RawToMany::<K, V>::deserialize(deserializer)?;
            std::mem::swap(raw, &mut des);
            Ok(xs)
    }
}
