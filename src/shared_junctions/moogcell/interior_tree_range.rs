use std::cell::Cell;
use std::mem::MaybeUninit;
use std::collections::btree_map;

use crate::keybound::Id;

use super::MoogCell;

pub struct InteriorTreeRange<'a, T, K: Id, V: 'static> {
    owner: &'a MoogCell<T>,
    state: Cell<u64>, 

    // note: this is safe because Range is not Drop
    value: MaybeUninit<btree_map::Range<'static, K, V>>, 
}

impl<'a, T, K: Id, V: 'static> Clone for InteriorTreeRange<'a, T, K, V> {
    fn clone(&self) -> Self { 
        InteriorTreeRange {
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
    pub fn create_interior_tree_range<K: Id, V: 'static>(&self) -> InteriorTreeRange<'_, T, K, V> { 
        InteriorTreeRange { 
            owner: self, 
            state: Cell::new(0), 
            value: MaybeUninit::uninit(),
        }
    }
}

impl<'a, T, K: Id, V: 'static> InteriorTreeRange<'a, T, K, V> {
    pub(crate) fn get_or_compute(
        &mut self, 
        compute: impl for<'b> FnOnce(&'b T) -> btree_map::Range<'b, K, V>
    ) -> &mut btree_map::Range<'a, K, V> {
        let og = self.owner.state.get();
        if self.state.get() != og {
            self.state.replace(og);

            let borrow = self.owner.borrow();
            let value: btree_map::Range<'a, K, V> = {
                let borrow_ref: &T = &borrow;
                let long_ref: &'a T = unsafe { std::mem::transmute(borrow_ref) };
                compute(long_ref)
            };

            let static_value: btree_map::Range<'static, K, V> = unsafe { std::mem::transmute(value) };
            self.value = MaybeUninit::new(static_value);
        }

        let old_ptr: &mut btree_map::Range<'static, K, V> = unsafe { self.value.assume_init_mut() };
        let new_ptr: &mut btree_map::Range<'a, K, V> = unsafe { std::mem::transmute(old_ptr) };

        new_ptr
    }
}