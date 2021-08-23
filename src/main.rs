pub mod relations;

use std::collections::BTreeSet;

use relations::{OneToOne, OneToSet, SetToOne, SetToSet, one_to_one, one_to_set, set_to_one, set_to_set};
use relations::interfaces::*;

fn main() {
    let mut l: SetToOne::<usize, usize> = SetToOne::new();

    l.mut_fwd().insert(123, 456);

    let mut test_set: BTreeSet<usize> = BTreeSet::new();
    test_set.insert(345);

    println!("Hello! {:?}", test_set);
    // TODO: Debug for internal SetLike types
    // println!("Hello! {:?}", l.bwd().get(456));
}
