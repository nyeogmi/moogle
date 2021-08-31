use crate::id::IdLike;

use super::{ViewSet, ViewAnyToOne, ViewAnyToMany};

pub trait AnySet<'a, T: IdLike>: ViewSet<'a, T> {
    fn insert(&mut self, t: T) -> Option<T>;  // return the evicted item (ex. a duplicate of this item)
    fn remove(&mut self, t: T) -> Option<T>;
} 

pub trait AnyToOne<'a, K: IdLike, V: IdLike>: ViewAnyToOne<'a, K, V> {
    fn insert(&mut self, k: K, v: V) -> Option<V>;
    fn expunge(&mut self, k: K) -> Option<V>;

    fn remove(&mut self, k: K, v: V) -> Option<V> {
        if self.get(k) == Some(v) { self.expunge(k) } else { None }
    }
}

pub trait AnyToMany<'a, K: IdLike, V: IdLike>: ViewAnyToMany<'a, K, V> {
    type MMulti: AnySet<'a, V>;
    type MExpunge;  // TODO: Set?

    fn get_mut(&'a mut self, k: K) -> Self::MMulti;
    fn insert(&mut self, k: K, v: V) -> Option<V>;  // note: only evicts if the inserted item was an exact duplicate
    fn expunge(&mut self, k: K) -> Self::MExpunge;

    fn remove(&'a mut self, k: K, v: V) -> Option<V> {
        self.get_mut(k).remove(v)
    }
}