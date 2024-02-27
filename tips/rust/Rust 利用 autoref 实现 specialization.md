tags:: tips, rust-programming, specialization, autoref

`#[feature(specialization)]` 和 `#[feature(min_specialization)]` 还没stablize,
如果一个类型 `T` 需要根据它实现了哪些 trait 来做分发, 可以利用 Rust 中的 autoref 特性来实现.

例如下面的 `trait Echo {fn echo(&self)}` 依次匹配以下规则, 调用最先匹配到的:
- 如果 `T` 是 `u64` 类型, 打印: `u64: xx`,
- 如果 `T`  实现了 `Debug` 的类型, 打印: `Debug: xx`,
- 对其他类型的 `T`, 只打印: `generic`

我们无法直接对 `T` 实现 specialization 特性, 但可以给它的一个 wrapper `Echoable(T)` 实现:

[specialization-with-autoref.rs](../rust-playground/src/bin/specialization-with-autoref.rs)

```rust
trait Echo { fn echo(&self); }

struct Echoable<T>(T);

impl           Echo for &&Echoable<u64> { fn echo(&self) { println!("u64: {}", self.0); } }
impl<T: Debug> Echo for  &Echoable<T>   { fn echo(&self) { println!("Debug: {:?}", self.0); } }
impl<T>        Echo for   Echoable<T>   { fn echo(&self) { println!("generic"); } }

fn main() {
    struct Foo;

    (&&&Echoable(Foo)  ).echo(); //  generic
    (&&&Echoable("str")).echo(); //  Debug: "str"
    (&&&Echoable(1u64) ).echo(); //  u64: 1
    ( &&Echoable(1u64) ).echo(); //  Debug: 1
    (  &Echoable(1u64) ).echo(); //  generic
}
```

- 上面代码中, 使用 `Echoable` 做为一个 wrapper 为 `T` 实现支持 `specialization` 的 `Echo` trait;
- 当使用 `&&&Echoable(T).echo()` 时, 当前类型如果实现了 `Echo` trait,
  则调用对应的 `echo()` 方法, 否则去掉一个 `&` 继续查找, 直到找到或报错.


参考:
- Generalized Autoref-Based Specialization: https://lukaskalbertodt.github.io/2019/12/05/generalized-autoref-based-specialization.html
- Autoref-based stable specialization https://github.com/dtolnay/case-studies/tree/master/autoref-specialization#realistic-application




