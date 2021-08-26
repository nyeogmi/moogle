use crate::keybound::Id;

use super::{Fwd, Bwd};

use crate::raw_junctions::set_to_set::RawSetToSet;
use super::super::moogcell::MoogCell;

// == Data structure ==
pub struct SharedSetToSet<A: Id, B: Id> {
    pub(super) raw: MoogCell<RawSetToSet<A, B>>
}

// == Constructor et al ==
impl<A: Id, B: Id> SharedSetToSet<A, B> {
    pub fn new() -> SharedSetToSet<A, B> {
        SharedSetToSet { raw: MoogCell::new(RawSetToSet::new()) }
    }

    pub fn fwd(&self) -> Fwd<A, B> { Fwd { me: self } }
    pub fn bwd(&self) -> Bwd<A, B> { Bwd { me: self } }
}