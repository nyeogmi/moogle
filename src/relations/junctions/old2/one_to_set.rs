use super::super::interfaces::{ViewSetLike, SetLike, ViewMapLike, MapLike, ViewMultiMapLike, MultiMapLike};
use super::super::structures::{BTreeMultiMap, MValues, VValues};
use std::collections::{BTreeMap, BTreeSet};

// == Data structure ==
struct OneToSet<A: Ord, B: Ord> {
    fwd: BTreeMultiMap<A, B>,
    bwd: BTreeMap<B, A>,
}

struct MFwd<'a, A: Ord, B: Ord>(&'a mut OneToSet<A, B>);
struct MFwdSet<'a, A: Ord, B: Ord>(MValues<'a, A, B>, &'a mut BTreeMap<B, A>);
struct MBwd<'a, A: Ord, B: Ord>(&'a mut OneToSet<A, B>);

struct VFwd<'a, A: Ord, B: Ord>(&'a OneToSet<A, B>);
struct VFwdSet<'a, B: Ord>(VValues<'a, B>);
struct VBwd<'a, A: Ord, B: Ord>(&'a OneToSet<A, B>);

// == Accessors ==
impl<A: Ord, B: Ord> OneToSet<A, B> {
    pub fn fwd(&self) -> VFwd<A, B> { VFwd(self) }
    pub fn bwd(&self) -> VBwd<A, B> { VBwd(self) }
} 

impl<A: Ord+Clone, B: Ord+Clone> OneToSet<A, B> {
    pub fn mut_fwd(&mut self) -> MFwd<A, B> { MFwd(self) }
    pub fn mut_bwd(&mut self) -> MBwd<A, B> { MBwd(self) }
} 

// == Forward ==
impl<'a, A: Ord+Clone, B: Ord+Clone> MultiMapLike<'a, A, B> for MFwd<'a, A, B> {
    type MMulti = MFwdSet<'a, A, B>;
    type MExpunge = BTreeSet<B>;

    fn get_mut(&mut self, a: A) -> MFwdSet<'_, A, B> { 
        return MFwdSet(self.0.fwd.get_mut(a), &mut self.0.bwd);
    }

    fn insert(&mut self, a: A, b: B) {
        self.0.fwd.insert(a.clone(), b.clone());
        self.0.bwd.insert(b, a);
     }
    fn remove(&mut self, a: &A) -> Self::MExpunge {
        let bs = self.0.fwd.remove(a);
        for b in bs.iter() {
            self.0.bwd.remove(b);
        }
        return bs;
    }
}

impl<'a, A: Ord, B: Ord> ViewMultiMapLike<'a, A, B> for MFwd<'a, A, B> {
    type VMulti = VFwdSet<'a, B>;

    fn get(&'a self, a: &A) -> VFwdSet<'_, B> { return VFwdSet(self.0.fwd.get(a)) }
    fn contains_key(&'a self, a: &A) -> bool { return self.0.fwd.contains_key(a) }
    fn len(&'a self) -> usize { return self.0.fwd.len() }
}

impl<'a, A: Ord, B: Ord> ViewMultiMapLike<'a, A, B> for VFwd<'a, A, B> {
    type VMulti = VFwdSet<'a, B>;

    fn get(&'a self, a: &A) -> VFwdSet<'_, B> { return VFwdSet(self.0.fwd.get(a)) }
    fn contains_key(&'a self, a: &A) -> bool { return self.0.fwd.contains_key(a) }
    fn len(&'a self) -> usize { return self.0.fwd.len() }
}

// == Forward (set) ==
impl<'a, A: Ord+Clone, B: Ord+Clone> SetLike<B> for MFwdSet<'a, A, B> {
    fn insert(&mut self, b: B) -> bool { 
        let present = self.0.insert(b.clone());
        self.1.insert(b, self.0.key().clone());
        return present;
    }
    fn remove(&mut self, b: &B) -> std::option::Option<B> { 
        let present = self.0.remove(b);
        if present.is_some() { self.1.remove(b); }
        return present;
    }
}

impl<'a, B: Ord> ViewSetLike<B> for VFwdSet<'a, B> {
    fn contains(&self, b: &B) -> bool { self.0.contains(b) }
    fn len(&self) -> usize { self.0.len() }
}

impl<'a, A: Ord, B: Ord> ViewSetLike<B> for MFwdSet<'a, A, B> {
    fn contains(&self, b: &B) -> bool { self.0.contains(b) }
    fn len(&self) -> usize { self.0.len() }
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