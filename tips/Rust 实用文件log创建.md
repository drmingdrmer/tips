tags:: tips, rust-programming, log

- 首先创建一个滚动切换的文件 `RollingFileAppender`, 可以实现最基本的 Write trait;

- 但是文件写会阻塞, 影响日志写入性能,
  于是第二步是把它封装到一个 `NonBlocking` 中,  NonBlocking 通过一个 channel 连接写入端
  和接受端, 如果接受端满了就丢弃消息. 所以 NonBlocking 虽然很快, 但负载过高会丢消息.

- 现在这个log写入接口对单笔写入比较高效,
    但整体还是低效, 因为日志写入端会对一行日志的每个小片段都调用一次 `Write::write` 接口.
    例如一段json `{"a":1}` 可能会分成 `{`, `"a"`, `:`, `1`, `}`, 调用5次
    `write()`.
    所以最后前端要加个 `BufWriter`, 成批写入, 提升效率.

[Databend log writer](https://github.com/datafuselabs/fuse-query/blob/11d0057da12f2a1273431723767afbc0ff819f32/src/common/tracing/src/minitrace.rs#L321-L320)

```rust
/// Create a `BufWriter<NonBlocking>` for a rolling file logger.
///
/// `BufWriter` collects log segments into a whole before sending to underlying writer.
/// `NonBlocking` sends log to another thread to execute the write IO to avoid blocking the thread
/// that calls `log`.
///
/// Note that `NonBlocking` will discard logs if there are too many `io::Write::write(NonBlocking)`,
/// especially when `fern` sends log segments one by one to the `Writer`.
/// Therefore a `BufWriter` is used to reduce the number of `io::Write::write(NonBlocking)`.
fn new_file_log_writer(dir: &str, name: impl ToString) -> (BufWriter<NonBlocking>, WorkerGuard) {
    let rolling = RollingFileAppender::new(Rotation::HOURLY, dir, name.to_string());
    let (non_blocking, flush_guard) = tracing_appender::non_blocking(rolling);
    let buffered_non_blocking = io::BufWriter::with_capacity(64 * 1024 * 1024, non_blocking);

    (buffered_non_blocking, flush_guard)
}
```
