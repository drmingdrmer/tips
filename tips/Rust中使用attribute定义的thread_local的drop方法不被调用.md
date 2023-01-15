type:: [[Tips]]
tags:: rust-programming, thread-local, drop, trap


Rust 中有2种方法声明 [thread-local](https://en.wikipedia.org/wiki/Thread-local_storage) 变量: 使用标准库的宏 [`thread_local!{}`](https://doc.rust-lang.org/std/macro.thread_local.html) 或使用 attribute [`#[thread_local]`](https://doc.rust-lang.org/beta/unstable-book/language-features/thread-local.html), 这里有个不**rust**的地方, `#[thread_local]` 按官方说法是被"translates directly to the `thread_local` attribute in LLVM", 线程销毁时不会调用它的`drop`方法, 但宏声明的thread-local变量没问题:

**使用宏 `thread_local!{...}` 定义, 正常调用了 `drop`**:

[Online Playground](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021&gist=998357c6308a37ea4a53300843d38fae)

[thread_local_macro_drop.rs](../rust-playground/src/bin/thread_local_macro_drop.rs)

```rust
struct Foo(usize);
impl Drop for Foo {
    fn drop(&mut self) { println!("dropped"); }
}

thread_local! {
    static MACRO_TLS: std::cell::RefCell<Foo> = std::cell::RefCell::new(Foo(0));
}

fn main() {
    let _ = std::thread::spawn(|| unsafe {
        MACRO_TLS.with(|f| {
            println!("foo: {}", f.borrow_mut().0);
        });
    })
    .join();
}
// Output:
// foo: 0
// dropped
```

**使用 attribute `#[thread_local]` 定义, 没有调用 `drop`**:

[Online Playground](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021&gist=a419e002b5b8d462dde9ceea93dc6e7f)

[thread_local_attr_not_dropped.rs](../rust-playground/src/bin/thread_local_attr_not_dropped.rs)

```rust
#![feature(thread_local)]
struct Foo(usize);
impl Drop for Foo {
    fn drop(&mut self) { println!("dropped"); }
}

#[thread_local]
static mut ATTR_TLS: Foo = Foo(0);

fn main() {
    let _ = std::thread::spawn(|| unsafe {
        println!("foo: {}", ATTR_TLS.0);
    })
    .join();
}
// Output:
// foo: 0
```
