use super::super::interfaces::{ViewMapLike, MapLike};
use std::collections::BTreeMap;

// == Data structure ==
struct OneToOne<A: Ord, B: Ord> {
    fwd: BTreeMap<A, B>,
    bwd: BTreeMap<B, A>,
}

struct MFwd<'a, A: Ord, B: Ord>(&'a mut OneToOne<A, B>);
struct MBwd<'a, A: Ord, B: Ord>(&'a mut OneToOne<A, B>);

struct VFwd<'a, A: Ord, B: Ord>(&'a OneToOne<A, B>);
struct VBwd<'a, A: Ord, B: Ord>(&'a OneToOne<A, B>);

// == Accessors ==
impl<A: Ord, B: Ord> OneToOne<A, B> {
    pub fn fwd(&self) -> VFwd<A, B> { VFwd(self) }
    pub fn bwd(&self) -> VBwd<A, B> { VBwd(self) }
} 

impl<A: Ord+Clone, B: Ord+Clone> OneToOne<A, B> {
    pub fn mut_fwd(&mut self) -> MFwd<A, B> { MFwd(self) }
    pub fn mut_bwd(&mut self) -> MBwd<A, B> { MBwd(self) }
} 

// == Forward ==
impl<'a, A: Ord+Clone, B: Ord+Clone> MapLike<'a, A, B> for MFwd<'a, A, B> {
    fn insert(&mut self, a: A, b: B) -> Option<B> { 
        let old_ab = self.0.fwd.insert(a.clone(), b.clone());
        let old_b = match old_ab {
            Some(b) => { self.0.bwd.remove(&b); Some(b) }
            None => { None }
        };
        self.0.bwd.insert(b, a);
        old_b
     }
    fn remove(&mut self, a: &A) -> std::option::Option<B> { 
        match self.0.fwd.remove_entry(a) {
            Some((_, b)) => { self.0.bwd.remove(&b); Some(b) }
            None => None
        }
    }
}

impl<'a, A: Ord, B: Ord> ViewMapLike<'a, A, B> for MFwd<'a, A, B> {
    fn get(&self, a: &A) -> std::option::Option<&B> { self.0.fwd.get(a) }
    fn contains_key(&self, a: &A) -> bool { self.0.fwd.contains_key(a) }
    fn len(&self) -> usize { self.0.fwd.len() }
}

impl<'a, A: Ord, B: Ord> ViewMapLike<'a, A, B> for VFwd<'a, A, B> {
    fn get(&self, a: &A) -> std::option::Option<&B> { self.0.fwd.get(a) }
    fn contains_key(&self, a: &A) -> bool { self.0.fwd.contains_key(a) }
    fn len(&self) -> usize { self.0.fwd.len() }
}

// == Backward ==
impl<'a, A: Ord+Clone, B: Ord+Clone> MapLike<'a, B, A> for MBwd<'a, A, B> {
    fn insert(&mut self, b: B, a: A) -> Option<A> { 
        let old_ba = self.0.bwd.insert(b.clone(), a.clone());
        let old_a = match old_ba {
            Some(a) => { self.0.fwd.remove(&a); Some(a) }
            None => { None }
        };
        self.0.fwd.insert(a, b);
        old_a
     }
    fn remove(&mut self, b: &B) -> std::option::Option<A> { 
        match self.0.bwd.remove_entry(b) {
            Some((_, a)) => { self.0.fwd.remove(&a); Some(a) }
            None => None
        }
    }
}

impl<'a, A: Ord, B: Ord> ViewMapLike<'a, B, A> for MBwd<'a, A, B> {
    fn get(&self, b: &B) -> std::option::Option<&A> { self.0.bwd.get(b) }
    fn contains_key(&self, b: &B) -> bool { self.0.bwd.contains_key(b) }
    fn len(&self) -> usize { self.0.bwd.len() }
}

impl<'a, A: Ord, B: Ord> ViewMapLike<'a, B, A> for VBwd<'a, A, B> {
    fn get(&self, b: &B) -> std::option::Option<&A> { self.0.bwd.get(b) }
    fn contains_key(&self, b: &B) -> bool { self.0.bwd.contains_key(b) }
    fn len(&self) -> usize { self.0.bwd.len() }
}

// == TODO: Re-export setlike views of VFwd et al ==