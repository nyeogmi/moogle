use crate::keybound::Id;

use super::{Fwd, Bwd};

use crate::raw_junctions::one_to_set::RawOneToSet;
use super::super::moogcell::MoogCell;

// == Data structure ==
pub struct OneToSet<A: Id, B: Id> {
    pub(super) raw: MoogCell<RawOneToSet<A, B>>
}

// == Constructor et al ==
impl<A: Id, B: Id> OneToSet<A, B> {
    pub fn new() -> OneToSet<A, B> {
        OneToSet { raw: MoogCell::new(RawOneToSet::new()) }
    }

    pub fn fwd(&self) -> Fwd<A, B> { Fwd { me: self } }
    pub fn bwd(&self) -> Bwd<A, B> { Bwd { me: self } }
}