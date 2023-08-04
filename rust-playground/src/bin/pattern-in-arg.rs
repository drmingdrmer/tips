struct Foo {
    a: u64,
    b: u64,
}

fn doit(Foo { a, .. }: Foo) {
    println!("{a}");
}

fn main() {
    doit(Foo { a: 3, b: 4 });
}
