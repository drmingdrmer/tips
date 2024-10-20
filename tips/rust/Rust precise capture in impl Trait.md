tags:: tips, rust-programming, trait, capture


Rust 1.82.0 以前,
这种一个return position Trait 捕捉多个lifetime是无法实现的,
例如有了&self再捕捉别的引用的Future整不了:

```rust
#[derive(Debug)]
struct Foo;

impl Foo {
    fn echo<'a, 'v>(&'a self, val: &'v str) -> impl Future<Output = ()> + 'v {
        async move {
            println!("{}: {:?}", val, self);
        }
    }
}
```

`impl Future` 后面不论是 `+'v` 还是 `+'a` 都不行,
因为实际上a和v是没有outlive关系的.

然后现在可以了, 1.82 以后, 可以使用`use`:

```rust
impl Foo {
    fn echo<'a, 'v>(&'a self, val: &'v str) -> impl Future<Output = ()> + use<'v, 'a> {}
}
```
https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=eaff31898429b430092ee7a3e1009c4b


Precise capturing `use<..>` syntax:
https://blog.rust-lang.org/2024/10/17/Rust-1.82.0.html#precise-capturing-use-syntax

```text
//@ edition: 2021
fn f(x: &()) -> impl Sized { x }
```

```text
error[E0700]: hidden type for `impl Sized` captures lifetime that does not appear in bounds
 --> src/main.rs:1:30
  |
1 | fn f(x: &()) -> impl Sized { x }
  |         ---     ----------   ^
  |         |       |
  |         |       opaque type defined here
  |         hidden type `&()` captures the anonymous lifetime defined here
  |
help: add a `use<...>` bound to explicitly capture `'_`
  |
1 | fn f(x: &()) -> impl Sized + use<'_> { x }
  |                            +++++++++
```
