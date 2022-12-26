struct Foo(usize);
impl Drop for Foo {
    fn drop(&mut self) {
        println!("dropped");
    }
}

thread_local! {
    static MACRO_TLS: std::cell::RefCell<Foo> = std::cell::RefCell::new(Foo(0));
}

fn main() {
    let _ = std::thread::spawn(|| unsafe {
        MACRO_TLS.with(|f| {
            println!("foo: {}", f.borrow_mut().0);
        });
    })
    .join();
}
