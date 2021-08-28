use std::cell::Cell;
use std::mem::MaybeUninit;
use std::collections::btree_set;

use crate::id::IdLike;

use super::MoogCell;

pub struct InteriorSetRange<'a, T, K: IdLike> {
    owner: &'a MoogCell<T>,
    state: Cell<u64>, 

    // note: this is safe because Range is not Drop
    value: MaybeUninit<btree_set::Range<'static, K>>,
}

impl<'a, T, K: IdLike> Clone for InteriorSetRange<'a, T, K> {
    fn clone(&self) -> Self { 
        InteriorSetRange {
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
    pub fn create_interior_set_range<K: IdLike>(&self) -> InteriorSetRange<'_, T, K> { 
        InteriorSetRange { 
            owner: self, 
            state: Cell::new(0), 
            value: MaybeUninit::uninit()
         }
    }
}

impl<'a, T, K: IdLike> InteriorSetRange<'a, T, K> {
    pub(crate) fn get_or_compute(
        &mut self, 
        compute: impl FnOnce() -> btree_set::Range<'a, K>
    ) -> &mut btree_set::Range<'a, K> {
        let og = self.owner.state.get();
        if self.state.get() != og {
            self.state.replace(og);

            let value: btree_set::Range<'a, K> = compute();
            let static_value: btree_set::Range<'static, K> = unsafe { std::mem::transmute(value) };
            self.value = MaybeUninit::new(static_value);
        }

        let old_ptr: &mut btree_set::Range<'static, K> = unsafe { self.value.assume_init_mut() };
        let new_ptr: &mut btree_set::Range<'a, K> = unsafe { std::mem::transmute(old_ptr) };

        new_ptr
    }
}