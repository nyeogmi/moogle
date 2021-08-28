use crate::id::IdLike;

use super::{Fwd, Bwd};

use crate::raw_junctions::one_to_set::RawOneToSet;
use crate::moogcell::MoogCell;

// == Data structure ==
pub struct OneToSet<A: IdLike, B: IdLike> {
    pub(in crate::shared_junctions) raw: MoogCell<RawOneToSet<A, B>>
}

// == Constructor et al ==
impl<A: IdLike, B: IdLike> OneToSet<A, B> {
    pub fn new() -> OneToSet<A, B> {
        OneToSet { raw: MoogCell::new(RawOneToSet::new()) }
    }

    pub fn raw(&mut self) -> &mut RawOneToSet<A, B> { self.raw.get_mut() }

    pub fn fwd(&self) -> Fwd<A, B> { Fwd { me: self } }
    pub fn bwd(&self) -> Bwd<A, B> { Bwd { me: self } }
}