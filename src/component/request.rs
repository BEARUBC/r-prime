pub enum Request<M> {
    HandleMessage(M),
    RunJob,
}
