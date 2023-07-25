struct Foo;

// error[E0382]: borrow of moved value: `v`
// |
// | fn borrow_of_moved_error(v: &mut Foo) {
// |                          - move occurs because `v` has type `&mut Foo`,
// |                            which does not implement the `Copy` trait
// |     generic_write(v);
// |                   - value moved here
// |     write(v);
// |           ^ value borrowed here after move
fn borrow_of_moved_error(v: &mut Foo) {
    generic_write(v);
    write(v);
}

fn solution(v: &mut Foo) {
    // Solution: manually re-borrow with `&mut *v`.
    // Refer to: https://github.com/rust-lang/rust/issues/62112
    //
    // Forcing the generic type is also a solution:
    // `Self::generic_write::<&mut Foo>(v);`
    generic_write(&mut *v);
    write(v);
}

fn generic_write<V>(_v: V) {}
fn write(_v: &mut Foo) {}

fn main() {}
