struct Foo<'d> {
    d: &'d (),
}

fn foo<T: 'static>(v: T) -> T {
    v
}

trait Doit {
    fn doit(&self) {}
}

impl<T> Doit for T where T: Send + Sync + 'static {}

fn main() {
    let f = Foo { d: &() };
    let f = foo(f);
    f.doit();
}
