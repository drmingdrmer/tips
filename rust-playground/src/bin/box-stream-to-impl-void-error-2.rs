//! When passing a BoxStream<'static> to a function expecting `impl Void`,
//! the function does not compile.
//!
//! Although this issue requries many conditions:
//! - a non-Pin-Box stream is ok.
//! - replace `impl Void` with impl Stream<Item = u64> + Send + 'static is ok.
//! - without sleep() is ok.
//! - without request.void() is ok.
//! - write it in `async fn` form is ok.

use futures::stream;
use futures::Stream;
use std::future::Future;
use std::pin::Pin;
use tokio::time::Duration;

async fn func_ok() -> () {
    let strm = stream::iter(vec![]);
    let strm: Pin<Box<dyn Stream<Item = u64> + Send + 'static>> = Box::pin(strm);

    stream_snapshot(strm).await;
}

fn func_ok2() -> impl Future<Output = ()> + Send {
    let strm = stream::iter(vec![]);
    let strm: Pin<Box<dyn Stream<Item = u64> + Send + 'static>> = Box::pin(strm);

    stream_snapshot(strm)
}

// This emit higher rank lifetime error:
fn func_not_compile() -> impl Future<Output = ()> {
    let strm = stream::iter(vec![]);
    let strm: Pin<Box<dyn Stream<Item = u64> + Send + 'static>> = Box::pin(strm);

    async move {
        stream_snapshot(strm).await;
    }
}

pub async fn stream_snapshot(request: impl Void) -> () {
    let _req = request.void();
    tokio::time::sleep(Duration::from_secs(1)).await;
}

pub trait Void {
    type Stream: Stream + Send + 'static;
    fn void(self) -> Self::Stream;
}

impl Void for Pin<Box<dyn Stream<Item = u64> + Send + 'static>> {
    type Stream = Self;

    fn void(self) -> Self::Stream {
        self
    }
}

fn main() {}
