use std::cell::Cell;
use std::mem::MaybeUninit;
use std::marker::PhantomData;
use crate::structures::VSet;

use crate::keybound::Id;

use super::MoogCell;

pub struct InteriorVSet<T, K: Id, V: Id> {
    parent: PhantomData<*const T>,
    state: Cell<u64>, 

    value: Cell<MaybeUninit<VSet<'static, K, V>>>,
}

impl<T, K: Id, V: Id> Clone for InteriorVSet<T, K, V> {
    fn clone(&self) -> Self { 
        InteriorVSet {
            parent: self.parent,
            state: self.state.clone(),
            value: Cell::new(
                if self.state.get() == 0 {
                    MaybeUninit::uninit()
                } else {
                    MaybeUninit::new(unsafe {self.value.get().assume_init_ref()}.clone())
                }
            )
        }
    }
}

impl<T> MoogCell<T> {
    pub fn create_interior_vset<K: Id, V: Id>(&self) -> InteriorVSet<T, K, V> { 
        InteriorVSet { 
            parent: PhantomData, 
            state: Cell::new(0), 
            value: Cell::new(MaybeUninit::uninit()),
        }
    }
}

impl<T, K: Id, V: Id> InteriorVSet<T, K, V> {
    pub(crate) fn get_or_compute<'a>(
        &self, 
        owner: &'a MoogCell<T>, 
        compute: impl for<'b> FnOnce(&'b T) -> VSet<'b, K, V>
    ) -> VSet<'a, K, V> {
        if self.state.get() != owner.state.get() {
            self.state.replace(owner.state.get());

            let borrow = owner.borrow();
            let value: VSet<'a, K, V> = {
                let borrow_ref: &T = &borrow;
                let long_ref: &'a T = unsafe { std::mem::transmute(borrow_ref) };
                compute(long_ref)
            };

            let static_value: VSet<'static, K, V> = unsafe { std::mem::transmute(value) };
            self.value.replace(MaybeUninit::new(static_value));
        }

        let old_ptr: VSet<'static, K, V> = unsafe { self.value.get().assume_init() };
        let new_ptr: VSet<'a, K, V> = unsafe { std::mem::transmute(old_ptr) };

        new_ptr.clone()
    }
}