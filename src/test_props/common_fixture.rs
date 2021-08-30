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

// TODO: Instead of this, just have two Phase types?
pub fn items_or_work<T: Clone>(vs: &Vec<IterWorkOr<T>>, mut handle: impl FnMut(T), mut work: impl FnMut(IterWork)) {
    for v in vs.iter() {
        match v { 
            IterWorkOr::IW(w) => work(*w),
            IterWorkOr::N(t) => { handle(t.clone()) }
        }
    }
}