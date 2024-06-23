use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct Container {
    value: i32,
}

struct ContainerFuture<'a> {
    container: Option<&'a mut Container>,
}

impl<'a> Future for ContainerFuture<'a> {
    type Output = &'a i32;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!()
        // Poll::Ready(&self.container.as_mut().unwrap().value)
    }
}

fn ck<'a, Fut>(f: Fut)
where
    Fut: Future<Output = &'a i32>,
{
}

fn main() {
    let x = 1i32;
    let q = async { &x };
    ck(q);
}
