tags:: tips, rust-programming, search, branchless

[bsearch-branchless.rs](../rust-playground/src/bin/bsearch-branchless.rs)

```rust
fn bsearch<T: PartialOrd>(elts: &[T], v: &T) -> usize {
    let mut lr = [0, elts.len()];

    while lr[0] + 1 < lr[1] {
        let mid = (lr[0] + lr[1]) / 2;
        lr[(v < &elts[mid]) as usize] = mid;
    }
    lr[0]
}
```

测试一下:

```rust
fn main() {
    let a: Vec<usize> = vec![1, 2, 5, 7, 8];

    for v in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] {
        let index = bsearch(&a, &v);
        println!("bsearch({:?}, {}) -> {}", a, v, index);
    }
}
// bsearch([1, 2, 5, 7, 8], 0) -> 0
// bsearch([1, 2, 5, 7, 8], 1) -> 0
// bsearch([1, 2, 5, 7, 8], 2) -> 1
// bsearch([1, 2, 5, 7, 8], 3) -> 1
// bsearch([1, 2, 5, 7, 8], 4) -> 1
// bsearch([1, 2, 5, 7, 8], 5) -> 2
// bsearch([1, 2, 5, 7, 8], 6) -> 2
// bsearch([1, 2, 5, 7, 8], 7) -> 3
// bsearch([1, 2, 5, 7, 8], 8) -> 4
// bsearch([1, 2, 5, 7, 8], 9) -> 4
```
