use crate::keybound::Id;

pub trait ViewSet<'a, T: Id> {
    type Iter: 'a+Iterator<Item=T>;

    fn contains(&self, k: T) -> bool;
    fn len(&self) -> usize;

    fn iter(&'a self) -> Self::Iter;
}

pub trait ViewAnyToOne<'a, K: Id, V: Id> {
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

pub trait ViewAnyToSet<'a, K: Id, V: Id> {
    type VMulti: ViewSet<'a, V>;

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