#![feature(generic_associated_types)]

/// Unexpected higher-ranked lifetime error in GAT usage #100013
/// https://github.com/rust-lang/rust/issues/100013
///
/// GAT Self: 'a bounds break some async blocks with impl Trait #92096
/// https://github.com/rust-lang/rust/issues/92096

pub trait FutureIterator {
    type Future<'s, 'cx>: Send
    where
        's: 'cx;
}

fn call_2<I: FutureIterator>() -> impl Send {
    async {
        // a generator checked for autotrait impl `Send`
        let x = None::<I::Future<'_, '_>>; // a type referencing GAT
        async {}.await; // a yield point
    }
}

fn main() {}
