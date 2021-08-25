use moogle::*;

fn main() {
    let s: SharedSetToSet::<char, usize> = SharedSetToSet::new();
    let fwd = s.fwd().get('a');

    fwd.insert(4);
    fwd.insert(5);

    for i in fwd.iter() {
        fwd.insert(3);
        println!("Found: {:?}", i)
    }

    for i in s.fwd().iter() {
        println!("Found: {:?}", i)
    }

    println!("reverse:");
    for i in s.fwd().iter().rev() {
        println!("Found: {:?}", i);
        s.fwd().get('a').insert(6);
    }
}
