pub trait Id: Copy+Ord+PartialEq+'static {

}

// TODO: Others?
impl Id for char {}
impl Id for usize {}