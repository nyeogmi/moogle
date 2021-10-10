use std::{any::type_name, fmt::Debug};

use crate::{Id, Set};
use crate::methods::*;

impl<T: Debug> Debug for super::RefCellPom<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.elements.fmt(f)
    }
}

impl<T> Debug for super::Index<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(type_name::<super::Index<T>>()).field(&DebugIndex { set: &self.0 }).finish()
    }
}

impl<T: Debug> Debug for super::Elements<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(type_name::<super::Index<T>>()).field(&self.0).finish()
    }
}

struct DebugIndex<'a, T: 'static> {
    set: &'a Set<Id<T>>
}

impl<'a, T> Debug for DebugIndex<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entries(self.set.fwd().iter().map(|k| k.0)).finish()
    }
}