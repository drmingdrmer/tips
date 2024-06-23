use futures_util::StreamExt;
use std::future;

// #[tokio::main]
// async fn main() {
//     let strm = futures::stream::iter([1, 2, 3]);
//     let s2 = strm
//         .take_while(|x| future::ready(x < &2))
//         .map(|x| if x < 2 { Some(x) } else { None });
//     let got = s2.collect::<Vec<_>>().await;
//     println!("{:?}", got)
// }

use std::fmt;

struct Person {
    name: String,
    age: u32,
}

impl fmt::Debug for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Person")
            .field("name", &self.name)
            .field("age", &self.age)
            .finish()
    }
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    println!("{:?}", person);
    println!("{:#?}", person);
}
