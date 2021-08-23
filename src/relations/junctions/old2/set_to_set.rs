use super::super::interfaces::{ViewSetLike, SetLike, ViewMultiMapLike, MultiMapLike};
use super::super::structures::{BTreeMultiMap, MValues, VValues};
use std::collections::BTreeSet;

// == Data structure ==
struct SetToSet<A: Ord, B: Ord> {
    fwd: BTreeMultiMap<A, B>,
    bwd: BTreeMultiMap<B, A>,
}

struct MFwd<'a, A: Ord, B: Ord>(&'a mut SetToSet<A, B>);
struct MFwdSet<'a, A: Ord, B: Ord>(MValues<'a, A, B>, &'a mut BTreeMultiMap<B, A>);
struct MBwd<'a, A: Ord, B: Ord>(&'a mut SetToSet<A, B>);
struct MBwdSet<'a, A: Ord, B: Ord>(MValues<'a, B, A>, &'a mut BTreeMultiMap<A, B>);

struct VFwd<'a, A: Ord, B: Ord>(&'a SetToSet<A, B>);
struct VFwdSet<'a, B: Ord>(VValues<'a, B>);
struct VBwd<'a, A: Ord, B: Ord>(&'a SetToSet<A, B>);
struct VBwdSet<'a, A: Ord>(VValues<'a, A>);

// == Accessors ==
impl<A: Ord, B: Ord> SetToSet<A, B> {
    pub fn fwd(&self) -> VFwd<A, B> { VFwd(self) }
    pub fn bwd(&self) -> VBwd<A, B> { VBwd(self) }
} 

impl<A: Ord+Clone, B: Ord+Clone> SetToSet<A, B> {
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

// == Backward (set) ==
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