// 想分别从结构体 Ref 和 RefMut 中拿出 &T,
// 遇到了 &T 和 &mut T 对生命周期的要求不一样的问题:
// RefMut::get() 额外要求 `'me: 'out`
//
// 因为 &mut T 不是 Copy 的,
// 要使用 &T 就必须保证这期间 &'me self 不被 drop,
// 否则会导致 &'d mut T 在 'd 的生命周期内有第2个引用而破坏借用原则.

struct Ref<'d> {
    data: &'d (),
}
struct RefMut<'d> {
    data: &'d mut (),
}

impl<'d> Ref<'d> {
    fn get<'me, 'o>(&'me self) -> &'o ()
    where
        'd: 'o,
    {
        self.data
    }
}

impl<'d> RefMut<'d> {
    fn get<'me, 'out>(&'me mut self) -> &'out ()
    where
        'd: 'out,
        // Without `where 'me: 'out`:
        // 36 |         self.data
        //    |         ^^^^^^^^^ associated function was supposed
        //    |         to return data with lifetime `'out` but it
        //    |         is returning data with lifetime `'me`.
        'me: 'out,
    {
        self.data
    }
}

fn main() {
    let mut d = ();
    Ref { data: &mut d }.get();
    RefMut { data: &mut d }.get();
}
