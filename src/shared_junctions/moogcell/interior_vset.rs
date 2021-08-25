use std::cell::Cell;
use std::mem::MaybeUninit;
use crate::structures::VSet;

use crate::keybound::Id;

use super::MoogCell;

pub struct InteriorVSet<'a, T, K: Id, V: Id> {
    owner: &'a MoogCell<T>,
    state: Cell<u64>, 

    value: Cell<MaybeUninit<VSet<'static, K, V>>>,
}

impl<'a, T, K: Id, V: Id> Clone for InteriorVSet<'a, T, K, V> {
    fn clone(&self) -> Self { 
        InteriorVSet {
            owner: self.owner,
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
    pub fn create_interior_vset<K: Id, V: Id>(&self) -> InteriorVSet<'_, T, K, V> { 
        InteriorVSet { 
            owner: self, 
            state: Cell::new(0), 
            value: Cell::new(MaybeUninit::uninit()),
        }
    }
}

impl<'a, T, K: Id, V: Id> InteriorVSet<'a, T, K, V> {
    pub(crate) fn get_or_compute(
        &self, 
        compute: impl for<'b> FnOnce(&'b T) -> VSet<'b, K, V>
    ) -> VSet<'a, K, V> {
        let og = self.owner.state.get();
        if self.state.get() != og {
            self.state.replace(og);

            let borrow = self.owner.borrow();
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