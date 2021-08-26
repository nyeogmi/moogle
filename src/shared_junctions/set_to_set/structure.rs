use crate::keybound::Id;

use super::{Fwd, Bwd};

use crate::raw_junctions::set_to_set::RawSetToSet;
use super::super::moogcell::MoogCell;

// == Data structure ==
pub struct SetToSet<A: Id, B: Id> {
    pub(in crate::shared_junctions) raw: MoogCell<RawSetToSet<A, B>>
}

// == Constructor et al ==
impl<A: Id, B: Id> SetToSet<A, B> {
    pub fn new() -> SetToSet<A, B> {
        SetToSet { raw: MoogCell::new(RawSetToSet::new()) }
    }

    pub fn fwd(&self) -> Fwd<A, B> { Fwd { me: self } }
    pub fn bwd(&self) -> Bwd<A, B> { Bwd { me: self } }
}