#[derive(Clone, Copy, Debug)]
pub enum IterWork { 
    Front,
    Back 
}

impl quickcheck::Arbitrary for IterWork {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self { 
        *g.choose(&[IterWork::Front, IterWork::Back]).unwrap()
    }
}

#[derive(Clone, Debug)]
pub enum IterWorkOr<T> {
    IW(IterWork),
    N(T),
}

impl <T: quickcheck::Arbitrary> quickcheck::Arbitrary for IterWorkOr<T> {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self { 
        let options: [fn(&mut quickcheck::Gen) -> IterWorkOr<T>; 4] = [
            |g_| IterWorkOr::IW(IterWork::arbitrary(g_)),
            |g_| IterWorkOr::N(T::arbitrary(g_)),
            |g_| IterWorkOr::N(T::arbitrary(g_)),
            |g_| IterWorkOr::N(T::arbitrary(g_)),
        ];
        g.choose(&options).unwrap()(g)
    }
}

#[derive(Clone, Debug)]
pub(crate) enum Phase {
    Insert { fwd: Vec<IterWorkOr<(u16, i16)>>, bwd: Vec<IterWorkOr<(i16, u16)>> },
    Expunge { fwd: Vec<IterWorkOr<u16>>, bwd: Vec<IterWorkOr<i16>> },
    Remove { fwd: Vec<IterWorkOr<(u16, i16)>>, bwd: Vec<IterWorkOr<(i16, u16)>> },
}

#[derive(Clone, Debug)]
pub(crate) struct Routine(pub Vec<Phase>);

// TODO: Instead of this, just have two Phase types?
pub fn items_only<T: Clone>(vs: &Vec<IterWorkOr<T>>) -> Vec<T> {
    let mut result = vec![];
    for v in vs.iter() {
        match v {
            IterWorkOr::IW(_) => {}
            IterWorkOr::N(t) => { result.push(t.clone()); }
        }
    }
    result
}

pub fn items_or_work<T: Clone>(vs: &Vec<IterWorkOr<T>>, mut handle: impl FnMut(T), mut work: impl FnMut(IterWork)) {
    for v in vs.iter() {
        match v { 
            IterWorkOr::IW(w) => work(*w),
            IterWorkOr::N(t) => { handle(t.clone()) }
        }
    }
}

// TODO: Test removal and expunging too
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

