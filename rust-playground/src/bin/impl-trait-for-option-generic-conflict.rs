pub trait Str {
    fn to_str(&self) -> String;
}

impl Str for u64 {
    fn to_str(&self) -> String {
        format!("{}", self)
    }
}

impl<U: Str> Str for Option<U> {
    fn to_str(&self) -> String {
        self.as_ref().to_str()
    }
}

impl<V: Str> Str for Option<&V> {
    fn to_str(&self) -> String {
        self.map(|x| x.to_str()).unwrap_or("-".to_string())
    }
}

fn main() {
    println!("{}", 5u64.to_str());
    println!("{}", Some(&5u64).to_str());
    println!("{}", Some(5u64).to_str());
    println!("{}", None::<&u64>.to_str());
}
