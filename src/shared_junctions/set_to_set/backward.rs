use super::SharedSetToSet;

use crate::keybound::Id;

use crate::methods::{SharedAnyToSet, SharedSet};
use crate::methods::{ViewAnyToSet, AnyToSet};
use crate::methods::{ViewSet, Set, EvictSet};

use crate::junctions::set_to_set::SetToSet;

use std::collections::BTreeSet;

use super::super::range_utils;
use super::super::moogcell::{InteriorSetRange, InteriorTreeRange, InteriorVSet};

use crate::structures::VSet;

pub struct Bwd<'a, A: Id, B: Id> { pub(super) me: &'a SharedSetToSet<A, B> }
pub struct BwdSet<'a, A: Id, B: Id> { 
    parent: &'a SharedSetToSet<A, B>, 
    cache: InteriorVSet<SetToSet<A, B>, B, A>,
    key: B 
}