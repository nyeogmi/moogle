use std::cell::{Cell, RefCell, Ref, RefMut};


pub struct MoogCell<T> {
    pub(in crate::moogcell) state: Cell<u64>,
    inner: RefCell<T>,
}

impl<T> MoogCell<T> {
    pub const fn new(value: T) -> MoogCell<T> {
        MoogCell { state: Cell::new(1), inner: RefCell::new(value) }
    }

    pub fn into_inner(self) -> T {
        return self.inner.into_inner()
    }


    fn mark_dirty(&self) {
        // TODO: Bounds check this?
        self.state.replace(self.state.get() + 1);
    }

    pub fn borrow(&self) -> Ref<'_, T> { self.inner.borrow() }
    pub fn borrow_mut(&self) -> RefMut<'_, T> { 
        self.mark_dirty();
        self.inner.borrow_mut()
    }

    pub fn get_mut(&mut self) -> &mut T {
        self.mark_dirty();
        self.inner.get_mut()
    }
}