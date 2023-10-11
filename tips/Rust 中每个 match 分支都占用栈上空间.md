tags:: tips, rust-programming, stack, match, overflow

[match-arm-stack-size.rs](../rust-playground/src/bin/match-arm-stack-size.rs)

Rust 中 match 的每个分支都会被分配一块 stack 上的内存, 并且这些内存不是公用的,
也就是说，如果 `outer` 函数里面有多个 `match` 的分支，并且每个分支都会分配一些栈上内存的话，
那 outer 调用的另一个 inner 函数的栈空间就会排在所有这些被占用的内存之后。

所以当这种分配了较多内存的 match 分支的函数被调用很多层之后，就非常容易出现栈溢出。

解决方法是把 match arm 放到另一个函数里.

```rust
static mut RSP: isize = 0;

fn main() {
    print_stack_pos("main");
    outer();
}

fn outer() {
    let v = 0;
    match v {
        _ if v == 0 => inner(),
        _ if v == 1 => [0u8; 1024][0],
        _ if v == 2 => [0u8; 1024 * 2][0],
        _ => 0,
    };
}

fn inner() -> u8 {
    print_stack_pos("inner");
    0
}

fn print_stack_pos(msg: impl Display) {
    let a = ();
    let ptr = &a as *const _ as isize;

    let prev = unsafe { RSP };
    println!("{:>5} stack ptr: {:x}, diff: {}", msg, ptr, prev - ptr);
    unsafe { RSP = ptr; }
}
```

上面的代码会打印出调用链 `main() -> outer() -> inner()` 过程中 `main()` 和 `inner()` 的栈位置相差大约3KB,
因为outer() 的 match 分支 **一共** 分配了3KB内存.

```
 main stack ptr: 16fd61ff7, diff: -6171271159
inner stack ptr: 16fd613b7, diff: 3136
```

不管是函数还是 block 都提供了一个作用域，但是函数作用域会改变 stack 顶的位置，而 block 的作用域不会改变 stack 顶的位置，所有的变量在 stack 上的位置是在进入函数时就已经分配好的, 不论是否会被使用.
例如如下代码 `inner()` 也会打出3KB大小的 stack 位置变化.

```rust
fn outer_blocks() {
    { inner(); }
    { let _c = [0u8; 1024]; }
    { let _c = [0u8; 1024 * 2]; }
}
```
