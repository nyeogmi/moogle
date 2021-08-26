use crate::keybound::Id;

use super::{Fwd, Bwd};

use crate::raw_junctions::set_to_one::RawSetToOne;
use super::super::moogcell::MoogCell;

// == Data structure ==
pub struct SetToOne<A: Id, B: Id> {
    pub(super) raw: MoogCell<RawSetToOne<A, B>>
}

// == Constructor et al ==
impl<A: Id, B: Id> SetToOne<A, B> {
    pub fn new() -> SetToOne<A, B> {
        SetToOne { raw: MoogCell::new(RawSetToOne::new()) }
    }

    pub fn fwd(&self) -> Fwd<A, B> { Fwd { me: self } }
    pub fn bwd(&self) -> Bwd<A, B> { Bwd { me: self } }
}