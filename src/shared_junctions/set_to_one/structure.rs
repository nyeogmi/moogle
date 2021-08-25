use crate::keybound::Id;

use super::{Fwd, Bwd};

use crate::junctions::set_to_one::SetToOne;
use super::super::moogcell::MoogCell;

// == Data structure ==
pub struct SharedSetToOne<A: Id, B: Id> {
    pub(super) raw: MoogCell<SetToOne<A, B>>
}

// == Constructor et al ==
impl<A: Id, B: Id> SharedSetToOne<A, B> {
    pub fn new() -> SharedSetToOne<A, B> {
        SharedSetToOne { raw: MoogCell::new(SetToOne::new()) }
    }

    pub fn fwd(&self) -> Fwd<A, B> { Fwd { me: self } }
    pub fn bwd(&self) -> Bwd<A, B> { Bwd { me: self } }
}