use std::cell::Cell;
use std::mem::MaybeUninit;
use std::marker::PhantomData;
use std::collections::btree_map;

use crate::keybound::Id;

use super::MoogCell;

pub struct InteriorTreeRange<T, K: Id, V: 'static> {
    parent: PhantomData<*const T>,
    state: Cell<u64>, 

    // note: this is safe because Range is not Drop
    value: MaybeUninit<btree_map::Range<'static, K, V>>, 
}

impl<T, K: Id, V: 'static> Clone for InteriorTreeRange<T, K, V> {
    fn clone(&self) -> Self { 
        InteriorTreeRange {
            parent: self.parent,
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
    pub fn create_interior_tree_range<K: Id, V: 'static>(&self) -> InteriorTreeRange<T, K, V> { 
        InteriorTreeRange { 
            parent: PhantomData, 
            state: Cell::new(0), 
            value: MaybeUninit::uninit(),
        }
    }
}

impl<T, K: Id, V: 'static> InteriorTreeRange<T, K, V> {
    pub(crate) fn get_or_compute<'a>(
        &mut self, 
        owner: &'a MoogCell<T>, 
        compute: impl for<'b> FnOnce(&'b T) -> btree_map::Range<'b, K, V>
    ) -> &mut btree_map::Range<'a, K, V> {
        if self.state.get() != owner.state.get() {
            self.state.replace(owner.state.get());

            let borrow = owner.borrow();
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