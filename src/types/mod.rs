// pub mod asdf;
pub mod driven;
pub mod driver;

use std::error::Error;
use std::thread::JoinHandle;

use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;

const BUFFER_SIZE: usize = 10usize;

type DriverFn<Con, Fut> = fn(Con) -> Fut;
type DrivenFn<Con, Rcv, Fut> = fn(Con, Receiver<Rcv>) -> Fut;
type DriverableResult<Er> = Result<JoinHandle<Result<(), Er>>, Box<dyn Error>>;

pub trait Componentable {
    type Type;
    type Con;
    type Er;

    fn new(f: Self::Type) -> Self;
    fn start(self, txs: Self::Con) -> DriverableResult<Self::Er>;
}

pub trait Initable {
    type Rcv;

    fn init(&mut self) -> Sender<Self::Rcv>;
}
