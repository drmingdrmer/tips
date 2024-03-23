trait Trait {
    fn trait_ids<'s>(&'s self) -> impl Iterator<Item = u64> + 'static;
}

struct Foo {
    ids: Vec<u64>,
}

impl Foo {
    fn ids<'s>(&'s self) -> impl Iterator<Item = u64> + 'static {
        self.ids.clone().into_iter()
    }
}

impl Trait for Foo {
    fn trait_ids<'s>(&'s self) -> impl Iterator<Item = u64> + 'static {
        self.ids.clone().into_iter()
    }
}

impl<'f> Trait for &'f Foo {
    fn trait_ids<'s>(&'s self) -> impl Iterator<Item = u64> + 'static {
        self.ids.clone().into_iter()
    }
}

fn main() {
    let u = {
        let f = Foo { ids: vec![] };
        Trait::trait_ids(&&f)
    };

    let u = { Foo { ids: vec![] }.ids() };
    println!("{:?}", u.collect::<Vec<_>>());
    // Good!

    let u = { Foo { ids: vec![] }.trait_ids() };
    println!("{:?}", u.collect::<Vec<_>>());
    // Compile error in an old version of Rust:
    //     error[E0716]: temporary value dropped while borrowed
    //         --> src/bin/return-position-2.rs:23:16
    //         |
    //         23 |     let _u = { Foo { ids: vec![] }.trait_ids() };
    //     |                ^^^^^^^^^^^^^^^^^^^              - temporary value is freed at the end of this statement
    //         |                |
    //     |                creates a temporary value which is freed while still in use
    //     24 | }
    // | - borrow might be used here, when `_u` is dropped and runs the destructor for type `impl Iterator<Item = u64> + 'static`
    // |
    // = note: consider using a `let` binding to create a longer lived value
}
