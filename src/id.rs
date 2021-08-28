use std::marker::PhantomData;

pub trait IdLike: Copy+Ord+PartialEq+'static {

}

// TODO: Others?
impl IdLike for char {}

impl IdLike for usize {}
impl IdLike for u64 {}
impl IdLike for u32 {}
impl IdLike for u16 {}
impl IdLike for u8 {}

impl IdLike for isize {}
impl IdLike for i64 {}
impl IdLike for i32 {}
impl IdLike for i16 {}
impl IdLike for i8 {}

// internal ID type, use if you have nothing else!!
pub struct Id<T>(pub(crate) u64, PhantomData<*const T>);

impl<T> Id<T> {
    pub(crate) fn new(val: u64) -> Self {
        Id(val, PhantomData)
    }

}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self { *self }
}

impl<T> Copy for Id<T> {

}

impl<T> std::fmt::Debug for Id<T> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        fmt.debug_tuple(std::any::type_name::<Id<T>>()).field(&self.0).finish()
    }
}

impl<T> PartialEq for Id<T> {
    fn eq(&self, id2: &Id<T>) -> bool {
        self.0 == id2.0
    } 
}

impl<T> Eq for Id<T> {

}

impl<T> PartialOrd for Id<T> {
    fn partial_cmp(&self, id2: &Id<T>) -> std::option::Option<std::cmp::Ordering> { 
        self.0.partial_cmp(&id2.0)
    }
}

impl<T> Ord for Id<T> {
    fn cmp(&self, id2: &Self) -> std::cmp::Ordering {
        self.0.cmp(&id2.0)
    } 
}

impl<T: 'static> IdLike for Id<T> {

}