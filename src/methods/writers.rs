use crate::keybound::Id;

use super::{ViewSet, ViewAnyToOne, ViewAnyToSet};

pub trait Set<'a, T: Id>: ViewSet<'a, T> {
    fn insert(&mut self, t: T) -> Option<T>;  // return the evicted item (ex. a duplicate of this item)
    fn remove(&mut self, t: T) -> Option<T>;
} 

pub trait AnyToOne<'a, K: Id, V: Id>: ViewAnyToOne<'a, K, V> {
    fn insert(&mut self, k: K, v: V) -> Option<V>;
    fn expunge(&mut self, k: K) -> Option<V>;

    fn remove(&mut self, k: K, v: V) -> Option<V> {
        if self.get(k) == Some(v) { self.expunge(k) } else { None }
    }
}

pub trait AnyToSet<'a, K: Id, V: Id>: ViewAnyToSet<'a, K, V> {
    type MMulti: Set<'a, V>;
    type MExpunge;  // TODO: Set?

    fn get_mut(&'a mut self, k: K) -> Self::MMulti;
    fn insert(&mut self, k: K, v: V) -> Option<V>;  // note: only evicts if the inserted item was an exact duplicate
    fn expunge(&mut self, k: K) -> Self::MExpunge;

    fn remove(&'a mut self, k: K, v: V) -> Option<V> {
        self.get_mut(k).remove(v)
    }
}