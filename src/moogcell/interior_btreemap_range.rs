use std::cell::Cell;
use std::mem::MaybeUninit;
use std::collections::btree_map;

use crate::id::IdLike;

use super::MoogCell;

pub struct InteriorBTreeMapRange<'a, T, K: IdLike, V: 'a> {
    owner: &'a MoogCell<T>,
    state: Cell<u64>, 

    // note: this is safe because Range is not Drop
    value: MaybeUninit<btree_map::Range<'a, K, V>>, 
}

impl<'a, T, K: IdLike, V: 'static> Clone for InteriorBTreeMapRange<'a, T, K, V> {
    fn clone(&self) -> Self { 
        InteriorBTreeMapRange {
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
    pub fn create_interior_btreemap_range<'a, K: IdLike, V: 'a>(&'a self) -> InteriorBTreeMapRange<'a, T, K, V> { 
        InteriorBTreeMapRange { 
            owner: self, 
            state: Cell::new(0), 
            value: MaybeUninit::uninit(),
        }
    }
}

impl<'a, T, K: IdLike, V: 'a> InteriorBTreeMapRange<'a, T, K, V> {
    pub(crate) fn get_or_compute_arg(
        &mut self, 
        compute: impl for<'b> FnOnce(&'b T) -> btree_map::Range<'b, K, V>
    ) -> &mut btree_map::Range<'a, K, V> {
        // panic if someone else borrowed our owner
        // (which would imply there is a &mut to it somewhere)
        { self.owner.borrow_mut(); }

        // ok let's go!
        let og = self.owner.state.get();
        if self.state.get() != og {
            self.state.replace(og);

            let borrow = self.owner.borrow();
            let value: btree_map::Range<'_, K, V> = compute(&borrow);
            let longer_value: btree_map::Range<'a, K, V> = unsafe { std::mem::transmute(value) };
            self.value = MaybeUninit::new(longer_value);
        }

        let new_ptr: &mut btree_map::Range<'a, K, V> = unsafe { self.value.assume_init_mut() };
        new_ptr
    }
}