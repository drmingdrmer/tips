use futures::future::FutureExt;
use std::pin::pin;

#[tokio::main]
async fn main() {
    // Use select()
    {
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<()>();

        let fu = rx.recv();
        let sleep = tokio::time::sleep(std::time::Duration::from_secs(1));

        let fpin = pin!(fu);
        let spin = pin!(sleep);

        futures::future::select(fpin, spin).await;
    }

    // Use futures::select_biased!()
    {
        let (_tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<()>();

        let sleep = tokio::time::sleep(std::time::Duration::from_secs(1));

        loop {
            futures::select_biased!(
                _ = rx.recv().fuse() => {
                    println!("recv completed");
                    break;
                }
                _ = sleep.fuse() => {
                    println!("sleep completed");
                    break;
                }
            );
        }
    }
}
