use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    std::thread::spawn(move || {
        monoio::RuntimeBuilder::<monoio::FusionDriver>::new()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(async move {
                let fu = async move {
                    monoio::time::sleep(Duration::from_millis(500)).await;
                    println!("inner-fu: before panic!");
                    // panic!("test");
                    1u64
                };

                println!("outer-fu: before spawn");
                let handle = monoio::spawn(fu);
                tx.send(handle).unwrap();
                println!("outer-fu: after tx.send");

                monoio::time::sleep(Duration::from_millis(1_000)).await;
                println!("outer-fu: after sleep");
            });

        println!("thread-1: after block_on");
    });

    // std::thread::sleep(Duration::from_millis(2_000));

    let handle = rx.recv().unwrap();

    println!("main: before block_on");
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
