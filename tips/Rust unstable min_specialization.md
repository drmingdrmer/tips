tags:: tips, rust-programming, unstable,  trait, min_specialization

启用 `min_specialization` 之后, 可以为 trait 提供一个默认(default)实现, 然后为多个特定 type 提供特化(specialized)实现.
当 trait 被特定 type 实例化时, specialized 实现将优先于 default 实现, 允许更精确地控制 type 的行为.
这种方法在需要根据不同 type 特性来调整代码逻辑时非常有用.

例子:
定义一个名为 `Trait` 的 trait , 并为任意 type `T` 提供一个 **default** 实现;
接下来, 为`u64` type 提供了一个 **speicliazed** 的实现. 它不会影响 `T` 的默认实现.

```rust
trait Trait {
    fn desc(&self) -> String;
}

/* default impl */
impl<T> Trait for T {
    default fn desc(&self) -> String {
        "something".to_string()
    }
}

/* specialized impl */
impl Trait for u64 {
    fn desc(&self) -> String {
        format!("{}", self)
    }
}

fn main() {
    println!("{}", 1u16.desc()); // "something"
    println!("{}", 1u32.desc()); // "something"
    println!("{}", 1u64.desc()); // "1"
}
```

[](../rust-playground/src/bin/min-specialization.rs)
[](../rust-playground/src/bin/specialization.rs)
