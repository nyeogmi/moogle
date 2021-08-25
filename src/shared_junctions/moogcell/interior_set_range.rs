use std::cell::Cell;
use std::mem::MaybeUninit;
use std::marker::PhantomData;
use std::collections::btree_set;

use crate::keybound::Id;

use super::MoogCell;

pub struct InteriorSetRange<T, K: Id> {
    parent: PhantomData<*const T>,
    state: Cell<u64>, 

    // note: this is safe because Range is not Drop
    value: MaybeUninit<btree_set::Range<'static, K>>,
}

impl<T, K: Id> Clone for InteriorSetRange<T, K> {
    fn clone(&self) -> Self { 
        InteriorSetRange {
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
    pub fn create_interior_set_range<K: Id>(&self) -> InteriorSetRange<T, K> { 
        InteriorSetRange { 
            parent: PhantomData, 
            state: Cell::new(0), 
            value: MaybeUninit::uninit()
         }
    }
}

impl<T, K: Id> InteriorSetRange<T, K> {
    pub(crate) fn get_or_compute<'a>(
        &mut self, 
        owner: &'a MoogCell<T>, 
        compute: impl FnOnce() -> btree_set::Range<'a, K>
    ) -> &mut btree_set::Range<'a, K> {
        if self.state.get() != owner.state.get() {
            self.state.replace(owner.state.get());

            let value: btree_set::Range<'a, K> = compute();

            let static_value: btree_set::Range<'static, K> = unsafe { std::mem::transmute(value) };
            self.value = MaybeUninit::new(static_value);
        }

        let old_ptr: &mut btree_set::Range<'static, K> = unsafe { self.value.assume_init_mut() };
        let new_ptr: &mut btree_set::Range<'a, K> = unsafe { std::mem::transmute(old_ptr) };

        new_ptr
    }
}