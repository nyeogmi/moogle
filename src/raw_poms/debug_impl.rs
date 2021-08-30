use std::fmt::{Debug, Formatter};
use std::fmt;

impl<T: Debug> Debug for super::RawPom<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.iter().map(|(k, v)| (k.0, v))).finish()
    }
}