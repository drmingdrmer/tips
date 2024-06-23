#![feature(try_blocks)]
fn main() -> anyhow::Result<()> {
    let x: Result<(), u64> = try {
        println!("Hello, world!");
        return Err(anyhow::anyhow!("error"));
    };

    println!("foo");
    Ok(())
}
