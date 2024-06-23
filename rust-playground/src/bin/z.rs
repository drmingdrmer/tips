struct Foo {
    f: u64,
}
struct Goo<'g> {
    g: &'g u64,
}

fn doit<'g, 'f>(foo: &'f Foo) -> Goo<'g> {
    Goo { g: &foo.f }
}

fn it<'g>(g: &'g Goo) -> impl Iterator<Item = &'g u64> {
    std::iter::once(&3)
}

fn main() {
    let a = Foo { f: 3 };
    {
        let b;
        {
            let r = &a;
            b = Goo { g: &r.f };
        }
        println!("{}", b.g);
    }

    // let b = a.doit();
}
