use std::cell::Cell;
use std::mem::MaybeUninit;
use std::collections::btree_set;

use crate::id::IdLike;

use super::MoogCell;

pub struct InteriorTupSetRange<'a, T, K: IdLike, V: IdLike> {
    owner: &'a MoogCell<T>,
    state: Cell<u64>, 

    // note: this is safe because Range is not Drop
    value: MaybeUninit<btree_set::Range<'static, (K, V)>>,
}

impl<'a, T, K: IdLike, V: IdLike> Clone for InteriorTupSetRange<'a, T, K, V> {
    fn clone(&self) -> Self { 
        InteriorTupSetRange {
            owner: self.owner,
            state: self.state.clone(),
            value: 
                if self.state.get() == 0 {
                    MaybeUninit::uninit()
                } else {
                    MaybeUninit::new(unsafe {self.value.assume_init_ref()}.clone())
                }
        }
    }
}

impl<T> MoogCell<T> {
    pub fn create_interior_tupset_range<K: IdLike, V: IdLike>(&self) -> InteriorTupSetRange<'_, T, K, V> { 
        InteriorTupSetRange { 
            owner: self, 
            state: Cell::new(0), 
            value: MaybeUninit::uninit()
         }
    }
}

impl<'a, T, K: IdLike, V: IdLike> InteriorTupSetRange<'a, T, K, V> {
    pub(crate) fn get_or_compute(
        &mut self, 
        compute: impl FnOnce() -> btree_set::Range<'a, (K, V)>
    ) -> &mut btree_set::Range<'a, (K, V)> {
        let og = self.owner.state.get();
        if self.state.get() != og {
            self.state.replace(og);

            let value: btree_set::Range<'a, (K, V)> = compute();
            let static_value: btree_set::Range<'static, (K, V)> = unsafe { std::mem::transmute(value) };
            self.value = MaybeUninit::new(static_value);
        }

        let old_ptr: &mut btree_set::Range<'static, (K, V)> = unsafe { self.value.assume_init_mut() };
        let new_ptr: &mut btree_set::Range<'a, (K, V)> = unsafe { std::mem::transmute(old_ptr) };

        new_ptr
    }
}