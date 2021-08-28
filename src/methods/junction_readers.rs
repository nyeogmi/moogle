use crate::id::IdLike;

pub trait ViewSet<'a, T: IdLike> {
    type Iter: 'a+DoubleEndedIterator<Item=T>;

    fn contains(&self, k: T) -> bool;
    fn len(&self) -> usize;

    fn iter(&'a self) -> Self::Iter;
}

pub trait ViewAnyToOne<'a, K: IdLike, V: IdLike> {
    type Iter: 'a+DoubleEndedIterator<Item=(K, V)>;
    type Keys: 'a+DoubleEndedIterator<Item=K>;
    type Values: 'a+DoubleEndedIterator<Item=V>;

    fn get(&self, k: K) -> Option<V>;
    fn contains_key(&self, k: K) -> bool;
    fn len(&self) -> usize;

    fn contains(&self, k: K, v: V) -> bool { self.get(k) == Some(v) }

    fn iter(&'a self) -> Self::Iter;
    fn keys(&'a self) -> Self::Keys;
    fn values(&'a self) -> Self::Values;
}

pub trait ViewAnyToSet<'a, K: IdLike, V: IdLike> {
    type VMulti: ViewSet<'a, V>;

    type Iter: 'a+DoubleEndedIterator<Item=(K, V)>;
    type Keys: 'a+DoubleEndedIterator<Item=K>;
    type Sets: 'a+DoubleEndedIterator<Item=(K, Self::VMulti)>;
    type Values: 'a+DoubleEndedIterator<Item=V>;

    fn get(&'a self, k: K) -> Self::VMulti;
    fn contains_key(&self, k: K) -> bool;

    fn len(&self) -> usize;  // TODO: Make sure it matches iter()
    fn keys_len(&self) -> usize;  // TODO: Make sure it matches iter()

    fn contains(&self, k: K, v: V) -> bool;

    fn iter(&'a self) -> Self::Iter;
    fn keys(&'a self) -> Self::Keys;
    fn sets(&'a self) -> Self::Sets;
    fn values(&'a self) -> Self::Values;
}
