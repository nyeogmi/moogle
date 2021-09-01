use serde::de::Visitor;
use serde::ser::SerializeSeq;
use serde::{Serializer, Deserializer, Serialize, Deserialize};

use std::any::type_name;

use std::iter::Iterator;

use crate::IdLike;
use crate::methods::*;

use super::{RawOneToOne, RawOneToMany, RawManyToOne, RawManyToMany};

impl<A: IdLike+Serialize, B: IdLike+Serialize> Serialize for RawOneToOne<A, B> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            serialize(self.fwd().len(), self.fwd().iter(), serializer)
    }
}

impl<'de, A: IdLike+Deserialize<'de>, B: IdLike+Deserialize<'de>> Deserialize<'de> for RawOneToOne<A, B> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de> {
            deserialize(
                RawOneToOne::new(), 
                |xs, a, b| { xs.mut_fwd().insert(a, b); }, 
                deserializer
            )
    }
}

impl<A: IdLike+Serialize, B: IdLike+Serialize> Serialize for RawOneToMany<A, B> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            serialize(self.fwd().len(), self.fwd().iter(), serializer)
    }
}

impl<'de, A: IdLike+Deserialize<'de>, B: IdLike+Deserialize<'de>> Deserialize<'de> for RawOneToMany<A, B> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de> {
            deserialize(
                RawOneToMany::new(), 
                |xs, a, b| { xs.mut_fwd().insert(a, b); }, 
                deserializer
            )
    }
}

impl<A: IdLike+Serialize, B: IdLike+Serialize> Serialize for RawManyToOne<A, B> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            serialize(self.fwd().len(), self.fwd().iter(), serializer)
    }
}

impl<'de, A: IdLike+Deserialize<'de>, B: IdLike+Deserialize<'de>> Deserialize<'de> for RawManyToOne<A, B> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de> {
            deserialize(
                RawManyToOne::new(), 
                |xs, a, b| { xs.mut_fwd().insert(a, b); }, 
                deserializer
            )
    }
}

impl<A: IdLike+Serialize, B: IdLike+Serialize> Serialize for RawManyToMany<A, B> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            serialize(self.fwd().len(), self.fwd().iter(), serializer)
    }
}

impl<'de, A: IdLike+Deserialize<'de>, B: IdLike+Deserialize<'de>> Deserialize<'de> for RawManyToMany<A, B> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de> {
            deserialize(
                RawManyToMany::new(), 
                |xs, a, b| { xs.mut_fwd().insert(a, b); }, 
                deserializer
            )
    }
}

fn serialize<A: Serialize, B: Serialize, S: Serializer>(
    len: usize, 
    iter: impl Iterator<Item=(A, B)>, 
    serializer: S 
) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> {
    let mut seq = serializer.serialize_seq(Some(len))?;
    for e in iter { seq.serialize_element(&e)?; }
    seq.end()
}

struct JunctionVisitor<'a, P, A, B> {
    value: P, 
    on_item: &'a mut dyn FnMut(&mut P, A, B)
}

fn deserialize<'de, P, A: Deserialize<'de>, B: Deserialize<'de>, D: Deserializer<'de>>(
    value: P,
    mut on_item: impl FnMut(&mut P, A, B),
    deserializer: D 
) -> Result<P, <D as Deserializer<'de>>::Error> {
    deserializer.deserialize_map(JunctionVisitor::<P, A, B> { value, on_item: &mut on_item })
}

impl<'de, P, A: Deserialize<'de>, B: Deserialize<'de>> Visitor<'de> for JunctionVisitor<'_, P, A, B> {
    type Value = P;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_fmt(format_args!("junction data ({}, {})", type_name::<A>(), type_name::<B>()))
    }

    fn visit_seq<S>(mut self, mut seq: S) -> Result<Self::Value, S::Error>
    where
        S: serde::de::SeqAccess<'de>,
    {
        while let Some((a, b)) = seq.next_element()? {
            (self.on_item)(&mut self.value, a, b);
        }

        Ok(self.value)
    }
}