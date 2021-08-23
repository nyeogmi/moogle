use super::super::interfaces::{SetLike, ListLike, MapLike};

struct SetToOne<A, B> {
}

struct Fwd<A, B> {

}

struct Rev<A, B> {

}

struct RevSet<A, B> {
    // set of As
}

// == Forward ==
impl<A, B> SetLike<A> for Fwd<A, B> {

}

impl<A, B> MapLike<A, B> for Fwd<A, B> {

}

// == Reverse ==
impl<A, B> SetLike<B> for Rev<A, B> {

}

impl<A, B> MapLike<B, RevSet<A, B>> for Rev<A, B> {

}