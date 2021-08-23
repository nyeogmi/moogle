use crate::relations::keybound::Id;

use crate::relations::interfaces::{ViewMapLike, MapLike};

use crate::relations::structures::{ToOne};

// == Data structure ==
pub struct OneToOne<A: Id, B: Id> {
    fwd: ToOne<A, B>,
    bwd: ToOne<B, A>,
}

pub struct MFwd<'a, A: Id, B: Id>(&'a mut OneToOne<A, B>);
pub struct MBwd<'a, A: Id, B: Id>(&'a mut OneToOne<A, B>);

pub struct VFwd<'a, A: Id, B: Id>(&'a OneToOne<A, B>);
pub struct VBwd<'a, A: Id, B: Id>(&'a OneToOne<A, B>);

// == Accessors ==
impl<A: Id, B: Id> OneToOne<A, B> {
    pub fn fwd(&self) -> VFwd<A, B> { VFwd(self) }
    pub fn bwd(&self) -> VBwd<A, B> { VBwd(self) }
} 

impl<A: Id, B: Id> OneToOne<A, B> {
    pub fn mut_fwd(&mut self) -> MFwd<A, B> { MFwd(self) }
    pub fn mut_bwd(&mut self) -> MBwd<A, B> { MBwd(self) }
} 

// == Forward ==
impl<'a, A: Id, B: Id> MapLike<'a, A, B> for MFwd<'a, A, B> {
    fn insert(&mut self, a: A, b: B) -> Option<B> {
        let bwd = &mut self.0.bwd;
        let result = self.0.fwd.insert(a.clone(), b.clone(), move |k, v| { bwd.remove(v, k, |_, _|{}); });

        let fwd = &mut self.0.fwd;
        self.0.bwd.insert(b, a, move |k, v| { fwd.remove(v, k, |_, _| {}); });

        result
     }

    fn expunge(&mut self, a: A) -> Option<B> { 
        let bwd = &mut self.0.bwd;
        self.0.fwd.expunge(a, move |k, v| { bwd.remove(v, k, |_, _|{}); })
    }
}

impl<'a, A: Id, B: Id> ViewMapLike<'a, A, B> for MFwd<'a, A, B> {
    fn get(&self, a: A) -> Option<B> { self.0.fwd.get(a).as_option() }
    fn contains_key(&self, a: A) -> bool { self.0.fwd.contains_key(a) }
    fn len(&self) -> usize { self.0.fwd.len() }
}

impl<'a, A: Id, B: Id> ViewMapLike<'a, A, B> for VFwd<'a, A, B> {
    fn get(&self, a: A) -> Option<B> { self.0.fwd.get(a).as_option() }
    fn contains_key(&self, a: A) -> bool { self.0.fwd.contains_key(a) }
    fn len(&self) -> usize { self.0.fwd.len() }
}

// == Backward ==
impl<'a, A: Id, B: Id> MapLike<'a, B, A> for MBwd<'a, A, B> {
    fn insert(&mut self, b: B, a: A) -> Option<A> {
        let fwd = &mut self.0.fwd;
        let result = self.0.bwd.insert(b.clone(), a.clone(), move |k, v| { fwd.remove(v, k, |_, _|{}); });

        let bwd = &mut self.0.bwd;
        self.0.fwd.insert(a, b, move |k, v| { bwd.remove(v, k, |_, _| {}); });
        result
     }

    fn expunge(&mut self, b: B) -> Option<A> { 
        let fwd = &mut self.0.fwd;
        self.0.bwd.expunge(b, move |k, v| { fwd.remove(v, k, |_, _|{}); })
    }
}

impl<'a, A: Id, B: Id> ViewMapLike<'a, B, A> for MBwd<'a, A, B> {
    fn get(&self, b: B) -> Option<A> { self.0.bwd.get(b).as_option() }
    fn contains_key(&self, b: B) -> bool { self.0.bwd.contains_key(b) }
    fn len(&self) -> usize { self.0.fwd.len() }
}

impl<'a, A: Id, B: Id> ViewMapLike<'a, B, A> for VBwd<'a, A, B> {
    fn get(&self, b: B) -> Option<A> { 
        let gb = self.0.bwd.get(b);
        gb.as_option() 
    }
    fn contains_key(&self, b: B) -> bool { self.0.bwd.contains_key(b) }
    fn len(&self) -> usize { self.0.fwd.len() }
}

// == TODO: Re-export setlike views of VFwd et al ==