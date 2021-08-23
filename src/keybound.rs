pub trait Id: Copy+Ord+PartialEq+'static {

}

// TODO: Others?
impl Id for char {}

impl Id for usize {}
impl Id for u64 {}
impl Id for u32 {}
impl Id for u16 {}
impl Id for u8 {}

impl Id for isize {}
impl Id for i64 {}
impl Id for i32 {}
impl Id for i16 {}
impl Id for i8 {}