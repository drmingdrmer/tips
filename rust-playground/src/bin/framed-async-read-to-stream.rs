use futures_util::StreamExt;
use tokio_util::codec::{FramedRead, LinesCodec};

#[tokio::main]
async fn main() {
    // Convert AsyncRead into a line based FramedRead
    let mut framed = FramedRead::new(tokio::io::stdin(), LinesCodec::new());

    while let Some(line) = framed.next().await {
        println!("{:?}", line);
    }
}
