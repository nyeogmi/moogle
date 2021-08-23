use super::super::interfaces::{SetLike, ListLike, MapLike};

struct OneToSet<A, B> {
}

struct Fwd<A, B> {

}

struct FwdSet<A, B> {
    // set of Bs
}

struct Rev<A, B> {

}

// == Forward ==
impl<A, B> SetLike<A> for Fwd<A, B> {

}

impl<A, B> MapLike<A, FwdSet<A, B>> for Fwd<A, B> {

}

// == Reverse ==
impl<A, B> SetLike<B> for Rev<A, B> {

}

impl<A, B> MapLike<B, A> for Rev<A, B> {

}