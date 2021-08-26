use crate::keybound::Id;

use super::{Fwd, Bwd};

use crate::raw_junctions::one_to_one::RawOneToOne;
use super::super::moogcell::MoogCell;

// == Data structure ==
pub struct OneToOne<A: Id, B: Id> {
    pub(super) raw: MoogCell<RawOneToOne<A, B>>
}

// == Constructor et al ==
impl<A: Id, B: Id> OneToOne<A, B> {
    pub fn new() -> OneToOne<A, B> {
        OneToOne { raw: MoogCell::new(RawOneToOne::new()) }
    }

    pub fn fwd(&self) -> Fwd<A, B> { Fwd { me: self } }
    pub fn bwd(&self) -> Bwd<A, B> { Bwd { me: self } }
}