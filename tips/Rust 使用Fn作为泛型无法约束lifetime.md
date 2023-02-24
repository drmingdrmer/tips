tags:: tips, rust-programming, generic, lifetime

Rust 中如果一个 struct 有一个泛型参数为 Fn 类型, 且这个 Fn 也返回一个泛型类型,
且跟Fn的参数有 lifetime 的约数, 那么是无法编译成功的.
个人对这个问题的理解是, 泛型定义(where)中, 无法把2个泛型参数使用 lifetime
关联起来; 而当这些泛型实例化时, 将实例类型填回泛型参数生成实际编译的代码后,
用lifetime约束再去检查时就会出现问题: 本该是互相约束的lifetime(Fut的和F的),
变成了不同的lifetime.

[fail](../rust-playground/src/bin/async-fn-as-generic.rs)

```rust
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
```

目前找到的一个解决方法依旧是pin-box一把梭:

[ok](../rust-playground/src/bin/async-fn-as-generic-pin-box.rs)

```rust
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
```
