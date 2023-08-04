tags:: tips, rust-programming, pattern

Rust 中的 pattern match 可以用在 `match` 中, `let`, `if-let`中, 以及函数参数中.

Rust 函数签名中可以直接 destructure 一个 struct(或其他):

```rust
struct Foo {
    a: u64,
    b: u64,
}

fn doit(Foo { a, .. }: Foo) {
    println!("{a}");
}

fn main() {
    doit(Foo { a: 3, b: 4 });
}
```


[pattern-in-arg](../rust-playground/src/bin/pattern-in-arg.rs)
