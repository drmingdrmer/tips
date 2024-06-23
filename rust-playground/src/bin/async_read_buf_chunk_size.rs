use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let path = "/Users/drdrxp/foo-data";
    let path_2 = "/Users/drdrxp/foo-data-2";
    let mut f = fs::OpenOptions::new()
        // .create_new(true)
        // .write(true)
        .read(true)
        .open(&path)
        .await?;

    let mut buf: Vec<u8> = Vec::with_capacity(1024 * 1024 * 10);

    while buf.capacity() > buf.len() {
        let c = f.read_buf(&mut buf).await?;
        if c == 0 {
            break;
        }
    }

    let mut f = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .read(true)
        .open(&path_2)
        .await?;

    f.write_all(&buf).await?;

    println!("n={}", buf.len());
    Ok(())
}
