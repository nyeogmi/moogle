use std::{cell::RefCell, marker::PhantomData};

use crate::Id;

pub(super) fn cast_plain<T>(id: Id<RefCell<T>>) -> Id<T> {
    Id(id.0, PhantomData)
}

pub(super) fn cast_refcell<T>(id: Id<T>) -> Id<RefCell<T>> {
    Id(id.0, PhantomData)
}