tags:: tips, rust-programming, async, future, stream

用来创建Stream/Sink的工具分散在各个crate里,
`futures::stream`, `tokio_stream`, `tokio_util` ...


## crate `futures::stream`


### Crate `futures::stream` 提供了一些创建 `Stream` 的函数:

https://docs.rs/futures/latest/futures/stream/index.html

Trivial `Stream`:

- `abortable()`            Creates a new Abortable stream and an AbortHandle which can be used to stop it.
- `empty()`                Creates a stream which contains no elements.
- `once()`                 Creates a stream of a single element.
- `pending()`              Creates a stream which never returns any elements.

Non async:

- `iter()`                 Converts an Iterator into a Stream which is always ready to yield the next value.
- `repeat()`               Create a stream which produces the same item repeatedly.
- `repeat_with()`          Creates a new stream that repeats elements of type A endlessly by applying the provided closure, the repeater, F: FnMut() -> A.

Create by calling async `Fn`:

- `poll_fn()`              Creates a new stream wrapping a function returning Poll<Option<T>>.
- `poll_immediate()`       Creates a new stream that always immediately returns Poll::Ready when awaiting it.
- `try_unfold()`           Creates a TryStream from a seed and a closure returning a TryFuture.
- `unfold()`               Creates a Stream from a seed and a closure returning a Future.

Combination utilities:

- `select()`               This function will attempt to pull items from both streams. Each stream will be polled in a round-robin fashion, and whenever a stream is ready to yield an item that item is yielded.
- `select_all()`           Convert a list of streams into a Stream of results from the streams.
- `select_with_strategy()` This function will attempt to pull items from both streams. You provide a closure to tell SelectWithStrategy which stream to poll. The closure can store state on SelectWithStrategy to which it will receive a &mut on every invocation. This allows basing the strategy on prior choices.


## crate `tokio_stream`


### `tokio_stream::wrappers` 将 channel(`mpsc`等) 等转成 `Stream`:

https://docs.rs/tokio-stream/latest/tokio_stream/wrappers/index.html

Most used:

- `ReceiverStream`                             A wrapper around `tokio::sync::mpsc::Receiver` that implements Stream.
- `WatchStream`             sync               A wrapper around `tokio::sync::watch::Receiver` that implements Stream.

Other:

- `BroadcastStream`         sync               A wrapper around `tokio::sync::broadcast::Receiver` that implements Stream.
- `CtrlBreakStream`         Windows and signal A wrapper around CtrlBreak that implements Stream.
- `CtrlCStream`             Windows and signal A wrapper around CtrlC that implements Stream.
- `IntervalStream`          time               A wrapper around Interval that implements Stream.
- `LinesStream`             io-util            A wrapper around `tokio::io::Lines` that implements Stream.
- `ReadDirStream`           fs                 A wrapper around `tokio::fs::ReadDir` that implements Stream.
- `SignalStream`            Unix and signal    A wrapper around Signal that implements Stream.
- `SplitStream`             io-util            A wrapper around `tokio::io::Split` that implements Stream.
- `TcpListenerStream`       net                A wrapper around `TcpListener` that implements Stream.
- `UnboundedReceiverStream`                    A wrapper around `tokio::sync::mpsc::UnboundedReceiver` that implements Stream.
- `UnixListenerStreamUnix`  and net            A wrapper around `UnixListener` that implements Stream.


## crate `tokio_util`


- `codec`   codec           Adaptors from AsyncRead/AsyncWrite to Stream/Sink
- `compat`  compat          Compatibility between the tokio::io and futures-io versions of the AsyncRead and AsyncWrite traits.
- `context` rt              Tokio context aware futures utilities.
- `either`                  Module defining an Either type.
- `io`      io              Helpers for IO related tasks.
- `net`     net and codec   TCP/UDP/Unix helpers for tokio.
- `sync`                    Synchronization primitives
- `task`    rt              Extra utilities for spawning tasks
- `time`    time            Additional utilities for tracking time.
- `udp`     net and codec   UDP framing


### `tokio_util::compat`

- `Compat` A compatibility layer that allows conversion between the tokio::io and futures-io AsyncRead and AsyncWrite traits.


### `tokio_util::io`

- `CopyToBytes`           A helper that wraps a Sink<Bytes> and converts it into a `Sink<&'a [u8]>` by copying each byte slice into an owned Bytes.
- `InspectReader`         An adapter that lets you inspect the data that’s being read.
- `InspectWriter`         An adapter that lets you inspect the data that’s being written.
- `ReaderStream`          Convert an AsyncRead into a Stream of byte chunks.
- `SinkWriter`            Convert a Sink of byte chunks into an AsyncWrite.
- `StreamReader`          Convert a Stream of byte chunks into an AsyncRead.
- `SyncIoBridge`  io-util Use a tokio::io::AsyncRead synchronously as a std::io::Read or a tokio::io::AsyncWrite as a std::io::Write.


### `tokio_util::codec` 将 `tokio::AsyncRead/AsyncWrite` 转换成 Stream/Sink

https://docs.rs/tokio-util/0.7.8/tokio_util/codec/index.html

- `AnyDelimiterCodec` A simple Decoder and Encoder implementation that splits up data into chunks based on any character in the given delimiter string.
- `BytesCodec`        A simple Decoder and Encoder implementation that just ships bytes around.
- `LinesCodec`        A simple Decoder and Encoder implementation that splits up data into lines.

[framed-async-read-to-stream.rs](../rust-playground/src/bin/framed-async-read-to-stream.rs)

```rust
use futures_util::StreamExt;
use tokio_util::codec::{FramedRead, LinesCodec};

let mut framed = FramedRead::new(tokio::io::stdin(), LinesCodec::new());

while let Some(line) = framed.next().await {
    println!("{:?}", line);
}
```


### `tokio_util::sync` 将 `mpsc::Sender` 转成 `Sink`

https://docs.rs/tokio-util/latest/tokio_util/sync/index.html

- `CancellationToken`              A token which can be used to signal a cancellation request to one or more tasks.
- `DropGuard`                      A wrapper for cancellation token which automatically cancels it on drop. It is created using drop_guard method on the CancellationToken.
- `PollSemaphore`                  A wrapper around Semaphore that provides a poll_acquire method.
- `PollSender`                     A wrapper around mpsc::Sender that can be polled.
- `ReusableBoxFuture`              A reusable `Pin<Box<dyn Future<Output = T> + Send + 'a>>`.
- `WaitForCancellationFuture`      A Future that is resolved once the corresponding CancellationToken is cancelled.
- `WaitForCancellationFutureOwned` A Future that is resolved once the corresponding CancellationToken is cancelled.
