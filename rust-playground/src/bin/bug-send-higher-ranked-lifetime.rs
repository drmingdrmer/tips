#![feature(type_alias_impl_trait)]

//! Build a minimized reproducible example for:
//!
//! https://github.com/rust-lang/rust/issues/100013
//! https://github.com/rust-lang/rust/issues/114046
//! https://github.com/rust-lang/rust/issues/110338

use std::future::Future;

pub fn assert_send<T: Send>(v: T) -> T {
    v
}

pub struct Level {}
pub struct Level2 {}

pub trait MapApiRO: Send + Sync {
    type GetFut<'f>: Future<Output = ()>
    where
        Self: 'f;

    fn get<'f>(self) -> Self::GetFut<'f>;
}

impl<'d> MapApiRO for &'d Level {
    type GetFut<'f> = impl Future<Output =()> + 'f where Self: 'f;

    fn get<'f>(self) -> Self::GetFut<'f>
    where
        Self: 'f,
    {
        async move { () }
    }
}

impl<'ro_d> MapApiRO for &'ro_d Level2
where
    for<'e> &'e Level: MapApiRO,
{
    type GetFut<'f> = impl Future<Output=()> + 'f where Self: 'f;

    fn get<'f>(self) -> Self::GetFut<'f>
    where
        Self: 'f,
    {
        async move {
            let l = Level {};
            l.get().await
        }
    }
}

#[tokio::main]
async fn main() {
    let l = Level {};
    let fu = l.get();
    let _ = assert_send(fu);

    // error: implementation of `MapApiRO` is not general enough
    //     --> src/bin/map-api-send-higher-ranked-lifetime.rs:60:13
    //     |
    //  60 |     let _ = assert_send(fu);
    //     |             ^^^^^^^^^^^^^^^ implementation of `MapApiRO` is not general enough
    //     |
    //     = note: `MapApiRO` would have to be implemented for the type `&'0 Level`, for any lifetime `'0`...
    //     = note: ...but `MapApiRO` is actually implemented for the type `&'1 Level`, for some specific lifetime `'1`
    let static_levels = Level2 {};
    let fu = static_levels.get();
    let _ = assert_send(fu);
}
