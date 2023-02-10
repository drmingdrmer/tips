// Detect nightly rust with `build.rs`:
// It output a compilation condition `foo_nightly` and a feature `bar_nightly`
//
// Test:
//   cargo +stable run
//   cargo +nightly run
fn main() {
    #[cfg(foo_nightly)]
    println!("foo_nightly");

    #[cfg(feature = "bar_nightly")]
    println!("bar_nightly");
}
