//! This does not work:
//! ```ignore
//! struct Echoable<T> {
//!     _p: PhantomData<T>,
//! }
//! <(&&Echoable<u32>)>::echo();
//! // function or associated item not found in `&&Echoable<u32>
//! // No autoref
//! ```
use std::fmt::Debug;

struct Echoable<T>(T);

trait Echo {
    fn echo(&self);
}

impl Echo for &&Echoable<u64> {
    fn echo(&self) {
        println!("u64: {}", self.0);
    }
}

impl<T: Debug> Echo for &Echoable<T> {
    fn echo(&self) {
        println!("Debug: {:?}", self.0);
    }
}

impl<T> Echo for Echoable<T> {
    fn echo(&self) {
        println!("generic");
    }
}

struct Foo;

fn main() {
    (&&&Echoable(Foo)).echo(); //   generic
    (&&&Echoable("str")).echo(); // Debug: "str"
    (&&&Echoable(1u64)).echo(); //  u64: 1
    (&&Echoable(1u64)).echo(); //   Debug: 1
    (&Echoable(1u64)).echo(); //    generic
}
