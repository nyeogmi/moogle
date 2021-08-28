use std::cell::Cell;
use std::mem::MaybeUninit;
use crate::structures::VSet;

use crate::id::IdLike;

use super::MoogCell;

pub struct InteriorVSet<'a, T, K: IdLike, V: IdLike> {
    owner: &'a MoogCell<T>,
    state: Cell<u64>, 

    value: Cell<MaybeUninit<VSet<'static, K, V>>>,
}

impl<'a, T, K: IdLike, V: IdLike> Clone for InteriorVSet<'a, T, K, V> {
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
    pub fn create_interior_vset<K: IdLike, V: IdLike>(&self) -> InteriorVSet<'_, T, K, V> { 
        InteriorVSet { 
            owner: self, 
            state: Cell::new(0), 
            value: Cell::new(MaybeUninit::uninit()),
        }
    }
}

impl<'a, T, K: IdLike, V: IdLike> InteriorVSet<'a, T, K, V> {
    pub(crate) fn get_or_compute_arg(
        &self, 
        compute: impl for<'b> FnOnce(&'b T) -> VSet<'b, K, V>
    ) -> VSet<'a, K, V> {
        let og = self.owner.state.get();
        if self.state.get() != og {
            self.state.replace(og);

            let borrow = self.owner.borrow();
            let value: VSet<'_, K, V> = compute(&borrow);
            let static_value: VSet<'static, K, V> = unsafe { value.unsafe_transmute_lifetime() };
            self.value.replace(MaybeUninit::new(static_value));
        }

        let old_ptr: VSet<'static, K, V> = unsafe { self.value.get().assume_init() };
        let new_ptr: VSet<'a, K, V> = unsafe { old_ptr.unsafe_transmute_lifetime() };

        new_ptr.clone()
    }
}