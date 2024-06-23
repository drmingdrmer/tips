use std::cell::Cell;

struct Bar {
    i: u64,
    j: u64,
}

struct Foo {
    a: Cell<Bar>,
}

fn doit(x: &Foo) {
    let mut a = x.a.get_mut();
    a.i = 3;
}

fn main() {
    let x = Foo {
        a: Cell::new(Bar { i: 1, j: 2 }),
    };
}
