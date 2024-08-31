use tokio::runtime::Runtime;
use tokio::task;

use std::thread;
use std::time::Duration;

fn main() {
    let runtime = Runtime::new().unwrap();

    runtime.block_on(async move {
        task::spawn_blocking(move || {
            thread::sleep(Duration::from_secs(10_000));
            println!("fnishing up");
        });
    });

    runtime.shutdown_timeout(Duration::from_millis(100));
}
