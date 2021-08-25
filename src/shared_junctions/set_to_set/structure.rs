use crate::keybound::Id;

use super::{Fwd, Bwd};

use crate::junctions::set_to_set::SetToSet;
use super::super::moogcell::MoogCell;

// == Data structure ==
pub struct SharedSetToSet<A: Id, B: Id> {
    pub(super) raw: MoogCell<SetToSet<A, B>>
}

// == Constructor et al ==
impl<A: Id, B: Id> SharedSetToSet<A, B> {
    pub fn new() -> SharedSetToSet<A, B> {
        SharedSetToSet { raw: MoogCell::new(SetToSet::new()) }
    }

    pub fn fwd(&self) -> Fwd<A, B> { Fwd { me: self } }
    pub fn bwd(&self) -> Bwd<A, B> { Bwd { me: self } }
}