use crate::id::IdLike;

use super::{Fwd, Bwd};

use crate::raw_junctions::many_to_one::RawManyToOne;
use crate::moogcell::MoogCell;

// == Data structure ==
pub struct ManyToOne<A: IdLike, B: IdLike> {
    pub(in crate::shared_junctions) raw: MoogCell<RawManyToOne<A, B>>
}

// == Constructor et al ==
impl<A: IdLike, B: IdLike> ManyToOne<A, B> {
    pub fn new() -> ManyToOne<A, B> {
        ManyToOne { raw: MoogCell::new(RawManyToOne::new()) }
    }

    pub fn raw(&mut self) -> &mut RawManyToOne<A, B> { self.raw.get_mut() }

    pub fn fwd(&self) -> Fwd<A, B> { Fwd { me: self } }
    pub fn bwd(&self) -> Bwd<A, B> { Bwd { me: self } }
}