use super::common_fixture::*;
pub use super::common_fixture::{items_only, items_or_work};

#[derive(Clone, Debug)]
pub(crate) enum Phase {
    Insert { fwd: Vec<IterWorkOr<(u16, i16)>>, bwd: Vec<IterWorkOr<(i16, u16)>> },
    Expunge { fwd: Vec<IterWorkOr<u16>>, bwd: Vec<IterWorkOr<i16>> },
    Remove { fwd: Vec<IterWorkOr<(u16, i16)>>, bwd: Vec<IterWorkOr<(i16, u16)>> },
}

#[derive(Clone, Debug)]
pub(crate) struct Routine(pub Vec<Phase>);

impl quickcheck::Arbitrary for Routine {
    fn arbitrary(g: &mut quickcheck::Gen) -> Routine { 
        let length: i16 = *g.choose(&[0, 1, 1, 2, 2, 2, 3, 3, 4, 5]).unwrap();
        let mut xs = vec![];
        for _ in 0..length {
            xs.push(Phase::arbitrary(g));
        }
        Routine(xs)
    }
}

impl quickcheck::Arbitrary for Phase {
    fn arbitrary(g: &mut quickcheck::Gen) -> Phase { 
        let options: [fn(&mut quickcheck::Gen) -> Phase; 3] = [
            |g_| Phase::Insert { fwd: Vec::arbitrary(g_), bwd: Vec::arbitrary(g_) },
            |g_| Phase::Expunge { fwd: Vec::arbitrary(g_), bwd: Vec::arbitrary(g_) },
            |g_| Phase::Remove { fwd: Vec::arbitrary(g_), bwd: Vec::arbitrary(g_) },
        ];
        g.choose(&options).unwrap()(g)
    }
}

