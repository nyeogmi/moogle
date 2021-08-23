use super::super::interfaces::{ViewSetLike, SetLike, ViewMapLike, MapLike, ViewMultiMapLike, MultiMapLike};
use super::super::structures::{BTreeMultiMap, MValues, VValues};
use std::collections::{BTreeMap, BTreeSet};

// == Data structure ==
struct SetToOne<A: Ord, B: Ord> {
    fwd: BTreeMap<A, B>,
    bwd: BTreeMultiMap<B, A>,
}

struct MFwd<'a, A: Ord, B: Ord>(&'a mut SetToOne<A, B>);
struct MFwdSet<'a, A: Ord, B: Ord>(MValues<'a, A, B>, &'a mut BTreeMultiMap<B, A>);
struct MBwd<'a, A: Ord, B: Ord>(&'a mut SetToOne<A, B>);
struct MBwdSet<'a, A: Ord, B: Ord>(MValues<'a, B, A>, &'a mut BTreeMap<A, B>);

struct VFwd<'a, A: Ord, B: Ord>(&'a SetToOne<A, B>);
struct VFwdSet<'a, B: Ord>(VValues<'a, B>);
struct VBwd<'a, A: Ord, B: Ord>(&'a SetToOne<A, B>);
struct VBwdSet<'a, A: Ord>(VValues<'a, A>);

// == Accessors ==
impl<A: Ord, B: Ord> SetToOne<A, B> {
    pub fn fwd(&self) -> VFwd<A, B> { VFwd(self) }
    pub fn bwd(&self) -> VBwd<A, B> { VBwd(self) }
} 

impl<A: Ord+Clone, B: Ord+Clone> SetToOne<A, B> {
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
impl<'a, A: Ord+Clone, B: Ord+Clone> MultiMapLike<'a, B, A> for MBwd<'a, A, B> {
    type MMulti = MBwdSet<'a, A, B>;
    type MExpunge = BTreeSet<A>;

    fn get_mut(&mut self, b: B) -> MBwdSet<'_, A, B> { 
        return MBwdSet(self.0.bwd.get_mut(b), &mut self.0.fwd);
    }

    fn insert(&mut self, b: B, a: A) {
        self.0.bwd.insert(b.clone(), a.clone());
        self.0.fwd.insert(a, b);
     }
    fn remove(&mut self, b: &B) -> Self::MExpunge {
        let as_ = self.0.bwd.remove(b);
        for a in as_.iter() {
            self.0.fwd.remove(a);
        }
        return as_;
    }
}

impl<'a, A: Ord, B: Ord> ViewMultiMapLike<'a, B, A> for MBwd<'a, A, B> {
    type VMulti = VBwdSet<'a, A>;

    fn get(&'a self, b: &B) -> VBwdSet<'_, A> { return VBwdSet(self.0.bwd.get(b)) }
    fn contains_key(&'a self, b: &B) -> bool { return self.0.bwd.contains_key(b) }
    fn len(&'a self) -> usize { return self.0.fwd.len() }
}

impl<'a, A: Ord, B: Ord> ViewMultiMapLike<'a, B, A> for VBwd<'a, A, B> {
    type VMulti = VBwdSet<'a, A>;

    fn get(&'a self, b: &B) -> VBwdSet<'_, A> { return VBwdSet(self.0.bwd.get(b)) }
    fn contains_key(&'a self, b: &B) -> bool { return self.0.bwd.contains_key(b) }
    fn len(&'a self) -> usize { return self.0.fwd.len() }
}

// == Forward (set) ==
impl<'a, A: Ord+Clone, B: Ord+Clone> SetLike<A> for MBwdSet<'a, A, B> {
    fn insert(&mut self, a: A) -> bool { 
        let present = self.0.insert(a.clone());
        self.1.insert(a, self.0.key().clone());
        return present;
    }
    fn remove(&mut self, a: &A) -> std::option::Option<A> { 
        let present = self.0.remove(a);
        if present.is_some() { self.1.remove(a); }
        return present;
    }
}

impl<'a, A: Ord> ViewSetLike<A> for VBwdSet<'a, A> {
    fn contains(&self, a: &A) -> bool { self.0.contains(a) }
    fn len(&self) -> usize { self.0.len() }
}

impl<'a, A: Ord, B: Ord> ViewSetLike<A> for MBwdSet<'a, A, B> {
    fn contains(&self, a: &A) -> bool { self.0.contains(a) }
    fn len(&self) -> usize { self.0.len() }
}

// == TODO: Re-export setlike views of VFwd et al ==