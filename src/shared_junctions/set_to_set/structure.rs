use crate::id::IdLike;

use super::{Fwd, Bwd};

use crate::raw_junctions::set_to_set::RawSetToSet;
use crate::moogcell::MoogCell;

// == Data structure ==
pub struct SetToSet<A: IdLike, B: IdLike> {
    pub(in crate::shared_junctions) raw: MoogCell<RawSetToSet<A, B>>
}

// == Constructor et al ==
impl<A: IdLike, B: IdLike> SetToSet<A, B> {
    pub fn new() -> SetToSet<A, B> {
        SetToSet { raw: MoogCell::new(RawSetToSet::new()) }
    }

    pub fn raw(&mut self) -> &mut RawSetToSet<A, B> { self.raw.get_mut() }

    pub fn fwd(&self) -> Fwd<A, B> { Fwd { me: self } }
    pub fn bwd(&self) -> Bwd<A, B> { Bwd { me: self } }
}