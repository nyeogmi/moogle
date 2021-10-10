use crate::Id; 

use super::{RefCellPom, cast::cast_plain};

use std::{cell::RefCell, iter::FromIterator};


impl<T> Default for RefCellPom<T> {
    fn default() -> Self { Self::new() }
}

impl<T> Extend<RefCell<T>> for RefCellPom<T> {
    fn extend<I: IntoIterator<Item = RefCell<T>>>(&mut self, iter: I) {
        for v in iter { self.insert(v); }
    }
}
impl<T> FromIterator<RefCell<T>> for RefCellPom<T> {
    fn from_iter<I: IntoIterator<Item = RefCell<T>>>(iter: I) -> Self {
        let mut xs = RefCellPom::new();
        xs.extend(iter);
        xs
    }
}

impl<T> IntoIterator for RefCellPom<T> {
    type Item = (Id<T>, RefCell<T>);

    type IntoIter = impl DoubleEndedIterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter().map(|(k, v)| (cast_plain(k), v) )
    }
}