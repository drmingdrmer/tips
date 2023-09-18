tags:: tips, rust-programming, generic, rust-borrow, reborrow, borrow-of-moved

Rust 编译器会尽量自动对形如 `write(v: &mut Foo)` 的调用进行参数的 **reborrow**: `&mut *v`, 来创建一个临时的 `&mut Foo`，以允许多次调用。但当遇到泛型时，Rust 默认将 `&mut Foo` 直接 move 给目标函数，因此可能导致 `borrow of moved value` 的错误。

#### 默认对 `&mut V` 的使用自动进行 **reborrow**:

下面的代码中 `write(v)` reborrow 后被翻译成 `write(&mut *v)`; reborrow 产生一个临时的 `&mut Foo` move 给目标函数, 所以多次调用 `write(v)` 没有问题的:

```rust
fn good(v: &mut Foo) {
    write(v); // ---> write(&mut *v)
    write(v); // ---> write(&mut *v)
}

fn write(_v: &mut Foo) {}
````

#### 泛型默认使用 move

而遇到 **泛型** 的时候, 泛型可能需要比被调用函数更长的 lifetime, 所以 Rust 这时默认不进行 reborrow, 例如下面 `generic_write(v)` 调用时直接将 `&mut Foo` move 给目标函数, 所以后续代码会产生 **`borrow of moved value`** 的错误. (可变引用 `&mut Foo` 显然不能允许 Copy, 如果也不能 reborrow, 最后只能 move 了):

```rust
// error[E0382]: borrow of moved value: `v`
// |
// | fn borrow_of_moved_error(v: &mut Foo) {
// |                          - move occurs because `v` has type `&mut Foo`,
// |                            which does not implement the `Copy` trait
// |     generic_write(v);
// |                   - value moved here
// |     write(v);
// |           ^ value borrowed here after move
fn borrow_of_moved_error(v: &mut Foo) {
    generic_write(v); // ---> generic_write(v);
    write(v);         // ---> write(&mut *v);
}

fn generic_write<V>(_v: V) {}
fn write(_v: &mut Foo) {}
````

#### 带泛型时需要手动 reborrow

解决这个问题的方法是使用 `&mut *v` 手动进行 reborrow, 或强制指定泛型类型:

```rust
fn solution(v: &mut Foo) {
    generic_write(&mut *v); // Manually reborrow
    write(v);
}

fn solution_2(v: &mut Foo) {
    generic_write::<&mut Foo>(v); // Specify the type for generic type
    write(v);
}

fn generic_write<V>(_v: V) {}
fn write(_v: &mut Foo) {}
```

---

- 代码例子: [manually-reborrow.rs](../rust-playground/src/bin/borrow-of-moved-reborrow.rs)
- Issue: https://github.com/rust-lang/rust/issues/62112
- 感谢帮忙解决问题的懂哥们:
  [walter](https://github.com/w41ter) @github
  [Winter Zhang](https://github.com/zhang2014) @github
  [Xuanwo](https://github.com/Xuanwo) @github


