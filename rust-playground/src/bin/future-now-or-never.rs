use futures::TryStream;
use futures_util::FutureExt;
use std::fmt::Debug;
use std::io;
use std::ops::RangeBounds;

fn main() {
    use futures::{future::pending, future::ready};
    let mut future_ready = ready("foobar");
    let mut future_pending = pending::<&'static str>();

    assert_eq!((&mut future_ready).now_or_never(), Some("foobar"));
    assert_eq!((&mut future_pending).now_or_never(), None);
    assert_eq!((&mut future_pending).now_or_never(), None);
}

trait OptionalSend {}
trait OptionalSync {}
trait RaftTypeConfig {
    type NodeId;
    type Entry;
}

pub trait RaftLogReader<C>: OptionalSend + OptionalSync + 'static + Send
where
    C: RaftTypeConfig,
{
    fn entries_stream(
        &mut self,
        start: u64,
    ) -> impl std::future::Future<
        Output = Result<impl TryStream<Ok = C::Entry, Error = ()>, io::Error>,
    > + Send {
        async move { Ok(futures::stream::iter(vec![])) }
    }
}
