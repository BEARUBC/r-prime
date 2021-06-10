pub trait Builder<A, B> {
    fn build(self) -> Result<A, B>;
}
