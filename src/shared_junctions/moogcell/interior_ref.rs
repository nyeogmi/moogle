use std::cell::{Cell, UnsafeCell};
use std::mem::MaybeUninit;
use std::marker::PhantomData;

use super::MoogCell;

pub struct InteriorRef<T, I> {
    parent: PhantomData<*const T>,
    state: Cell<u64>, 
    value: UnsafeCell<MaybeUninit<*const I>>,
}

impl<T, I> Clone for InteriorRef<T, I> {
    fn clone(&self) -> Self { 
        InteriorRef {
            parent: self.parent,
            state: self.state.clone(),
            value: UnsafeCell::new(unsafe {*self.value.get()}.clone()),
        }
    }
}

impl<T> MoogCell<T> {
    pub const fn create_interior_ref<X>(&self) -> InteriorRef<T, X> { 
        InteriorRef { 
            parent: PhantomData, 
            state: Cell::new(0), 
            value: UnsafeCell::new(MaybeUninit::uninit()),
        }
    }
}

impl<T, I> InteriorRef<T, I> {
    fn set<'a>(&self, owner: &'a MoogCell<T>, value: &'a I) {
        self.state.replace(owner.state.get());
        let val = MaybeUninit::new(value as *const I);
        unsafe{*self.value.get() = val;}
    }

    pub(crate) fn extract<'a>(&self, owner: &'a MoogCell<T>) -> Option<&'a I> {
        if self.state.get() == owner.state.get() {
            return Some(unsafe { &*(*self.value.get()).assume_init() })
        }
        return None
    }

    pub(crate) fn get_or_compute<'a>(&self, owner: &'a MoogCell<T>, compute: impl FnOnce() -> &'a I) -> &'a I {
        match self.extract(owner) {
            Some(x) => x,
            None => {
                let value = compute();
                self.set(owner, value);
                value
            }
        }
    }
}
