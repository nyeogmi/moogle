use crate::Id; 

use super::Pom;

use std::{iter::FromIterator};


impl<T> Default for Pom<T> {
    fn default() -> Self { Self::new() }
}

impl<T> Extend<T> for Pom<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for v in iter { self.insert(v); }
    }
}
impl<T> FromIterator<T> for Pom<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut xs = Pom::new();
        xs.extend(iter);
        xs
    }
}

impl<T> IntoIterator for Pom<T> {
    type Item = (Id<T>, T);

    type IntoIter = impl DoubleEndedIterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}