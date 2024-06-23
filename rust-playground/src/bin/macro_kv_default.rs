trait RaftTypeConfig {
    type A;
    type B;
    type C;
}

macro_rules! impl2 {
    ($($name:ident = $type:ty),* ) => {
        impl2!(@F_0, $($name = $type,)* @T);
    };

    // error: local ambiguity when calling macro `impl2`: multiple parsing options: built-in NTs ident ('n1') or 1 other option.
    // (@R_0, $($n1:ident = $t1:ty,)* A=$t: ty, $($n2:ident = $t2:ty,)* @T) => {
    //     A=$t:ty,
    //     impl2!(@R_1, $($n1 = $t1,)* $($n2 = $t2,)* @T);
    // };
    //
    // (@R_1, $($n1:ident = $t1:ty,)* B=$t: ty, $($n2:ident = $t2:ty,)* @T) => {
    //     B=$t:ty,
    //     impl2!(@R_2, $($n1 = $t1,)* $($n2 = $t2,)* @T);
    // };
    //
    // (@R_2, $($n1:ident = $t1:ty,)* C=$t: ty, $($n2:ident = $t2:ty,)* @T) => {
    //     C=$t:ty,
    // };

    (@F_0, A=$t: ty, $($name:ident = $type:ty,)* @T) => {
        type A = $t;
        impl2!(@F_1, $($name = $type,)* @T);
    };

    (@F_0, $($name:ident = $type:ty,)* @T ) => {
        type A = Foo;
        impl2!(@F_1, $($name = $type,)* @T);
    };

    (@F_1, B=$t: ty, $($name:ident = $type:ty,)* @T ) => {
        type B = $t;
        impl2!(@F_2, $($name = $type,)* @T);
    };

    (@F_1, $($name:ident = $type:ty,)* @T ) => {
        type B = Bar;
        impl2!(@F_2, $($name = $type,)* @T);
    };

    (@F_2, C=$t: ty, $($name:ident = $type:ty,)* @T ) => {
        type C = $t;
        impl2!(@F_ABC, $($name = $type,)* @T);
    };

    (@F_2, $($name:ident = $type:ty,)* @T) => {
        type C = Wow;
        impl2!(@F_3, $($name = $type,)* @T);
    };

    (@F_3, @T ) => {
    };

    () => {};
}

struct Foo {}

impl RaftTypeConfig for Foo {
    impl2!(B = u16, A = u32);
}

// impl RaftTypeConfig for Foo {
//     impl2!(A = u32, B = u16, C = u8);
// }
// impl RaftTypeConfig for Foo {
//     impl2!(A = u32, B = u16);
// }
// impl RaftTypeConfig for Foo {
//     impl2!(A = u32, C = u8);
// }
// impl RaftTypeConfig for Foo {
//     impl2!(B = u16, C = u8);
// }
// impl RaftTypeConfig for Foo {
//     impl2!(A = u32);
// }
// impl RaftTypeConfig for Foo {
//     impl2!(B = u16);
// }
// impl RaftTypeConfig for Foo {
//     impl2!(C = u8);
// }
// impl RaftTypeConfig for Foo {
//     impl2!();
// }

fn main() {}
