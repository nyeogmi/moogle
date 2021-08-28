use std::cell::{Cell, RefCell, Ref, RefMut};


pub struct MoogCell<T> {
    pub(in crate::moogcell) state: Cell<u64>,
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
        self.mark_dirty();
        self.inner.borrow_mut()
    }

    pub fn get_exclusive(&mut self) -> &T {
        // don't mark dirty because we don't let you mutate anything with the ref
        self.inner.get_mut()
    }

    pub fn get_mut(&mut self) -> &mut T {
        self.mark_dirty();
        self.inner.get_mut()
    }
}