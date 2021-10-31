use std::future::Future;
use std::thread;

use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;

use crate::types::Componentable;
use crate::types::DrivenFn;
use crate::types::Initable;
use crate::types::BUFFER_SIZE;

pub struct Driven<Con, Rcv, Fut> {
    f: DrivenFn<Con, Rcv, Fut>,
    rx: Option<Receiver<Rcv>>,
}

impl<Con, Rcv, Fut, Er> Componentable for Driven<Con, Rcv, Fut>
where
    Con: 'static + Send + Sync,
    Rcv: 'static + Send + Sync,
    Fut: 'static + Future<Output = Result<(), Er>>,
    Er: 'static + Send + Sync,
{
    type Type = DrivenFn<Con, Rcv, Fut>;
    type Con = Con;
    type Er = Er;

    fn new(f: DrivenFn<Con, Rcv, Fut>) -> Self {
        Self { f, rx: None }
    }

    fn start(self, txs: Con) -> super::DriverableResult<Er> {
        let runtime = Runtime::new()?;
        let join_handle =
            thread::spawn(move || runtime.block_on((self.f)(txs, self.rx.expect("Driven::init must be called first"))));

        Ok(join_handle)
    }
}

impl<Con, Rcv, Fut> Initable for Driven<Con, Rcv, Fut> {
    type Rcv = Rcv;

    fn init(&mut self) -> Sender<Rcv> {
        let (tx, rx) = mpsc::channel::<Rcv>(BUFFER_SIZE);
        self.rx = Some(rx);

        tx
    }
}
