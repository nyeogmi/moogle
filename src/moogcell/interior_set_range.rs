use std::cell::Cell;
use std::mem::MaybeUninit;
use std::collections::btree_set;

use super::MoogCell;

pub struct InteriorSetRange<'a, T, Item: Copy+'a> {
    owner: &'a MoogCell<T>,
    state: Cell<u64>, 

    // note: this is safe because Range is not Drop
    value: MaybeUninit<btree_set::Range<'a, Item>>,
}

impl<'a, T, Item: Copy+'a> Clone for InteriorSetRange<'a, T, Item> {
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
    pub fn create_interior_btreeset_range<'a, Item: Copy+'a>(&'a self) -> InteriorSetRange<'a, T, Item> { 
        InteriorSetRange { 
            owner: self, 
            state: Cell::new(0), 
            value: MaybeUninit::uninit()
         }
    }
}

impl<'a, T, Item: Copy+'a> InteriorSetRange<'a, T, Item> {
    pub(crate) fn get_or_compute(
        &mut self, 
        compute: impl FnOnce() -> btree_set::Range<'a, Item> 
    ) -> &mut btree_set::Range<'a, Item> {
        let og = self.owner.state.get();
        if self.state.get() != og {
            self.state.replace(og);

            let value: btree_set::Range<'_, Item> = compute();
            let long_value: btree_set::Range<'a, Item> = unsafe { std::mem::transmute(value) };
            self.value = MaybeUninit::new(long_value);
        }

        unsafe { self.value.assume_init_mut() }
    }

    pub(crate) fn get_or_compute_arg(
        &mut self, 
        compute: impl for<'b> FnOnce(&'b T) -> btree_set::Range<'b, Item> 
    ) -> &mut btree_set::Range<'a, Item> {
        let og = self.owner.state.get();
        if self.state.get() != og {
            self.state.replace(og);

            let borrow = self.owner.borrow();
            let value: btree_set::Range<'_, Item> = compute(&borrow);
            let long_value: btree_set::Range<'a, Item> = unsafe { std::mem::transmute(value) };
            self.value = MaybeUninit::new(long_value);
        }

        unsafe { self.value.assume_init_mut() }
    }
}