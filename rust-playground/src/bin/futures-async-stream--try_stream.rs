#![feature(generators)]

use futures::stream::Stream;
use futures_async_stream::try_stream;
use futures_util::TryStreamExt;
use std::collections::BTreeMap;
use std::num::ParseIntError;

#[try_stream(boxed, ok = i32, error = ParseIntError)]
async fn foo(stream: impl Stream<Item = String> + Send + 'static) {
    #[for_await]
    for x in stream {
        yield x.parse()?;
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut s = foo(futures::stream::iter(["1".to_string()]));
    while let Some(x) = s.try_next().await? {
        println!("{}", x);
    }

    let x: BTreeMap<String, i32> = BTreeMap::new();

    let v = x.range(""..);

    Ok(())
}
