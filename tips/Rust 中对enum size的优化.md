tags:: tips, rust-programming, enum, memory

`enum` 的内存大小一般是它最大的 variant 的大小, 加上一个 `tag` 的大小,
tag 是 enum 内部用于区分当前是哪个 variant. 因为 tag 需要参与计算 enum 的内存对齐,
所以如果 enum 中最大的 variant 是一个 8 bytes 的指针, 那么它的大小理论上是对齐到 8 bytes 的 tag 加
8 bytes 的指针, 共16 bytes:

```rust
// 8 bytes pointer
Arc1(Arc<i32>)

// 16 bytes = 8 bytes tag + union of two 8 bytes pointers
//
// memory layout: [8 bytes tag][union [8 bytes pointer],
//                                    [8 bytes pointer]]
enum Arc_Arc {
    Arc1(Arc<i64>),
    Arc2(Arc<i32>),
}
```

在实现上, Rust 会为特殊的 variant 类型做优化以减少 tag 的空间开销,
如果:
- enum 只有2个 variant;
- 其中一个 variant 在某段内存上一定**非0**,
- 另一个 variant 在同样一段内存上一定**是0**,

那么 Rust 就不会为这个 enum 添加tag,
而直接使用这段内存 **是否是0** 来判断 enum 当前是哪个variant.

例如, `Arc<i32>` 是 8 bytes 的非空指针,  因为 `Blank` 不包含数据, 可以认为是全0的值,
所以下面这个 enum 不需要 tag:

```rust
// size: 8; without tag
enum Blank_Arc {
    Blank,
    Arc(Arc<i32>),
}
```

再如, `Arc<i32>` 是1个 8 bytes 指针, `Arc<dyn Trait>` 是2个 8 bytes 指针(data
pointer和vtable pointer),
在它们2个的内存布局上有一个 8 bytes 分别是全0和非全0的. 所以这样一个 enum
也不需要 tag:

```rust
// size: 16; without tag
enum Arc_ArcDyn {
    Arc(Arc<i32>),
    ArcDyn(Arc<dyn Trait>),
}
```


[enum-size.rs](../rust-playground/src/bin/enum-size.rs)
