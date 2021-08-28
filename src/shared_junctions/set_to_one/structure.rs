use crate::id::IdLike;

use super::{Fwd, Bwd};

use crate::raw_junctions::set_to_one::RawSetToOne;
use crate::moogcell::MoogCell;

// == Data structure ==
pub struct SetToOne<A: IdLike, B: IdLike> {
    pub(in crate::shared_junctions) raw: MoogCell<RawSetToOne<A, B>>
}

// == Constructor et al ==
impl<A: IdLike, B: IdLike> SetToOne<A, B> {
    pub fn new() -> SetToOne<A, B> {
        SetToOne { raw: MoogCell::new(RawSetToOne::new()) }
    }

    pub fn raw(&mut self) -> &mut RawSetToOne<A, B> { self.raw.get_mut() }

    pub fn fwd(&self) -> Fwd<A, B> { Fwd { me: self } }
    pub fn bwd(&self) -> Bwd<A, B> { Bwd { me: self } }
}