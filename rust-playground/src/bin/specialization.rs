#![feature(specialization)]

trait Trait {
    fn desc(&self) -> String;
}

/* default impl */
impl<T> Trait for T {
    default fn desc(&self) -> String {
        "something".to_string()
    }
}

/* specialized impl */
impl Trait for u64 {
    // Default impl does not work:
    //
    // error[E0520]: `desc2` specializes an item from a parent `impl`, but that item is not marked `default`
    //   --> src/bin/specialization.rs:34:5
    //    |
    // 30 |   impl<T> TraitWithDefaultImpl for T {}
    //    |   ---------------------------------- parent `impl` is here
    // ...
    // 34 | /     fn desc2(&self) -> String {
    // 35 | |         format!("{}", self)
    // 36 | |     }
    //    | |_____^ cannot specialize default item `desc2`
    //    |
    //    = note: to specialize, `desc2` in the parent `impl` must be marked `default`
    fn desc(&self) -> String {
        format!("{}", self)
    }
}

// ---

/* default impl in trait definition */
trait TraitWithDefaultImpl {
    fn desc2(&self) -> String {
        "unknown_desc".to_string()
    }
}

impl<T> TraitWithDefaultImpl for T {}

/* specialized impl */
impl TraitWithDefaultImpl for u64 {
    fn desc2(&self) -> String {
        format!("{}", self)
    }
}

// ---

trait IterExt<T> {}

impl<T, I: Iterator<Item = T>> IterExt<T> for I {} /* default impl */
impl<T> IterExt<T> for std::vec::IntoIter<T> {} /* specialized impl */

fn main() {
    println!("{}", 1u16.desc());
    println!("{}", 1u32.desc());
    println!("{}", 1u64.desc());

    println!("{}", 1u16.desc2());
    println!("{}", 1u64.desc2());
}
