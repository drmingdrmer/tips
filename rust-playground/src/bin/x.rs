#![feature(pin_macro)]

use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};

struct Get<'a>
where
    Self: 'a,
{
    value: Box<i32>,
    r: &'a i32,
}

impl<'a> Get<'a>
where
    Self: 'a,
{
    fn new(value: i32) -> Self {
        let b = Box::new(value);
        let r = unsafe { &*((&*b) as *const i32) };
        Self { value: b, r }
    }

    fn get_r<'b>(&'b self) -> &'a i32
    where
        'b: 'a,
    {
        self.r
    }
}

impl<'a> Future for Get<'a>
where
    Self: 'a,
{
    type Output = &'a i32 where Self: 'a;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(self.r)
    }
}

fn main() {
    // let got = {
    //     let mut a = Get::new(2);
    //     a.get_r()
    // };

    let cx = &mut Context::from_waker(futures::task::noop_waker_ref());
    let got = {
        let mut a = Get::new(2);
        let fu = std::pin::pin!(a);
        let got = fu.poll(cx);
        got
    };

    dbg!(got);
}
