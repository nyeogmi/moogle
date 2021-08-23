pub trait Id: Copy+Ord+PartialEq+'static {

}

// TODO: Others?
impl Id for usize {}