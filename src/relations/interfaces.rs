use crate::relations::keybound::Id;

// == Readers ==
pub trait ViewSetLike<T: Id> {
    fn contains(&self, k: T) -> bool;
    fn len(&self) -> usize;

    // TODO: Iterators
}

pub trait ViewMapLike<'a, K: Id, V: Id> {
    fn get(&self, k: K) -> Option<V>;
    fn contains_key(&self, k: K) -> bool;
    fn len(&self) -> usize;

    fn contains(&self, k: K, v: V) -> bool { self.get(k) == Some(v) }

    // TODO: Iterators
}

pub trait ViewMultiMapLike<'a, K: Id, V: Id> {
    type VMulti: ViewSetLike<V>;

    fn get(&'a self, k: K) -> Self::VMulti;
    fn contains_key(&self, k: K) -> bool;
    fn len(&self) -> usize;

    fn contains(&'a self, k: K, v: V) -> bool { self.get(k).contains(v) }

    // TODO: Iterators
}

// == Writers ==
pub trait SetLike<T: Id>: ViewSetLike<T> {
    fn insert(&mut self, t: T) -> Option<T>;  // return the evicted item (ex. a duplicate of this item)
    fn remove(&mut self, t: T) -> Option<T>;
} 

pub(crate) trait EvictSetLike<K: Id, V: Id>: ViewSetLike<V> {
    fn insert(&mut self, v: V, on_evict: impl FnOnce(K, V)) -> Option<V>;  // return the evicted item if one was evicted
    fn remove(&mut self, v: V, on_evict: impl FnOnce(K, V)) -> Option<V>;
} 

pub trait MapLike<'a, K: Id, V: Id>: ViewMapLike<'a, K, V> {
    fn insert(&mut self, k: K, v: V) -> Option<V>;
    fn expunge(&mut self, k: K) -> Option<V>;

    // TODO: Iterators
}

pub trait MultiMapLike<'a, K: Id, V: Id>: ViewMultiMapLike<'a, K, V> {
    type MMulti: SetLike<V>;
    type MExpunge;  // TODO: SetLike?

    fn get_mut(&'a mut self, k: K) -> Self::MMulti;
    fn insert(&mut self, k: K, v: V) -> Option<V>;  // note: only evicts if the inserted item was an exact duplicate
    fn expunge(&mut self, k: K) -> Self::MExpunge;

    // TODO: Iterators
}