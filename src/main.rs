#![feature(min_type_alias_impl_trait)]
pub mod relations;

use std::collections::BTreeSet;

use relations::{OneToOne, OneToSet, SetToOne, SetToSet, one_to_one, one_to_set, set_to_one, set_to_set};
use relations::interfaces::*;

fn main() {
    let mut l: SetToOne::<char, usize> = SetToOne::new();

    l.mut_fwd().insert('A', 456);
    l.mut_fwd().insert('B', 456);
    l.mut_fwd().insert('C', 456);
    l.mut_fwd().insert('W', 456);

    l.mut_fwd().insert('W', 1001);
    l.mut_fwd().insert('x', 1001);
    l.mut_fwd().insert('y', 1001);
    l.mut_fwd().insert('z', 1001);

    println!("Hello! {:?}", l);

    for i in l.bwd().get(456).iter() {
        println!("Item: {}", i);
    }

    println!("Map (forwards): {:?}", l.fwd());
    println!("Map (backwards): {:?}", l.bwd());
}
