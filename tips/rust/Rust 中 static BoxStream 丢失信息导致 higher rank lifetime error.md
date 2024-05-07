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

online:
https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=8c382b5a6d932aaf81815f3825efd5ed
