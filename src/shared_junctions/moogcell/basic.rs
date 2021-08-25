use std::cell::{Cell, RefCell, Ref, RefMut, BorrowError, BorrowMutError};


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

    pub fn into_inner(self) -> T { self.inner.into_inner() }
    pub fn replace(&self, t: T) -> T {
        self.mark_dirty();
        self.inner.replace(t)
    }

    pub fn replace_with(&self, f: impl FnOnce(&mut T) -> T) -> T {
        self.mark_dirty();
        self.inner.replace_with(f)
    }

    pub fn swap(&self, other: &MoogCell<T>) {
        self.mark_dirty();
        other.mark_dirty();
        self.inner.swap(&other.inner);
    }

    pub fn borrow(&self) -> Ref<'_, T> { self.inner.borrow() }
    pub fn try_borrow(&self) -> Result<Ref<'_, T>, BorrowError> { self.inner.try_borrow() }
    pub fn borrow_mut(&self) -> RefMut<'_, T> { 
        let result = self.inner.borrow_mut();
        self.mark_dirty();
        result
    }
    pub fn try_borrow_mut(&self) -> Result<RefMut<'_, T>, BorrowMutError> { 
        let result = self.inner.try_borrow_mut()?;
        self.mark_dirty();
        Ok(result)
    }

    // TODO: Expose as_ptr?

    pub fn get_mut(&mut self) -> &mut T {
        self.mark_dirty();
        self.inner.get_mut()
    }
}