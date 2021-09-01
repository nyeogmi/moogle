use std::any::type_name;
use std::marker::PhantomData;

use super::{RawSet, RawToMany, RawToOne};
use serde::de::Visitor;
use serde::ser::{SerializeMap, SerializeSeq};
use serde::{Serialize, Deserialize};

use crate::methods::*;

use crate::IdLike;

// == sets ==
impl<T: IdLike+Serialize> Serialize for RawSet<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            let mut out = serializer.serialize_seq(Some(self.fwd().len()))?;
            for k in self.fwd().iter() {
                out.serialize_element(&k)?;
            }
            
            out.end()
    }
}

impl<'de, T: IdLike+Deserialize<'de>> Deserialize<'de> for RawSet<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
            deserializer.deserialize_seq(RawSetVisitor(PhantomData))
    }
}

struct RawSetVisitor<T: IdLike>(PhantomData<*const T>);

impl<'de, T: IdLike+Deserialize<'de>> Visitor<'de> for RawSetVisitor<T> {
    type Value = RawSet<T>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_fmt(format_args!("a set of {}", type_name::<T>()))
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where A: serde::de::SeqAccess<'de> {
        let mut result = RawSet::new();

        while let Some(x) = seq.next_element()? {
            result.mut_fwd().insert(x);
        }

        Ok(result)
    }
}

// == ToOne ==
impl<K: IdLike+Serialize, V: Serialize+IdLike> Serialize for RawToOne<K, V> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            let mut out = serializer.serialize_map(Some(self.fwd().len()))?;
            for (k, v) in self.fwd().iter() {
                out.serialize_entry(&k, &v)?;
            }
            out.end()
    }
}

impl<'de, K: IdLike+Deserialize<'de>, V: IdLike+Deserialize<'de>> Deserialize<'de> for RawToOne<K, V> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
            deserializer.deserialize_map(RawToOneVisitor(PhantomData))
    }
}


struct RawToOneVisitor<K: IdLike, V: IdLike>(PhantomData<(*const K, *const V)>);

impl<'de, K: IdLike+Deserialize<'de>, V: IdLike+Deserialize<'de>> Visitor<'de> for RawToOneVisitor<K, V> {
    type Value = RawToOne<K, V>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_fmt(format_args!("a map from {} to {}", type_name::<K>(), type_name::<V>()))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where A: serde::de::MapAccess<'de> {
        let mut result = RawToOne::new();

        while let Some((k, v)) = map.next_entry()? {
            result.mut_fwd().insert(k, v);
        }

        Ok(result)
    }
}

// == ToMany ==
impl<K: IdLike+Serialize, V: IdLike+Serialize> Serialize for RawToMany<K, V> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            let mut out = serializer.serialize_map(Some(self.fwd().len()))?;
            for (k, v) in self.fwd().iter() {
                out.serialize_entry(&k, &v)?;
            }
            out.end()
    }
}

impl<'de, K: IdLike+Deserialize<'de>, V: IdLike+Deserialize<'de>> Deserialize<'de> for RawToMany<K, V> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
            deserializer.deserialize_map(RawToManyVisitor(PhantomData))
    }
}

struct RawToManyVisitor<K: IdLike, V: IdLike>(PhantomData<(*const K, *const V)>);

impl<'de, K: IdLike+Deserialize<'de>, V: IdLike+Deserialize<'de>> Visitor<'de> for RawToManyVisitor<K, V> {
    type Value = RawToMany<K, V>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_fmt(format_args!("a map from {} to {}", type_name::<K>(), type_name::<V>()))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where A: serde::de::MapAccess<'de> {
        let mut result = RawToMany::new();

        while let Some((k, vs)) = map.next_entry::<K, Vec<V>>()? {
            // NYEO NOTE: TODO: This performance is terrible
            for v in vs {
                result.mut_fwd().insert(k, v);
            }
        }

        Ok(result)
    }
}