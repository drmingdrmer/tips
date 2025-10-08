tags:: tips, rust-programming, trait, seal

**这个方法不对，用户完全可以不写 where 而摆脱这个限制:**

```rust
impl foo::Trait for Implementor {
    fn unimplementable(&self)
    // where
    //     Self: foo::sealed::Sealed,
    {
        todo!()
    }
}
```

Rust 中的**部分可实现** Trait

允许用户实现 public trait，但某些方法可能已有默认实现且不允许被覆盖。这种设计有两个主要目的：

1. 用户专注实现核心功能
2. 系统在用户实现的基础上提供安全封装

示例代码: [non-implementable-trait-method.rs](../../rust-playground/src/bin/non-implementable-trait-method.rs)

实现机制

Rust 使用 sealed trait 技巧来实现这一功能。以下是具体实现方法：

1. 定义公共 trait 和私有 sealed trait

```rust
pub trait Trait {
    fn unimplementable(&self)
    where Self: sealed::Sealed
    {
        self.user_impl_this();
    }

    fn user_impl_this(&self);
}

mod sealed {
    pub trait Sealed {}
    impl<T> Sealed for T {}
}
```

这里，`Trait` 包含两个方法：

- `user_impl_this`：用户需要实现的方法
- `unimplementable`：用户无法覆盖的方法

2. 用户实现

```rust
pub struct Implementor;

impl foo::Trait for Implementor {
    fn user_impl_this(&self) {
        println!("user_impl_this");
    }
}
```

用户只能实现 `user_impl_this` 方法，无法修改 `unimplementable` 方法。

工作原理

- `sealed::Sealed` trait 是私有的，用户无法在外部引用它。
- `unimplementable` 方法要引用 `sealed::Sealed`，因此用户无法实现这个方法。
  用户试图实现 `unimplementable` 方法时，会因无法访问私有的 `sealed` 模块而失败。
- 系统为所有类型实现了 `Sealed`，使 `unimplementable` 方法可用。
