## Rust 中 `#[thread_local]` 的drop方法不被调用

tags: #rust #thread-local #drop #trap

Rust 中有2种方法声明 [thread-local](https://en.wikipedia.org/wiki/Thread-local_storage) 变量: 使用标准库的宏 `thread_local!{...}` 或使用 attribute `#[thread_local]...`, 这里有个不**rust**的地方, `#[thread_local]` 按官方说法是被"translates directly to the `thread_local` attribute in LLVM", 线程销毁时不会调用它的`drop`方法, 但宏声明的thread-local变量没问题:

**使用宏 `thread_local!{...}` 定义, 正常调用了 `drop`**:

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
