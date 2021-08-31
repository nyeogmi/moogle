use crate::id::IdLike;

use super::{Fwd, Bwd};

use crate::raw_junctions::many_to_many::RawManyToMany;
use crate::moogcell::MoogCell;

// == Data structure ==
pub struct ManyToMany<A: IdLike, B: IdLike> {
    pub(in crate::shared_junctions) raw: MoogCell<RawManyToMany<A, B>>
}

// == Constructor et al ==
impl<A: IdLike, B: IdLike> ManyToMany<A, B> {
    pub fn new() -> ManyToMany<A, B> {
        ManyToMany { raw: MoogCell::new(RawManyToMany::new()) }
    }

    pub fn raw(&mut self) -> &mut RawManyToMany<A, B> { self.raw.get_mut() }

    pub fn fwd(&self) -> Fwd<A, B> { Fwd { me: self } }
    pub fn bwd(&self) -> Bwd<A, B> { Bwd { me: self } }
}