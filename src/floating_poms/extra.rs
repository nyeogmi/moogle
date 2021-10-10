use super::{FloatingPom, floating::Floating};

use std::{iter::FromIterator};


impl<T> Default for FloatingPom<T> {
    fn default() -> Self { Self::new() }
}

impl<T> Extend<Floating<T>> for FloatingPom<T> {
    fn extend<I: IntoIterator<Item = Floating<T>>>(&mut self, iter: I) {
        for v in iter { self.insert(v); }
    }
}
impl<T> FromIterator<Floating<T>> for FloatingPom<T> {
    fn from_iter<I: IntoIterator<Item = Floating<T>>>(iter: I) -> Self {
        let mut xs = FloatingPom::new();
        xs.extend(iter);
        xs
    }
}