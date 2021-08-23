// == Readers ==
pub trait ViewSetLike<T> {
    fn contains(&self, k: &T) -> bool;
    fn len(&self) -> usize;

    // TODO: Iterators
}

pub trait ViewMapLike<'a, K, V> {
    fn get(&'a self, k: &K) -> Option<&'a V>;
    fn contains_key(&'a self, k: &K) -> bool;
    fn len(&'a self) -> usize;

    // TODO: Iterators
}

pub trait ViewMultiMapLike<'a, K, V> {
    type VMulti: ViewSetLike<V>;

    fn get(&'a self, k: &K) -> Self::VMulti;
    fn contains_key(&'a self, k: &K) -> bool;
    fn len(&'a self) -> usize;

    // TODO: Iterators
}

// == Writers ==
pub trait SetLike<T>: ViewSetLike<T> {
    fn insert(&mut self, t: T) -> bool;  // false if present
    fn remove(&mut self, t: &T) -> Option<T>;
} 

pub trait EvictSetLike<K, V>: ViewSetLike<V> {
    fn insert(&mut self, v: V, on_evict: impl FnOnce(&K, &V)) -> bool;
    fn remove(&mut self, v: &V, on_evict: impl FnOnce(&K, &V)) -> Option<V>;
} 

pub trait MapLike<'a, K, V>: ViewMapLike<'a, K, V> {
    fn insert(&'a mut self, k: K, v: V) -> Option<V>;
    fn expunge(&'a mut self, k: &K) -> Option<V>;

    // TODO: Iterators
}

pub trait MultiMapLike<'a, K, V>: ViewMultiMapLike<'a, K, V> {
    type MMulti: SetLike<V>;
    type MExpunge;  // TODO: SetLike?

    fn get_mut(&'a mut self, k: K) -> Self::MMulti;
    fn insert(&'a mut self, k: K, v: V);
    fn expunge(&'a mut self, k: &K) -> Self::MExpunge;

    // TODO: Iterators
}

// == Extension methods ==
trait EqViewMapLike<'a, K, V>: ViewMapLike<'a, K, V> where V: PartialEq {
    fn contains(&'a self, k: &K, v: &'a V) -> bool { self.get(k) == Some(v) }
}

impl<'a, T, K, V> EqViewMapLike<'a, K, V> for T where V: PartialEq, T: ViewMapLike<'a, K, V> { }

trait EqViewMultiMapLike<'a, K, V>: ViewMultiMapLike<'a, K, V> where V: PartialEq {
    fn contains(&'a self, k: &'a K, v: &'a V) -> bool { self.get(k).contains(v) }
}

impl<'a, T, K, V> EqViewMultiMapLike<'a, K, V> for T where V: PartialEq, T: ViewMultiMapLike<'a, K, V> { }



// == MISC ==
/*
pub trait ListLike<T> {
    fn push(&mut self, t: T) -> usize;  
    fn remove(&mut self, ix: usize) -> Option<T>;  // only if present
    fn pop(&mut self) -> Option<T> {
        let l = self.len();
        if l > 0 { return self.remove(l - 1); }
        return None;
    }
    fn contains(&mut self, ix: usize) -> bool;

    fn len(&self) -> usize;

    // TODO: Iterators
}
*/
