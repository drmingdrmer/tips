tags:: tips, rust-programming, lifetime, boxstream

错误:
```text
error: higher-ranked lifetime error
  --> src/bin/box-stream-to-impl-void-lifetime-error.rs:28:5
   |
28 | /     async move {
29 | |         stream_snapshot(strm).await;
30 | |     }
   | |_____^

```

[box-stream-to-impl-void-lifetime-error.rs](../../rust-playground/src/bin/box-stream-to-impl-void-lifetime-error.rs)
