use std::{cell::{BorrowError, BorrowMutError, Ref, RefCell, RefMut}, fmt::{self, Formatter}, marker::PhantomData, rc::Rc};
use std::fmt::Debug;

use crate::Id;

pub(super) fn cast_plain<T>(id: Id<Floating<T>>) -> Id<T> {
    Id(id.0, PhantomData)
}

pub(super) fn cast_refcell<T>(id: Id<T>) -> Id<Floating<T>> {
    Id(id.0, PhantomData)
}


// the below is an implementation of a big subset of the RefCell interface
// this attempts to hide the implementation detail of Rc

pub struct Floating<T>(Rc<RefCell<T>>);

impl<T> Floating<T> {
    pub fn new(value: T) -> Floating<T> {
        Floating(Rc::new(RefCell::new(value)))
    }

    pub fn replace(&self, value: T) -> T {
        self.0.replace(value)
    }

    pub fn replace_with<F>(&self, value: F) -> T where F: FnOnce(&mut T) -> T {
        self.0.replace_with(value)
    }

    pub fn swap(&self, other: &Floating<T>) {
        self.0.swap(&other.0)
    }

    pub fn borrow(&self) -> Ref<'_, T> {
        self.0.borrow()
    }

    pub fn try_borrow(&self) -> Result<Ref<'_, T>, BorrowError> {
        self.0.try_borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        self.0.borrow_mut()
    }

    pub fn try_borrow_mut(&self) -> Result<RefMut<'_, T>, BorrowMutError> {
        self.0.try_borrow_mut()
    }

    pub(crate) fn internal_share(&self) -> Floating<T> {
        // Shares the Rc
        // Because we provide this, we must assume that many references to the same Floating can exist
        // However, because it's limited to the crate, the general public may not use it
        let it: Rc<RefCell<T>> = self.0.clone();
        return Self(it)
    }
}

impl<T: Clone> Clone for Floating<T> {
    fn clone(&self) -> Self {
        let ref_cell: &RefCell<T> = self.0.as_ref();
        Self(Rc::new(ref_cell.clone()))
    }
}

impl<T: Debug> Debug for Floating<T> {
    // borrowed from the RefCell implementation
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self.try_borrow() {
            Ok(borrow) => f.debug_struct("Floating").field("value", &borrow).finish(),
            Err(_) => {
                // The RefCell is mutably borrowed so we can't look at its value
                // here. Show a placeholder instead.
                struct BorrowedPlaceholder;

                impl Debug for BorrowedPlaceholder {
                    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
                        f.write_str("<borrowed>")
                    }
                }

                f.debug_struct("RefCell").field("value", &BorrowedPlaceholder).finish()
            }
        }
    }
}

impl<T: Default> Default for Floating<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: Eq> Eq for Floating<T> {
}

impl<T: Ord> Ord for Floating<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let x1: &RefCell<T> = &*self.0;
        let x2: &RefCell<T> = &*other.0;
        x1.cmp(x2)
    }
}

impl<T: PartialEq> PartialEq<Floating<T>> for Floating<T> {
    fn eq(&self, other: &Floating<T>) -> bool {
        let x1: &RefCell<T> = &*self.0;
        let x2: &RefCell<T> = &*other.0;
        x1 == x2
    }
}

impl<T: PartialOrd> PartialOrd<Floating<T>> for Floating<T> {
    fn partial_cmp(&self, other: &Floating<T>) -> Option<std::cmp::Ordering> {
        let x1: &RefCell<T> = &*self.0;
        let x2: &RefCell<T> = &*other.0;
        x1.partial_cmp(x2)
    }
}