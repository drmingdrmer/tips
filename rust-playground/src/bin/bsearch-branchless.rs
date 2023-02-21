fn bsearch<T: PartialOrd>(elts: &[T], v: &T) -> usize {
    let mut lr = [0, elts.len()];

    while lr[0] + 1 < lr[1] {
        let mid = (lr[0] + lr[1]) / 2;
        lr[(v < &elts[mid]) as usize] = mid;
    }
    lr[0]
}

fn main() {
    let a: Vec<usize> = vec![1, 2, 5, 7, 8];

    for v in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] {
        let index = bsearch(&a, &v);
        println!("bsearch({:?}, {}) -> {}", a, v, index);
    }
}
