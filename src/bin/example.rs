use moogle::*;

fn main() {
    let mut s: SharedSetToSet::<char, usize> = SharedSetToSet::new();

    let fwd = s.fwd().get('a');

    fwd.insert(1);
    fwd.insert(2);

    for i in fwd.iter() {
        fwd.insert(0);
        println!("Found: {:?}", i)
    }
}
