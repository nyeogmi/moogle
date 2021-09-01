use std::marker::PhantomData;

use super::RawPom;
use serde::{Deserialize, Serialize, de::{self, MapAccess, SeqAccess, Visitor}, ser::SerializeStruct};

impl<T: Serialize> Serialize for RawPom<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            let mut state = serializer.serialize_struct("Pom", 2)?;

            state.serialize_field("next_id", &self.next_id)?;
            state.serialize_field("elements", &self.members)?;

            state.end()
    }
}


#[derive(Deserialize)]
#[serde(field_identifier, rename_all = "snake_case")]
enum Field { NextId, Elements }

struct PomVisitor<T>(PhantomData<*const T>);

impl<'de, T: Deserialize<'de>> Deserialize<'de> for RawPom<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
            deserializer.deserialize_struct(
                "Pom", 
                &["next_id", "elements"], 
                PomVisitor(PhantomData)
            )
    }
}

impl<'de, T: Deserialize<'de>> Visitor<'de> for PomVisitor<T> {
    type Value = RawPom<T>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("next_id and elements")
    }

    fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
    where V: SeqAccess<'de> {
        let next_id = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;
        let elements = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(1, &self))?;
        Ok(RawPom::<T>::from_raw_parts(next_id, elements))
    }

    fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
    where V: MapAccess<'de> {
        let mut next_id = None;
        let mut elements = None;
        while let Some(key) = map.next_key()? {
            match key {
                Field::NextId => { 
                    if next_id.is_some() { return Err(de::Error::duplicate_field("next_id")) }
                    next_id = map.next_value()?; 
                }
                Field::Elements => { 
                    if elements.is_some() { return Err(de::Error::duplicate_field("elements")) }
                    elements = map.next_value()?; 
                }
            }
        }
        let next_id = next_id.ok_or_else(||de::Error::missing_field("next_id"))?;
        let elements = elements.ok_or_else(||de::Error::missing_field("elements"))?;
        Ok(RawPom::<T>::from_raw_parts(next_id, elements))
    }
}