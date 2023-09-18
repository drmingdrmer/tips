// 想分别从结构体 Ref 和 RefMut 中拿出 &T,
// 遇到了 &T 和 &mut T 对生命周期的要求不一样的问题:
// RefMut::get_data() 额外要求 `'me: 't`
//
// 因为 &mut T 不是 Copy 的,
// 要使用 &T 就必须保证这期间 &'me self 不被 drop,
// 否则会导致 &'t mut T 在 't 的生命周期内有第2个引用而破坏借用原则.

struct Ref<'t> {
    data: &'t (),
}
struct RefMut<'t> {
    data: &'t mut (),
}

impl<'t> Ref<'t> {
    fn get<'me>(&'me self) -> &'t () {
        self.data
    }
}

impl<'t> RefMut<'t> {
    // Without `where 'me: 't`:
    // |  self.data
    // |  ^^^^^^^^^ associated function was supposed
    // |            to return data with lifetime `'t`
    // |            but it is returning data with lifetime `'me`
    fn get<'me>(&'me mut self) -> &'t () {
        self.data
    }
}

fn main() {
    let mut d = ();
    Ref { data: &mut d }.get();
    RefMut { data: &mut d }.get();
}
