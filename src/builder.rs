pub trait Builder<SUC, ERR> {
    fn build(self) -> Result<SUC, ERR>;
}
