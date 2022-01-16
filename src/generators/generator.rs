pub trait Generator<T> {
    fn generate(&self) -> T;
}
