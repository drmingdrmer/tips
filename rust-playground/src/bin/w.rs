trait Get<'a, 'b> {
    fn foo(&'a self) -> &'b u64;
}

struct Static {
    v: u64,
}
struct Ref<'r> {
    r: &'r u64,
}

impl<'a> Get<'a, 'a> for Static {
    fn foo(&'a self) -> &'a u64 {
        &self.v
    }
}

impl<'a, 'b> Get<'a, 'b> for Ref<'b> {
    fn foo(&'a self) -> &'b u64 {
        &self.r
    }
}

fn main() {
    let s = Static { v: 3 };

    let u;
    {
        let r = Ref { r: &s.v };
        u = r.foo();
    }

    println!("{}", u);
}

// fn foo<'outer, 'inner, 'x>(input: &'outer (&'inner i64)) -> &'x i64
//     where
//         'inner: 'x,
// {
//     *input
// }
//
// fn bar(input: &i64) -> &i64 {
//     input
// }
//
// fn gg<'f, 'r, 'b>(f: &'f Foo<'r>) -> &'b u64
// where
//     'r: 'b,
// {
//     &f.r
// }
