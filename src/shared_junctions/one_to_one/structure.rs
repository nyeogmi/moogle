use crate::id::IdLike;

use super::{Fwd, Bwd};

use crate::raw_junctions::one_to_one::RawOneToOne;
use crate::moogcell::MoogCell;

// == Data structure ==
pub struct OneToOne<A: IdLike, B: IdLike> {
    pub(in crate::shared_junctions) raw: MoogCell<RawOneToOne<A, B>>
}

// == Constructor et al ==
impl<A: IdLike, B: IdLike> OneToOne<A, B> {
    pub fn new() -> OneToOne<A, B> {
        OneToOne { raw: MoogCell::new(RawOneToOne::new()) }
    }

    pub fn fwd(&self) -> Fwd<A, B> { Fwd { me: self } }
    pub fn bwd(&self) -> Bwd<A, B> { Bwd { me: self } }
}