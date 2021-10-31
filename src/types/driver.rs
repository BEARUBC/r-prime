use std::future::Future;
use std::thread;

use tokio::runtime::Runtime;

use crate::types::Componentable;
use crate::types::DriverFn;

pub struct Driver<Con, Fut> {
    f: DriverFn<Con, Fut>,
}

impl<Con, Fut, Er> Componentable for Driver<Con, Fut>
where
    Con: 'static + Send + Sync,
    Fut: 'static + Future<Output = Result<(), Er>>,
    Er: 'static + Send + Sync,
{
    type Con = Con;
    type Er = Er;
    type Type = DriverFn<Con, Fut>;

    fn new(f: DriverFn<Con, Fut>) -> Self {
        Self { f }
    }

    fn start(self, txs: Con) -> super::DriverableResult<Er> {
        let runtime = Runtime::new()?;
        let join_handle = thread::spawn(move || runtime.block_on((self.f)(txs)));

        Ok(join_handle)
    }
}
