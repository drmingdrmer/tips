struct Foo {
    ids: Vec<u64>,
}

struct Ref<T> {
    r: T,
}

trait Trait<'r> {
    fn ids<'s>(&'s self) -> impl Iterator<Item = u64> + 'r;
}

impl<'a> Trait<'static> for Ref<&'a Foo> {
    fn ids<'s>(&'s self) -> impl Iterator<Item = u64> + 'static {
        self.r.ids.clone().into_iter()
    }
}

impl Foo {
    // `ids()` does not capture lifetime of `self`. But it does not seem to work:
    // https://rust-lang.github.io/rfcs/3498-lifetime-capture-rules-2024.html#appendix-e-adding-a-static-bound
    //
    // edition 2024:
    //
    // error[E0597]: `r` does not live long enough
    //    --> src/bin/return-position.rs:22:19
    //    |
    // 20 |     fn get_ids<'a>(&'a self) -> impl Iterator<Item = u64> + 'static {
    //    |                -- lifetime `'a` defined here
    // 21 |         let r: Ref<&'a Foo> = Ref { r: self };
    //    |             - binding `r` declared here
    // 22 |         let ids = r.ids();
    //    |                   ^------
    //    |                   |
    //    |                   borrowed value does not live long enough
    //    |                   argument requires that `r` is borrowed for `'a`
    // 23 |         ids
    // 24 |     }
    //    |     - `r` dropped here while still borrowed
    fn get_ids<'a>(&'a self) -> impl Iterator<Item = u64> + 'static {
        let r: Ref<&'a Foo> = Ref { r: self };
        let ids = r.ids();
        ids
    }
}

fn main() {
    let u = { Foo { ids: vec![] }.get_ids() };
    println!("{:?}", u.collect::<Vec<_>>());
}
