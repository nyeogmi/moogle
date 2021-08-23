use crate::relations::interfaces::{ViewMultiMapLike, MultiMapLike, ViewMapLike, MapLike};
use crate::relations::interfaces::{ViewSetLike, SetLike, EvictSetLike};

use crate::relations::structures::{ToOne};
use crate::relations::structures::{ToSet, VSet, MSet};

use std::collections::BTreeSet;

// == Data structure ==
struct OneToSet<A: Ord, B: Ord> {
    fwd: ToSet<A, B>,
    bwd: ToOne<B, A>,
}

struct MFwd<'a, A: Ord, B: Ord>(&'a mut OneToSet<A, B>);
struct MFwdSet<'a, A: Ord, B: Ord>(MSet<'a, A, B>, &'a mut ToOne<B, A>);
struct MBwd<'a, A: Ord, B: Ord>(&'a mut OneToSet<A, B>);

struct VFwd<'a, A: Ord, B: Ord>(&'a OneToSet<A, B>);
struct VFwdSet<'a, A: Ord, B: Ord>(VSet<'a, A, B>);
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

    fn get_mut(&'a mut self, a: A) -> MFwdSet<'a, A, B> {
        MFwdSet(self.0.fwd.get_mut(a), &mut self.0.bwd)
    }

    fn insert(&mut self, a: A, b: B) {
        let bwd = &mut self.0.bwd;
        let result = self.0.fwd.insert(a.clone(), b.clone(), move |k, v| { bwd.remove(v, k, |_, _|{}); });

        let fwd = &mut self.0.fwd;
        self.0.bwd.insert(b, a, move |k, v| { fwd.remove(v, k, |_, _| {}); });

        result
     }

    fn expunge(&mut self, a: &A) -> BTreeSet<B> { 
        let bwd = &mut self.0.bwd;
        self.0.fwd.expunge(a, move |k, v| { bwd.remove(v, k, |_, _| {}); })
    }
}

impl<'a, A: Ord, B: Ord> ViewMultiMapLike<'a, A, B> for MFwd<'a, A, B> {
    type VMulti = VFwdSet<'a, A, B>;

    fn get(&self, a: &A) -> VFwdSet<'_, A, B> { VFwdSet(self.0.fwd.get(a)) }
    fn contains_key(&self, a: &A) -> bool { self.0.fwd.contains_key(a) }
    fn len(&self) -> usize { self.0.fwd.len() }
}

impl<'a, A: Ord, B: Ord> ViewMultiMapLike<'a, A, B> for VFwd<'a, A, B> {
    type VMulti = VFwdSet<'a, A, B>;

    fn get(&self, a: &A) -> VFwdSet<'_, A, B> { VFwdSet(self.0.fwd.get(a)) }
    fn contains_key(&self, a: &A) -> bool { self.0.fwd.contains_key(a) }
    fn len(&self) -> usize { self.0.fwd.len() }
}

// == Forward (sets) ==
impl<'a, A: Ord+Clone, B: Ord+Clone> SetLike<B> for MFwdSet<'a, A, B> {
    fn insert(&mut self, b: B) -> bool { 
        let alt = &mut self.1;
        let result = self.0.insert(b.clone(), move |k, v| { alt.remove(v, k, |_, _|{}); });

        let key = self.0.key().clone();
        let stt = &mut self.0;

        self.1.insert(b, key, move |k, _| { stt.remove(k, |_, _| {}); });
        result
    }
    fn remove(&mut self, b: &B) -> Option<B> { 
        let opposite = &mut self.1;
        self.0.remove(b, move |k, v| { opposite.remove(v, k, |_, _|{}); }) 
    }
}

impl<'a, A: Ord, B: Ord> ViewSetLike<B> for MFwdSet<'a, A, B> {
    fn contains(&self, b: &B) -> bool { self.0.contains(b) }
    fn len(&self) -> usize { self.0.len() }
}

impl<'a, A: Ord, B: Ord> ViewSetLike<B> for VFwdSet<'a, A, B> {
    fn contains(&self, b: &B) -> bool { self.0.contains(b) }
    fn len(&self) -> usize { self.0.len() }
}

// == Backward ==
impl<'a, A: Ord+Clone, B: Ord+Clone> MapLike<'a, B, A> for MBwd<'a, A, B> {
    fn insert(&mut self, b: B, a: A) -> Option<A> {
        let fwd = &mut self.0.fwd;
        let result = self.0.bwd.insert(b.clone(), a.clone(), move |k, v| { fwd.remove(v, k, |_, _|{}); });

        let bwd = &mut self.0.bwd;
        self.0.fwd.insert(a, b, move |k, v| { bwd.remove(v, k, |_, _| {}); });
        result
     }

    fn expunge(&mut self, b: &B) -> Option<A> { 
        let fwd = &mut self.0.fwd;
        self.0.bwd.expunge(b, move |k, v| { fwd.remove(v, k, |_, _|{}); })
    }
}

impl<'a, A: Ord, B: Ord> ViewMapLike<'a, B, A> for MBwd<'a, A, B> {
    fn get(&self, b: &B) -> Option<&A> { self.0.bwd.get(b).as_option() }
    fn contains_key(&self, b: &B) -> bool { self.0.bwd.contains_key(b) }
    fn len(&self) -> usize { self.0.fwd.len() }
}

impl<'a, A: Ord, B: Ord> ViewMapLike<'a, B, A> for VBwd<'a, A, B> {
    fn get(&'a self, b: &B) -> Option<&'a A> { 
        let gb = self.0.bwd.get(b);
        gb.as_option() 
    }
    fn contains_key(&self, b: &B) -> bool { self.0.bwd.contains_key(b) }
    fn len(&self) -> usize { self.0.fwd.len() }
}

// == TODO: Re-export setlike views of VFwd et al ==