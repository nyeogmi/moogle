use crate::id::IdLike;

pub trait SharedAnySet<'a, T: IdLike> {
    type Iter: 'a+DoubleEndedIterator<Item=T>;

    fn contains(&self, k: T) -> bool;
    fn len(&self) -> usize;

    fn iter(&self) -> Self::Iter;

    fn insert(&self, t: T) -> Option<T>; 
    fn remove(&self, t: T) -> Option<T>;
}

pub trait SharedAnyToOne<'a, K: IdLike, V: IdLike> {
    type Iter: 'a+DoubleEndedIterator<Item=(K, V)>;
    type Keys: 'a+DoubleEndedIterator<Item=K>;
    type Values: 'a+DoubleEndedIterator<Item=V>;

    fn get(&self, k: K) -> Option<V>;
    fn contains_key(&self, k: K) -> bool;
    fn len(&self) -> usize;

    fn contains(&self, k: K, v: V) -> bool { self.get(k) == Some(v) }

    fn iter(&self) -> Self::Iter;
    fn keys(&self) -> Self::Keys;
    fn values(&self) -> Self::Values;

    fn insert(&self, k: K, v: V) -> Option<V>;
    fn expunge(&self, k: K) -> Option<V>;

    fn remove(&self, k: K, v: V) -> Option<V> {
        if self.get(k) == Some(v) { self.expunge(k) } else { None }
    }
}

pub trait SharedAnyToMany<'a, K: IdLike, V: IdLike> {
    type Multi: SharedAnySet<'a, V>;
    type Expunge;  // TODO: Set?

    type Iter: 'a+DoubleEndedIterator<Item=(K, V)>;
    type Keys: 'a+DoubleEndedIterator<Item=K>;
    type Sets: 'a+DoubleEndedIterator<Item=(K, Self::Multi)>;
    type Values: 'a+DoubleEndedIterator<Item=V>;

    fn get(&self, k: K) -> Self::Multi;
    fn contains_key(&self, k: K) -> bool;

    fn len(&self) -> usize;  // TODO: Make sure it matches iter()
    fn keys_len(&self) -> usize;  // TODO: Make sure it matches iter()

    fn contains(&'a self, k: K, v: V) -> bool { self.get(k).contains(v) }

    fn iter(&self) -> Self::Iter;
    fn keys(&self) -> Self::Keys;
    fn sets(&self) -> Self::Sets;
    fn values(&self) -> Self::Values;

    fn insert(&self, k: K, v: V) -> Option<V>;  // note: only evicts if the inserted item was an exact duplicate
    fn expunge(&self, k: K) -> Self::Expunge;

    fn remove(&self, k: K, v: V) -> Option<V> {
        self.get(k).remove(v)
    }
}