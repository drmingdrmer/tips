#![feature(thread_local)]

struct Foo(usize);
impl Drop for Foo {
    fn drop(&mut self) {
        println!("dropped");
    }
}

#[thread_local]
static mut ATTR_TLS: Foo = Foo(0);

fn main() {
    let _ = std::thread::spawn(|| unsafe {
        println!("foo: {}", ATTR_TLS.0);
    })
    .join();
}
