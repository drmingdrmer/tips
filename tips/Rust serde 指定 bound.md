tags:: tips, rust-programming, serde

serde bound tutorial: https://serde.rs/attr-bound.html

感谢热心网友 https://github.com/tvsfx 详细解释了这个问题: https://github.com/datafuselabs/openraft/pull/993#issuecomment-1906027920

## 去掉多余的自动生成的 trait bound

本文使用的源码:
[serde-bound.rs](../rust-playground/src/bin/serde-bound.rs)

使用 `#[derive(Serialize, Deserialize)]` 时,
Rust 会帮我们为泛型参数添加一个 `where T: Serialize + Deserialize<'_>` 的 trait bound,
但有时会遇到自动生成的条件约束无法正确匹配到类型的问题,
例如下面这个例子中, `NodeId` 本身已经指定了需要有 `Serialize` 和 `Deserialize` 实现,
而 `#[derive(Serialize, Deserialize)]` 又会给 `Fatal` 增加一个 `where NID: Serialize + Deserialize<'_>` 的约束,
导致重复的约束的编译错误:


```rust
pub trait NodeId: serde::Serialize + for<'a> serde::Deserialize<'a> {}

#[derive(serde::Deserialize, serde::Serialize)]
pub enum Fatal<NID>
where NID: NodeId,
{
    StorageError(NID),
}

// 8  | pub enum Fatal<NID>
//    |          ^^^^^^^^^^
//    |
// note: multiple `impl`s or `where` clauses satisfying `NID: Deserialize<'_>` found
```

这时我们需要用 `#[serde(bound="")]` 告诉 `derive`, 不要再自动产生 `Serialize + Deserialize` 的约束了:

```rust
pub trait NodeId: serde::Serialize + for<'a> serde::Deserialize<'a> {}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(bound = "")] // <---- clear auto trait bound
pub enum Fatal<NID>
where NID: NodeId,
{
    StorageError(NID),
}
```

## 指定所需的trait bound

但是 `#[serde(bound="")]` 并不能解决更复杂一点的情况, 例如下面 `NID: NodeId`
本身已经指定了 `Serialize + Deserialize<'_>`, 可以用 `#[serde(bound="")]`
来去掉自动生成的 trait bound, 但是`E: std::error::Error` 并不包括 `Serialize + Deserialize<'_>`,
它仍然需要添加额外的 trait bound 约束:


```rust
pub trait NodeId: serde::Serialize + for<'a> serde::Deserialize<'a> {}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(bound = "")]
struct MyError<NID, E>
where NID: NodeId,
      E: std::error::Error,
{
    nid: NID,
    err: E,
}
// 24   |     err: E,
//      |        ^ the trait `Deserialize<'_>` is not implemented for `E`
//      |
// note: required by a bound in `next_element`
```

这时需要手动指定 `NID` 和 `E` 的 trait bound:

```rust
pub trait NodeId: serde::Serialize + for<'a> serde::Deserialize<'a> {}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(bound(serialize = "E: serde::Serialize"))]
#[serde(bound(deserialize = "E: for <'d> serde::Deserialize<'d>"))]
struct MyError<NID, E>
where NID: NodeId,
      E: std::error::Error,
{
    nid: NID,
    err: E,
}
```

上面的代码表示,
- 对 `serialize` 的实现, 要求 `E` 实现 `serde::Serialize`,
- 对 `deserialize` 的实现, 要求 `E` 实现 `serde::Deserialize<'d'>`.

## trait bound 写成一行带来的问题

这里也可以写成一行: `[serde(bound = "E: serde::Serialize + for <'d> serde::Deserialize<'d>")]`,
但是这样就让 `Serialize` 和 `Deserialize` 的约束绑定到一起了, 带来的问题是,
实现 `Serialize` 时要求**不必要**的 `Deserialize`, 实现 `Deserialize` 时要求**不必要**的 `Serialize` .

使用 `cargo expand --bin serde-bound --tests` 查看两种模式下生成的代码:

分开写
`#[serde(bound(serialize = "E: serde::Serialize"))]
#[serde(bound(deserialize = "E: for <'d> serde::Deserialize<'d>"))]`:

```rust
impl<NID, E> _serde::Serialize for MyError<NID, E>
where NID: NodeId,
    E: std::error::Error,
    E: serde::Serialize {} // <---- just Serialize, good

impl<'de, NID, E> _serde::Deserialize<'de> for MyError<NID, E>
where NID: NodeId,
    E: std::error::Error,
    E: for<'d> serde::Deserialize<'d>{} // <---- just Deserialize, good
```

写成一行时
`#[serde(bound = "E: serde::Serialize + for <'d> serde::Deserialize<'d>")]`:

```rust
impl<NID, E> _serde::Serialize for MyError<NID, E>
where NID: NodeId,
    E: std::error::Error,
    E: serde::Serialize + for<'d> serde::Deserialize<'d>, // <---- Deserialize is not necessary

impl<'de, NID, E> _serde::Deserialize<'de> for MyError<NID, E>
where NID: NodeId,
    E: std::error::Error,
    E: serde::Serialize + for<'d> serde::Deserialize<'d>, // <---- Serialize is not necessary
```
