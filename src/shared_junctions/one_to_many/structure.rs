use crate::id::IdLike;

use super::{Fwd, Bwd};

use crate::raw_junctions::one_to_many::RawOneToMany;
use crate::moogcell::MoogCell;

// == Data structure ==
pub struct OneToMany<A: IdLike, B: IdLike> {
    pub(in crate::shared_junctions) raw: MoogCell<RawOneToMany<A, B>>
}

// == Constructor et al ==
impl<A: IdLike, B: IdLike> OneToMany<A, B> {
    pub fn new() -> OneToMany<A, B> {
        OneToMany { raw: MoogCell::new(RawOneToMany::new()) }
    }

    pub fn raw(&mut self) -> &mut RawOneToMany<A, B> { self.raw.get_mut() }

    pub fn fwd(&self) -> Fwd<A, B> { Fwd { me: self } }
    pub fn bwd(&self) -> Bwd<A, B> { Bwd { me: self } }
}