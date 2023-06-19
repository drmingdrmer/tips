tags:: tips, rust-programming, ref, borrow, pattern

在 Rust 的 pattern match 中, 使用`ref` 来创建引用, 可以减小引用的 scope,
从而解决一些直接使用`& foo` 无法解决的问题, 例如 `can not borrow mutable twice` 的问题.

这是因为 `match &foo {...}` 或 `if let Some(x) = &foo {}` 的语句中, `&foo`
的引用一直被持有直到 `match` 或 `if` 结束.

看代码:

```rust
struct Foo {
    writer: Option<u64>,
}

impl Foo {
    fn ref_keyword_is_ok(&mut self) -> &mut u64 {
        if let Some(ref mut w) = self.writer {  //
            w                                   // Scope of `&mut self.writer` ends here
        } else {                                //
            self.writer.insert(5)               //
        }                                       //
    }

    fn as_mut_does_not_compile(&mut self) -> &mut u64 {
        if let Some(w) = self.writer.as_mut() {  // 
            w                                    //
        } else {                                 //
            self.writer.insert(5)                //
        }                                        // Scope of `&mut self.writer` lasts as long as the if-let statement,
                                                 // and ends here
    }
}

fn main() {}
```

```
error[E0499]: cannot borrow `self.writer` as mutable more than once at a time
   |
15 |       async fn maybe_init_writer_that_does_not_compile(&mut self) -> &mut u64 {
   |  ______________________________________________________-______________________-
   | |                                                      |
   | |                                                      let's call the lifetime of this reference `'1`
16 | |         if let Some(writer) = self.writer.as_mut() {
   | |                               -------------------- first mutable borrow occurs here
17 | |             writer
18 | |         } else {
19 | |             self.writer.insert(5)
   | |             ^^^^^^^^^^^^^^^^^^^^^ second mutable borrow occurs here
20 | |         }
21 | |     }
   | |_____- returning this value requires that `self.writer` is borrowed for `'1`
```
