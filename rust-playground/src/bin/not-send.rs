use std::marker::PhantomData;

fn accept_send<T: Send>(_t: T) {}

#[derive(Default)]
struct NotSend {
    p: PhantomData<*const ()>,
}

struct Foo<T: Types> {
    c: T::Inner,
    p: PhantomData<T>,
}

trait Types: Send + Sync {
    type Inner;
}

struct Config;

impl Types for Config {
    type Inner = NotSend;
}

impl Types for NotSend {
    type Inner = ();
}

fn main() {
    let a = NotSend {
        p: PhantomData::default(),
    };

    accept_send(Foo::<NotSend> {
        c: Default::default(),
        p: PhantomData::default(),
    });
}
