trait Quorum {
    fn ids<'s>(&'s self) -> impl Iterator<Item = u64> + 'static;
}

struct Example {
    data: Vec<u64>,
}

impl Quorum for Example {
    fn ids<'s>(&'s self) -> impl Iterator<Item = u64> + 'static {
        self.data.clone().into_iter()
    }
}
fn main() {}
