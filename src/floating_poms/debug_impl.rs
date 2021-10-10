use std::{fmt::Debug};

impl<T: Debug> Debug for super::FloatingPom<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.elements.fmt(f)
    }
}