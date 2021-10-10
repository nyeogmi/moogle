#[derive(Clone, Debug)]
pub enum UnsharedOp {
    Insert(String),
    Remove,
    Append(String),

    // shuffle the stack of IDs
    Shuffle(u8),
}

#[derive(Clone, Debug)]
pub enum SharedOp {
    Append(String),

    // shuffle the stack of IDs
    Shuffle(u8),
    IterateKeys,
}

#[derive(Clone, Debug)]
pub struct Routine { pub(crate) phases: Vec<Phase> }

#[derive(Clone, Debug)]
pub struct Phase(
    pub(crate) Vec<UnsharedOp>,
    pub(crate) Vec<SharedOp>,
);


impl quickcheck::Arbitrary for Routine {
    fn arbitrary(g: &mut quickcheck::Gen) -> Routine { 
        let length: i16 = *g.choose(&[0, 1, 1, 2, 2, 2, 3, 3, 4, 5]).unwrap();
        let mut xs = vec![];
        for _ in 0..length {
            xs.push(Phase::arbitrary(g));
        }
        Routine { phases: xs }
    }
}
impl quickcheck::Arbitrary for Phase {
    fn arbitrary(g: &mut quickcheck::Gen) -> Phase { 
        let length: i16 = *g.choose(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]).unwrap();
        let mut unshared = vec![];
        for _ in 0..length {
            unshared.push(UnsharedOp::arbitrary(g));
        }
        let mut shared = vec![];
        for _ in 0..length {
            shared.push(SharedOp::arbitrary(g));
        }
        Phase(unshared, shared)
    }
}

impl quickcheck::Arbitrary for UnsharedOp {
    fn arbitrary(g: &mut quickcheck::Gen) -> UnsharedOp { 
        let options: [fn(&mut quickcheck::Gen) -> UnsharedOp; 4] = [
            |g_| UnsharedOp::Insert(String::arbitrary(g_)),
            |_| UnsharedOp::Remove,
            |g_| UnsharedOp::Append(String::arbitrary(g_)),
            |g_| UnsharedOp::Shuffle(*g_.choose(&[1, 2, 3, 4]).unwrap()),
        ];
        g.choose(&options).unwrap()(g)
    }
}

impl quickcheck::Arbitrary for SharedOp {
    fn arbitrary(g: &mut quickcheck::Gen) -> SharedOp { 
        let options: [fn(&mut quickcheck::Gen) -> SharedOp; 3] = [
            |g_| SharedOp::Append(String::arbitrary(g_)),
            |g_| SharedOp::Shuffle(*g_.choose(&[1, 2, 3, 4]).unwrap()),
            |_| SharedOp::IterateKeys,
        ];
        g.choose(&options).unwrap()(g)
    }
}

