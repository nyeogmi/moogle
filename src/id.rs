use std::hash::Hash;
use std::marker::PhantomData;

pub trait IdLike: Copy+Hash+Ord+PartialEq+'static {
    // named to avoid stepping on other libraries etc
    fn id_min_value() -> Self;
    fn id_max_value() -> Self;
}

impl IdLike for () {
    #[inline] fn id_min_value() -> Self { () }
    #[inline] fn id_max_value() -> Self { () }
}

impl IdLike for char {
    // NOTE: This depends on the definition of unicode and could change
    #[inline] fn id_min_value() -> Self { '\u{000000}' }
    #[inline] fn id_max_value() -> Self { '\u{10ffff}' }
}

impl IdLike for usize {
    #[inline] fn id_min_value() -> Self { Self::min_value() }
    #[inline] fn id_max_value() -> Self { Self::max_value() }
}
impl IdLike for u64 {
    #[inline] fn id_min_value() -> Self { Self::min_value() }
    #[inline] fn id_max_value() -> Self { Self::max_value() }
}
impl IdLike for u32 {
    #[inline] fn id_min_value() -> Self { Self::min_value() }
    #[inline] fn id_max_value() -> Self { Self::max_value() }
}
impl IdLike for u16 {
    #[inline] fn id_min_value() -> Self { Self::min_value() }
    #[inline] fn id_max_value() -> Self { Self::max_value() }
}
impl IdLike for u8 {
    #[inline] fn id_min_value() -> Self { Self::min_value() }
    #[inline] fn id_max_value() -> Self { Self::max_value() }
}

impl IdLike for isize {
    #[inline] fn id_min_value() -> Self { Self::min_value() }
    #[inline] fn id_max_value() -> Self { Self::max_value() }
}
impl IdLike for i64 {
    #[inline] fn id_min_value() -> Self { Self::min_value() }
    #[inline] fn id_max_value() -> Self { Self::max_value() }
}
impl IdLike for i32 {
    #[inline] fn id_min_value() -> Self { Self::min_value() }
    #[inline] fn id_max_value() -> Self { Self::max_value() }
}
impl IdLike for i16 {
    #[inline] fn id_min_value() -> Self { Self::min_value() }
    #[inline] fn id_max_value() -> Self { Self::max_value() }
}
impl IdLike for i8 {
    #[inline] fn id_min_value() -> Self { Self::min_value() }
    #[inline] fn id_max_value() -> Self { Self::max_value() }
}

impl<T1: IdLike, T2: IdLike> IdLike for (T1, T2) {
    fn id_min_value() -> Self { (T1::id_min_value(), T2::id_min_value()) }
    fn id_max_value() -> Self { (T1::id_max_value(), T2::id_max_value()) }
}

impl<T1: IdLike, T2: IdLike, T3: IdLike> IdLike for (T1, T2, T3) {
    fn id_min_value() -> Self { (T1::id_min_value(), T2::id_min_value(), T3::id_min_value()) }
    fn id_max_value() -> Self { (T1::id_max_value(), T2::id_max_value(), T3::id_max_value()) }
}

impl<T1: IdLike, T2: IdLike, T3: IdLike, T4: IdLike> IdLike for (T1, T2, T3, T4) {
    fn id_min_value() -> Self { (T1::id_min_value(), T2::id_min_value(), T3::id_min_value(), T4::id_min_value()) }
    fn id_max_value() -> Self { (T1::id_max_value(), T2::id_max_value(), T3::id_max_value(), T4::id_max_value()) }
}

// internal ID type, use if you have nothing else!!
pub struct Id<T>(pub(crate) u64, PhantomData<*const T>);

impl<T> Id<T> {
    pub(crate) fn new(val: u64) -> Self {
        Id(val, PhantomData)
    }

    pub fn get_value(&self) -> u64 {
        return self.0
    }
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self { *self }
}

impl<T> Copy for Id<T> {

}

impl<T> std::fmt::Debug for Id<T> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        fmt.debug_tuple(std::any::type_name::<T>()).field(&self.0).finish()
    }
}

impl<T> Hash for Id<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T> PartialEq for Id<T> {
    fn eq(&self, id2: &Id<T>) -> bool {
        self.0 == id2.0
    } 
}

impl<T> Eq for Id<T> {

}

impl<T> PartialOrd for Id<T> {
    fn partial_cmp(&self, id2: &Id<T>) -> std::option::Option<std::cmp::Ordering> { 
        self.0.partial_cmp(&id2.0)
    }
}

impl<T> Ord for Id<T> {
    fn cmp(&self, id2: &Self) -> std::cmp::Ordering {
        self.0.cmp(&id2.0)
    } 
}

impl<T: 'static> IdLike for Id<T> {
    #[inline] fn id_min_value() -> Self { Id(u64::min_value(), PhantomData) }
    #[inline] fn id_max_value() -> Self { Id(u64::max_value(), PhantomData) }
}

#[cfg(feature="serde1")]
mod serde_impl {
    use std::marker::PhantomData;

    use super::Id;
    use serde::{Serialize, Deserialize};

    impl<T> Serialize for Id<T> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer {
            self.0.serialize(serializer)
        }
    }

    impl<'de, T> Deserialize<'de> for Id<T> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de> {
            let u: u64 = u64::deserialize(deserializer)?;
            Ok(Id(u, PhantomData))
        }
    }
}