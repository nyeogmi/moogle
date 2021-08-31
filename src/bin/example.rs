use moogle::*;

fn main1() {
    let s: OneToMany::<char, usize> = OneToMany::new();
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
    for i in s.bwd().iter().rev() {
        println!("Found: {:?}", i);
        s.fwd().get('a').insert(6);
    }
    println!("{:?}", s);
}

fn main() {
    main1();

    let mut table = Pom::new();
    let m1 = table.insert("Kupdi Koop");
    let m2 = table.insert("Pukla Puki");
    let m3 = table.insert("Pukna Pako");
    let m4 = table.insert("Kipli Kipp");
    let m5 = table.insert("Puksi Piko");
    let m6 = table.insert("Kupqu Kogi");
    let m7 = table.insert("Kupta Kapa");

    println!("Roll call! {:?}", [m1, m2, m3, m4, m5, m6, m7]);

    println!("Table: {:#?}", table);
    let (index, mut elements) = table.share();
    println!("Index: {:#?}", index);
    println!("Elements: {:#?}", elements);
    for m in index.keys() {
        println!("It's {:?}: {:?}", m, elements.get(m));
        *elements.get_mut(m4).unwrap() = "Kwilly!";
    }
}