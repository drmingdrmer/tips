tags:: tips, rust-programming, bug, issue

目前 Rust 语言中遇到的bug:

-   [ ] incorrect lifetime bound errors when asserting Send for a Future:

    Tags: async, lifetime, generator, generic-associated-type, 

    Repro: [bug-send-higher-ranked-lifetime.rs](../rust-playground/src/bin/bug-send-higher-ranked-lifetime.rs)


    相关issue:

    Unexpected higher-ranked lifetime error in GAT usage #100013
    https://github.com/rust-lang/rust/issues/100013

    这个issue中提到的另一个repro:

    ```rust
    #![feature(generic_associated_types)]

    pub trait FutureIterator {
        type Future<'s, 'cx>: Send
        where 's: 'cx;
    }

    fn call_2<I: FutureIterator>() -> impl Send {
        async { // a generator checked for autotrait impl `Send`
            let x = None::<I::Future<'_, '_>>; // a type referencing GAT
            async {}.await; // a yield point
        }
    }
    ```

    Higher ranked lifetime error when tryng to see that a future is send. #114046
    https://github.com/rust-lang/rust/issues/114046

    Tracking issue for incorrect lifetime bound errors in async #110338
    https://github.com/rust-lang/rust/issues/110338

    Lifetime bounds in auto trait impls prevent trait from being implemented on generators #64552
    https://github.com/rust-lang/rust/issues/64552
