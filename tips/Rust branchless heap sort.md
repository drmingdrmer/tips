tags:: tips, rust-programming, sort, branchless

https://github.com/rust-lang/rust/pull/107894

3个版本的汇编代码对比, 后2种少一个分支:

- Original https://godbolt.org/z/M9zrMWbW1
- Combined cond https://godbolt.org/z/7Es3avoYn
- Split cond https://godbolt.org/z/nhxoKcW3z

优化点:

- `child + 1 < v.len()` 是一个非常可预期的判断, 大部分情况为 `true`;
- `is_less()` 不可预期, 会造成比较多的分支预测失效, 所以这里把它优化成整数操作.


最快的是这样, 1个分支加一个整数运算:

```rust
if child + 1 < v.len() {
    // We need a branch to be sure not to out-of-bounds index,
    // but it's highly predictable.  The comparison, however,
    // is better done branchless, especially for primitives.
    child += is_less(&v[child], &v[child + 1]) as usize;
}
// lea     rcx, [rax + 2]
// cmp     rcx, rsi
// jae     .LBB0_5
// mov     rcx, qword ptr [rdi + 8*rdx]
// cmp     rcx, qword ptr [rdi + 8*rax + 16]
// adc     rdx, 0
```

但是似乎强制都写成整数运算也还会留下一个branch:
https://godbolt.org/z/xrYPKfxjo

```rust
child += ((child + 1 < v.len()) as usize) & (is_less(&v[child], &v[child + 1]) as usize);
// lea     rcx, [rax + 2]
// cmp     rcx, rsi
// jae     .LBB0_21
// mov     rcx, qword ptr [rdi + 8*rdx]
// cmp     rcx, qword ptr [rdi + 8*rax + 16]
// mov     rcx, rdx
// adc     rcx, 0
```

```rust
pub fn heapsort<T, F>(v: &mut [T], mut is_less: F)
where
    F: FnMut(&T, &T) -> bool,
{
    // This binary heap respects the invariant `parent >= child`.
    let mut sift_down = |v: &mut [T], mut node| {
        loop {
            // Children of `node`.
            let mut child = 2 * node + 1;
            if child >= v.len() {
                break;
            }

            // Before optimization:
            // if (child + 1 < v.len() && is_less(&v[child], &v[child + 1])) {
            //     child += 1;
            // }

            // Choose the greater child.
            if child + 1 < v.len() {
                // We need a branch to be sure not to out-of-bounds index,
                // but it's highly predictable.  The comparison, however,
                // is better done branchless, especially for primitives.
                child += is_less(&v[child], &v[child + 1]) as usize;
            }

            // Stop if the invariant holds at `node`.
            if !is_less(&v[node], &v[child]) {
                break;
            }

            // Swap `node` with the greater child, move one step down, and continue sifting.
            v.swap(node, child);
            node = child;
        }
    };

    // Build the heap in linear time.
    for i in (0..v.len() / 2).rev() {
        sift_down(v, i);
    }

    // Pop maximal elements from the heap.
    for i in (1..v.len()).rev() {
        v.swap(0, i);
        sift_down(&mut v[..i], 0);
    }
}

pub fn x(v: &mut [u64]) {
    heapsort(v, |a, b| a.lt(b));
}
```
