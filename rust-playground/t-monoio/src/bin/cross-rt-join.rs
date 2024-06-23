//! Test if JoinHandle can be used in another thread.
//!
//! - Without enabling `sync` feature flag, `JoinHandle.await` sometimes blocks forever.
//! - With `sync` feature flag enabled, `JoinHandle.await` returns correctly. But block forever if the task panics.
//!
//! Thanks to @Miaxos
//! https://github.com/bytedance/monoio/issues/241#issuecomment-1950230596

use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    std::thread::spawn(move || {
        let mut rt = monoio::RuntimeBuilder::<monoio::FusionDriver>::new()
            .enable_all()
            .build()
            .expect("Failed building the Runtime");

        rt.block_on(async move {
            let fu = async move {
                // println!("inner-fu: ready");
                // panic!("inner-fu: panic");
                1u64
            };

            let handle = monoio::spawn(fu);
            tx.send(handle).unwrap();

            monoio::time::sleep(Duration::from_millis(1_000)).await;
            println!("outer-fu: after sending handle and sleep");
        });
    });

    let handle = rx.recv().unwrap();

    let mut rt = monoio::RuntimeBuilder::<monoio::FusionDriver>::new()
        .enable_all()
        .build()
        .expect("Failed building the Runtime");

    rt.block_on(async move {
        println!("joiner: before handle.await");
        let got = handle.await;
        println!("joiner: after handle.await: {:?}", got);
    });
}
