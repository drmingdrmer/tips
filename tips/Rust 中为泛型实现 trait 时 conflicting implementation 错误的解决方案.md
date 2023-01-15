title:: Rust 中为泛型实现 trait 时 conflicting implementation 错误的解决方案
type:: [[Tips]]
tags:: rust-programming, trait, generic, trait-conflict


```
error[E0119]:
conflicting implementations of trait `Str` for type `std::option::Option<&_>`

11 | impl<T: Str> Str for Option<T> {
   | ------------------------------ first implementation here
...
17 | impl<T: Str> Str for Option<&T> {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ 
            conflicting implementation for `std::option::Option<&_>`
```

**问题**: 假设我需要一个将 `T` 转成字符串的功能, 定义在一个 trait 里: `trait Str { fn to_str(&self) -> String }` , 并已经给一些常用类型, 例如 `u64` 实现了这个`trait`:

```rust
pub trait Str {
    fn to_str(&self) -> String;
}

impl Str for u64 {
    fn to_str(&self) -> String {
        format!("{}", self)
    }
}
```

这时我还希望对所有 `T: Str`, 也自动给如 `Option<T>` 和 `Option<&T>`
这样常见的相关类型也实现转字符串的功能:

```rust
impl<U: Str> Str for Option<U> {
    fn to_str(&self) -> String {
        self.as_ref().to_str()
    }
}

impl<V: Str> Str for Option<&V> {           // compile error!!!
    fn to_str(&self) -> String {
        self.map(|x| x.to_str()).unwrap_or("-".to_string())
    }
}

fn main() {
    println!("{}", 5u64.to_str());          // 5
    println!("{}", Some(&5u64).to_str());   // 5
    println!("{}", Some(5u64).to_str());    // 5
    println!("{}", None::<&u64>.to_str());  // -
}
```

但这时会报一个编译错误: 因为 `U` 是 `&V` 的超集, 即可能存在 `U==&V`, 造成 `trait Str` 在同一个类型上有2个不同的实现:

```
error[E0119]:
conflicting implementations of trait `Str` for type `std::option::Option<&_>`

11 | impl<T: Str> Str for Option<T> {
   | ------------------------------ first implementation here
...
17 | impl<T: Str> Str for Option<&T> {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ 
            conflicting implementation for `std::option::Option<&_>`
```

**要解决这个问题**, 即避免同一类型上的 trait 实现冲突, 就需要这2个 trait 的签名不同,
所以我们把目标类型也作为 trait 签名的一部分, 将其改写为: `Str<T>`,
这样, 即使存在 `U==&V` 时, `Option<U>` 实现的 trait 是 `Str<&V>`, `Option<&V>` 实现的 trait 是 `Str<V>`,
这是2个不同的 trait, 允许同时实现在同一个类型上, 达到我们所需. 于是最终通用 trait 实现为:

```rust
pub trait Str<T> {
    fn to_str(&self) -> String;
}

impl Str<u64> for u64 {
    fn to_str(&self) -> String {
        format!("{}", self)
    }
}

impl<U: Str<U>> Str<U> for Option<U> {
    fn to_str(&self) -> String {
        self.as_ref().to_str()
    }
}

impl<V: Str<V>> Str<V> for Option<&V> {      // it works!!!
    fn to_str(&self) -> String {
        self.map(|x| x.to_str()).unwrap_or("-".to_string())
    }
}

fn main() {
    println!("{}", 5u64.to_str());           // 5
    println!("{}", Some(&5u64).to_str());    // 5
    println!("{}", Some(5u64).to_str());     // 5
    println!("{}", None::<&u64>.to_str());   // -
}
```

- Play [conflict.rs online](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=70731f9785148d2980b537cd5cd42875)
- Play [solution.rs online](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=0afb51d262ec45ac191825f57aa55580)

Source files:

- [conflict.rs](https://github.com/drmingdrmer/tips/blob/main/rust-playground/src/bin/impl-trait-for-option-generic-conflict.rs)
- [solution.rs](https://github.com/drmingdrmer/tips/blob/main/rust-playground/src/bin/impl-trait-for-option-generic-solution.rs)
