use std::cell::RefCell;

use crate::{Id, Set, RawPom};

use crate::methods::*;

pub use self::floating::*;

mod floating;
mod debug_impl;

#[cfg(feature="serde1")]
mod serde_impl;

mod extra;

#[derive(Clone)]
pub struct FloatingPom<T: 'static> { 
    index: Set<Id<T>>,
    elements: RefCell<RawPom<Floating<T>>>,
}

impl<T: 'static> FloatingPom<T> {
    pub fn new() -> Self {
        FloatingPom { 
            index: Set::new(),
            elements: RefCell::new(RawPom::new()),
        }
    }

    #[cfg(feature="serde1")]  // only needs to exist for serde
    pub(crate) fn from_raw(elements: RawPom<Floating<T>>) -> Self {
        let index = Set::new();
        for k in elements.keys() { index.fwd().insert(cast_plain(k)); }
        FloatingPom { index, elements: RefCell::new(elements) }
    }

    pub fn insert(&mut self, t: Floating<T>) -> Id<T> { 
        let id = cast_plain(self.elements.borrow_mut().insert(t));
        self.index.fwd().insert(id);
        id
    }
    pub fn remove(&mut self, k: Id<T>) -> Option<Floating<T>> { 
        self.index.fwd().remove(k);
        self.elements.borrow_mut().remove(cast_refcell(k)) 
    }

    pub fn get(&self, k: Id<T>) -> Option<Floating<T>> { 
        let floating = {
            let elements = self.elements.borrow();
            let floating_ref = elements.get(cast_refcell(k));
            floating_ref.map(|x| x.internal_share())
        };
        return floating
    }

    pub fn contains_key(&self, k: Id<T>) -> bool { 
        self.index.fwd().contains(k)
    }

    pub fn len(&self) -> usize { 
        self.index.fwd().len() 
    }

    pub fn keys(&self) -> impl '_+DoubleEndedIterator<Item=Id<T>> {
        self.index.fwd().iter()
    }
}