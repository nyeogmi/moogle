use crate::Id; 

use super::RawPom;

use std::{iter::FromIterator};


impl<T> Default for RawPom<T> {
    fn default() -> Self { Self::new() }
}

impl<T> Extend<T> for RawPom<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for v in iter {
            self.insert(v);
        }
    }
}
impl<T> FromIterator<T> for RawPom<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut xs = RawPom::new();
        xs.extend(iter);
        xs
    }
}

impl<T> IntoIterator for RawPom<T> {
    type Item = (Id<T>, T);

    type IntoIter = impl DoubleEndedIterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.members.into_iter()
    }
}