//! Provide methods via `AsRef` trait.

trait RemoteVote {
    fn remote_vote(&self) -> u64;
}

impl<T> RemoteVote for T
where
    T: AsRef<VoteStore>,
{
    fn remote_vote(&self) -> u64 {
        self.as_ref().remote_vote()
    }
}

struct VoteStore {
    vote: u64,
}

impl RemoteVote for VoteStore {
    fn remote_vote(&self) -> u64 {
        self.vote
    }
}

trait RemoteLog {
    fn log_index(&self) -> u32;
}

impl<T> RemoteLog for T
where
    T: AsRef<LogStore>,
{
    fn log_index(&self) -> u32 {
        self.as_ref().log_index()
    }
}

struct LogStore {
    log: u32,
}

impl RemoteLog for LogStore {
    fn log_index(&self) -> u32 {
        self.log
    }
}

#[derive(derive_more::AsRef)]
struct My {
    #[as_ref]
    vote_store: VoteStore,
    #[as_ref]
    log_store: LogStore,
}

fn main() {
    let my = My {
        vote_store: VoteStore { vote: 3 },
        log_store: LogStore { log: 4 },
    };

    println!("{}", my.remote_vote());
    println!("{}", my.log_index());
}
