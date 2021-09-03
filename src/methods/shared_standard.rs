pub trait SharedExtend<A> {
    fn extend<T: IntoIterator<Item = A>>(&self, iter: T);

    fn extend_one(&self, item: A) {
        self.extend(Some(item))
    }
}