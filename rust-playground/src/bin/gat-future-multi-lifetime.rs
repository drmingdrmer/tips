#![feature(type_alias_impl_trait)]

use futures::executor::block_on;
use std::borrow::Borrow;
use std::future::Future;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Val(u64);

trait MapApiRO<'d, K>: Send + Sync {
    type GetFut<'f, 'g>: Future<Output = u64>
    where
        Self: 'f,
        Self: 'g;

    fn get<'f, 'g>(self, key: &'f u64, v: &'g u64) -> Self::GetFut<'f, 'g>;
}

struct Foo;

impl<'d, K> MapApiRO<'d, K> for Foo {
    type GetFut<'f, 'g> = impl Future<Output = u64> + 'f + 'g
    where
        Self: 'f,
        Self: 'g
    ;

    fn get<'f, 'g>(self, key: &'f u64, v: &'g u64) -> Self::GetFut<'f, 'g> {
        async move {
            //
            1
        }
    }
}

fn main() {
    let mut foo = Foo;

    let k = 1;
    let v = 2;

    let fut = block_on(<Foo as MapApiRO<'_, String>>::get(foo, &k, &v));

    println!("{:?}", fut);
}
