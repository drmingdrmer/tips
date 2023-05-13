#![feature(min_specialization)]

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
    fn desc(&self) -> String {
        format!("{}", self)
    }
}

trait IterExt<T> {}

impl<T, I: Iterator<Item = T>> IterExt<T> for I {} /* default impl */
impl<T> IterExt<T> for std::vec::IntoIter<T> {} /* specialized impl */

fn main() {
    println!("{}", 1u16.desc());
    println!("{}", 1u32.desc());
    println!("{}", 1u64.desc());
}
