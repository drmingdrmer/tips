pub mod foo {

    pub trait Trait {
        /// This method is unimplementable by users,
        /// because it references a trait in a private mod `sealed`
        fn unimplementable(&self)
        where
            Self: sealed::Sealed,
        {
            self.user_impl_this();
        }

        fn user_impl_this(&self);
    }

    mod sealed {
        pub trait Sealed {}
        impl<T> Sealed for T {}
    }
}

////////////////////////////////////////////////////////

pub struct Implementor;

impl foo::Trait for Implementor {
    fn unimplementable(&self)
    // where
    //     Self: foo::sealed::Sealed,
    {
        todo!()
    }

    fn user_impl_this(&self) {
        println!("user_impl_this");
    }
}

fn main() {}
