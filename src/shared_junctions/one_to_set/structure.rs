use crate::keybound::Id;

use super::{Fwd, Bwd};

use crate::raw_junctions::one_to_set::RawOneToSet;
use super::super::moogcell::MoogCell;

// == Data structure ==
pub struct SharedOneToSet<A: Id, B: Id> {
    pub(super) raw: MoogCell<RawOneToSet<A, B>>
}

// == Constructor et al ==
impl<A: Id, B: Id> SharedOneToSet<A, B> {
    pub fn new() -> SharedOneToSet<A, B> {
        SharedOneToSet { raw: MoogCell::new(RawOneToSet::new()) }
    }

    pub fn fwd(&self) -> Fwd<A, B> { Fwd { me: self } }
    pub fn bwd(&self) -> Bwd<A, B> { Bwd { me: self } }
}