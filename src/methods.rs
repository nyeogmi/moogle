// named "methods" instead of "traits" to convey that you probably wanna import this if you'd like access to the methods

use crate::keybound::Id;

// == Readers ==
pub trait ViewSetLike<'a, T: Id> {
    type Iter: 'a+Iterator<Item=T>;

    fn contains(&self, k: T) -> bool;
    fn len(&self) -> usize;

    fn iter(&'a self) -> Self::Iter;
}

pub trait ViewMapLike<'a, K: Id, V: Id> {
    type Iter: 'a+Iterator<Item=(K, V)>;
    type Keys: 'a+Iterator<Item=K>;
    type Values: 'a+Iterator<Item=V>;

    fn get(&self, k: K) -> Option<V>;
    fn contains_key(&self, k: K) -> bool;
    fn len(&self) -> usize;

    fn contains(&self, k: K, v: V) -> bool { self.get(k) == Some(v) }

    fn iter(&'a self) -> Self::Iter;
    fn keys(&'a self) -> Self::Keys;
    fn values(&'a self) -> Self::Values;
}

pub trait ViewMultiMapLike<'a, K: Id, V: Id> {
    type VMulti: ViewSetLike<'a, V>;

    type Iter: 'a+Iterator<Item=(K, V)>;
    type Keys: 'a+Iterator<Item=K>;
    type Sets: 'a+Iterator<Item=(K, Self::VMulti)>;
    type Values: 'a+Iterator<Item=V>;

    fn get(&'a self, k: K) -> Self::VMulti;
    fn contains_key(&self, k: K) -> bool;

    fn len(&self) -> usize;  // TODO: Make sure it matches iter()
    fn keys_len(&self) -> usize;  // TODO: Make sure it matches iter()

    fn contains(&'a self, k: K, v: V) -> bool { self.get(k).contains(v) }

    fn iter(&'a self) -> Self::Iter;
    fn keys(&'a self) -> Self::Keys;
    fn sets(&'a self) -> Self::Sets;
    fn values(&'a self) -> Self::Values;
}

// == Writers ==
pub trait SetLike<'a, T: Id>: ViewSetLike<'a, T> {
    fn insert(&mut self, t: T) -> Option<T>;  // return the evicted item (ex. a duplicate of this item)
    fn remove(&mut self, t: T) -> Option<T>;
} 

pub(crate) trait EvictSetLike<'a, K: Id, V: Id>: ViewSetLike<'a, V> {
    fn insert(&mut self, v: V, on_evict: impl FnOnce(K, V)) -> Option<V>;  // return the evicted item if one was evicted
    fn remove(&mut self, v: V, on_evict: impl FnOnce(K, V)) -> Option<V>;
} 

pub trait MapLike<'a, K: Id, V: Id>: ViewMapLike<'a, K, V> {
    fn insert(&mut self, k: K, v: V) -> Option<V>;
    fn expunge(&mut self, k: K) -> Option<V>;

    fn remove(&mut self, k: K, v: V) -> Option<V> {
        if self.get(k) == Some(v) { self.expunge(k) } else { None }
    }
}

pub trait MultiMapLike<'a, K: Id, V: Id>: ViewMultiMapLike<'a, K, V> {
    type MMulti: SetLike<'a, V>;
    type MExpunge;  // TODO: SetLike?

    fn get_mut(&'a mut self, k: K) -> Self::MMulti;
    fn insert(&mut self, k: K, v: V) -> Option<V>;  // note: only evicts if the inserted item was an exact duplicate
    fn expunge(&mut self, k: K) -> Self::MExpunge;

    fn remove(&'a mut self, k: K, v: V) -> Option<V> {
        self.get_mut(k).remove(v)
    }
}