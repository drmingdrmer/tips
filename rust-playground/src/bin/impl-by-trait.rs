//! Try to implement differently for a method if the struct implements a trait.
//!
//! Using specialization it can be done.

#![feature(specialization)]

trait Trait {}

trait Doit {
    fn doit() -> u64 {
        0
    }
}

#[derive()]
struct Foo<T> {
    _p: std::marker::PhantomData<T>,
}

impl<T> Doit for Foo<T> {
    default fn doit() -> u64 {
        3
    }
}

impl<T> Doit for Foo<T>
where
    T: Trait,
{
    fn doit() -> u64 {
        4
    }
}

impl Trait for u32 {}

fn main() {
    println!("{}", Foo::<u32>::doit());
    println!("{}", Foo::<u64>::doit());
}
