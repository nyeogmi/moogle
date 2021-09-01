use serde::{Serialize, Deserialize};

use crate::IdLike;

use super::{OneToOne, OneToMany, ManyToOne, ManyToMany};
use crate::{RawOneToOne, RawOneToMany, RawManyToOne, RawManyToMany};

impl<A: IdLike+Serialize, B: IdLike+Serialize> Serialize for OneToOne<A, B> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            self.raw.borrow().serialize(serializer)
    }
}

impl<'de, A: IdLike+Deserialize<'de>, B: IdLike+Deserialize<'de>> Deserialize<'de> for OneToOne<A, B> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
            let mut xs = OneToOne::new();
            let raw = xs.raw();
            let mut des = RawOneToOne::<A, B>::deserialize(deserializer)?;
            std::mem::swap(raw, &mut des);
            Ok(xs)
    }
}

impl<A: IdLike+Serialize, B: IdLike+Serialize> Serialize for OneToMany<A, B> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            self.raw.borrow().serialize(serializer)
    }
}

impl<'de, A: IdLike+Deserialize<'de>, B: IdLike+Deserialize<'de>> Deserialize<'de> for OneToMany<A, B> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
            let mut xs = OneToMany::new();
            let raw = xs.raw();
            let mut des = RawOneToMany::<A, B>::deserialize(deserializer)?;
            std::mem::swap(raw, &mut des);
            Ok(xs)
    }
}

impl<A: IdLike+Serialize, B: IdLike+Serialize> Serialize for ManyToOne<A, B> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            self.raw.borrow().serialize(serializer)
    }
}

impl<'de, A: IdLike+Deserialize<'de>, B: IdLike+Deserialize<'de>> Deserialize<'de> for ManyToOne<A, B> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
            let mut xs = ManyToOne::new();
            let raw = xs.raw();
            let mut des = RawManyToOne::<A, B>::deserialize(deserializer)?;
            std::mem::swap(raw, &mut des);
            Ok(xs)
    }
}

impl<A: IdLike+Serialize, B: IdLike+Serialize> Serialize for ManyToMany<A, B> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            self.raw.borrow().serialize(serializer)
    }
}

impl<'de, A: IdLike+Deserialize<'de>, B: IdLike+Deserialize<'de>> Deserialize<'de> for ManyToMany<A, B> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
            let mut xs = ManyToMany::new();
            let raw = xs.raw();
            let mut des = RawManyToMany::<A, B>::deserialize(deserializer)?;
            std::mem::swap(raw, &mut des);
            Ok(xs)
    }
}