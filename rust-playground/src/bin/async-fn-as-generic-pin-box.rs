use std::future::Future;
use std::pin::Pin;

struct Foo<F>
where F: for<'a> Fn(&'a u64) -> Pin<Box<dyn Future<Output = ()> + 'a>>
{
    doit: F,
}

impl<F> Foo<F>
where F: for<'a> Fn(&'a u64) -> Pin<Box<dyn Future<Output = ()> + 'a>>
{
    async fn rock(&self) {
        let x = 3u64;
        (self.doit)(&x).await;
    }
}

fn sing<'a>(_x: &'a u64) -> Pin<Box<dyn Future<Output = ()> + 'a>> {
    let f = async move {
        println!("{}", _x);
    };
    Box::pin(f)
}

#[tokio::main]
async fn main() {
    let f = Foo { doit: sing };
    f.rock().await;
}
