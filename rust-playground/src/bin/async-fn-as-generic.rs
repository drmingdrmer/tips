use std::future::Future;

struct Foo<Fut, F>
where
    Fut: Future<Output = ()>,
    F: for<'a> Fn(&'a u64) -> Fut,
{
    doit: F,
}

impl<Fut, F> Foo<Fut, F>
where
    Fut: Future<Output = ()>,
    F: for<'a> Fn(&'a u64) -> Fut,
{
    async fn rock(&self) {
        let x = 3u64;
        (self.doit)(&x).await;
    }
}

fn sing<'a>(_x: &'a u64) -> impl Future<Output = ()> + 'a {
    async move {
        println!("{}", _x);
    }
}

#[tokio::main]
async fn main() {
    let _f = Foo {
        doit: sing, // Fail to compile
    };

    // _f.rock().await;
}

// error[E0308]: mismatched types
//   --> src/bin/try-it.rs:30:14
//    |
// 30 |       let _f = Foo {
//    |  ______________^
// 31 | |         doit: sing, // Fail to compile
// 32 | |     };
//    | |_____^ one type is more general than the other
//    |
//    = note: expected trait `for<'a> <for<'a> fn(&'a u64) -> impl Future<Output = ()> + 'a {sing} as FnOnce<(&'a u64,)>>`
//               found trait `for<'a> <for<'a> fn(&'a u64) -> impl Future<Output = ()> + 'a {sing} as FnOnce<(&'a u64,)>>`
// note: the lifetime requirement is introduced here
//   --> src/bin/try-it.rs:6:31
//    |
// 6  |     F: for<'a> Fn(&'a u64) -> Fut,
//    |                               ^^^
