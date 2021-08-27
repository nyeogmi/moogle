use super::fixture::IterWork;

pub const DESIRED_N_ITERATORS: usize = 4;


pub(super) struct IterBank<'a> {
    source_fn: Box<dyn 'a+Fn() -> Vec<Box<dyn 'a+DoubleEndedIterator<Item=()>>>>,
    iterators: Vec<Box<dyn 'a+DoubleEndedIterator<Item=()>>>
}

impl<'a> IterBank<'a> {
    pub fn new(
        source_fn: impl 'a+Fn() -> Vec<Box<dyn 'a+DoubleEndedIterator<Item=()>>>,
    ) -> IterBank<'a> {
        IterBank { source_fn: Box::new(source_fn), iterators: Vec::new() }
    }

    pub fn add_iterator<T>(&mut self, iterator: impl 'a+DoubleEndedIterator<Item=T>) {
        self.iterators.push(wrap(iterator));
    }

    pub fn do_work(&mut self, work: IterWork) {
        let mut iterators_2 = vec![];
        if self.iterators.len() < DESIRED_N_ITERATORS {
            self.iterators.extend((self.source_fn)())
        }

        for mut iter in self.iterators.drain(..) {
            let item = match work { IterWork::Front => iter.next(), IterWork::Back => iter.next_back() };
            match item {
                None => {} ,
                Some(_) => iterators_2.push(iter)
            };
        }

        self.iterators = iterators_2;
    }
}

pub fn wrap<'a, T>(a: impl 'a+DoubleEndedIterator<Item=T>) -> Box<dyn 'a+DoubleEndedIterator<Item=()>> {
    Box::new(a.map(|_| ()))
}