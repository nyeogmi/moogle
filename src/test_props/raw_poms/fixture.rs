#[derive(Clone, Debug)]
pub enum Op {
    Insert(String),
    Remove,
    Append(String),

    // shuffle the stack of IDs
    Shuffle(u8),
}

#[derive(Clone, Debug)]
pub struct Routine(pub(crate) Vec<Op>);

impl quickcheck::Arbitrary for Routine {
    fn arbitrary(g: &mut quickcheck::Gen) -> Routine { 
        let length: i16 = *g.choose(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]).unwrap();
        let mut xs = vec![];
        for _ in 0..length {
            xs.push(Op::arbitrary(g));
        }
        Routine(xs)
    }
}

impl quickcheck::Arbitrary for Op {
    fn arbitrary(g: &mut quickcheck::Gen) -> Op { 
        let options: [fn(&mut quickcheck::Gen) -> Op; 4] = [
            |g_| Op::Insert(String::arbitrary(g_)),
            |_| Op::Remove,
            |g_| Op::Append(String::arbitrary(g_)),
            |g_| Op::Shuffle(*g_.choose(&[1, 2, 3, 4]).unwrap()),
        ];
        g.choose(&options).unwrap()(g)
    }
}

