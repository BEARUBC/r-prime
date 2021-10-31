use std::future::Future;
use std::time::Duration;

use tokio::sync::oneshot;
use tokio::task::spawn;
use tokio::time::sleep as tokio_sleep;

pub type Response<T> = oneshot::Sender<T>;

pub async fn sleep(millis: u64) {
    tokio_sleep(Duration::from_millis(millis)).await
}

pub fn set_timeout<F>(f: fn() -> F, millis: u64)
where
    F: 'static + Send + Future<Output = ()>,
{
    spawn(async move {
        sleep(millis).await;
        f().await;
    });
}

pub fn create_response_channel<T>() -> (oneshot::Sender<T>, oneshot::Receiver<T>) {
    oneshot::channel::<T>()
}
