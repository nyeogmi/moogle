use std::cell::{Cell, RefCell, Ref, RefMut};


pub struct MoogCell<T> {
    pub(in crate::shared_junctions::moogcell) state: Cell<u64>,
    inner: RefCell<T>,
}

impl<T> MoogCell<T> {
    pub const fn new(value: T) -> MoogCell<T> {
        MoogCell { state: Cell::new(1), inner: RefCell::new(value) }
    }

    fn mark_dirty(&self) {
        // TODO: Bounds check this?
        self.state.replace(self.state.get() + 1);
    }

    pub fn borrow(&self) -> Ref<'_, T> { self.inner.borrow() }
    pub fn borrow_mut(&self) -> RefMut<'_, T> { 
        let result = self.inner.borrow_mut();
        self.mark_dirty();
        result
    }
}