- [[rust-unstable-features]]
- 线上书:
  `<< The Rustonomicon >>`: The Dark Arts of Advanced and Unsafe Rust Programming
  https://doc.rust-lang.org/nomicon/intro.html
  https://github.com/rust-lang/nomicon
- **Env variables**
  https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/cargo/reference/environment-variables.html
- cargo-run
  https://doc.rust-lang.org/cargo/commands/cargo-run.html#cargo-run1
- What's unwind-safe?
- list enabled features when compiling: `cargo  tree -e features`
- skip boundary check; Informs the optimizer that a condition is always true. https://doc.rust-lang.org/core/intrinsics/fn.assume.html
- flatten_error? flatten result?
- crate num and num-traits: define traits for numbers
  num depends on num-traits
  https://docs.rs/num/0.4.0/num/
	- std::ops::Mul
- ```rust
  pub fn flush<const *NEED_ROLLBACK*: bool>(&mut self) -> Result<(), OutOfLimit> {
      if NEED_ROLLBACK {}
  }
  state_buffer.flush::<true>()
  ```
-
- ```
  #![deny(unused_crate_dependencies)]
  ```
  To detect unused crate when compiling
- https://lazy.codes/posts/awesome-unstable-rust-features/
- Awesome Unstable Rust Features #read
-
-
- `toy-rpc`: enables `serde_bincode` by default. `bincode` does not support `#[serde(flatten)]`. https://docs.rs/crate/toy-rpc/latest/features
- `bincode` https://github.com/bincode-org/bincode
	- does not support `#[serde(flatten)]` and `skip`: https://docs.rs/bincode/2.0.0-alpha.1/bincode/serde/index.html#known-issues
-
- Generate assembly, targeting x86_64, in intel asm syntax:
  `rustc -O --crate-type lib --target x86_64 -C "llvm-args=-x86-asm-syntax=intel" --emit asm x.rs`
-
- rust building rollup:
  https://forge.rust-lang.org/release/rollups.html
-
- sparse-index: 
  The feature causes Cargo to access the crates.io index over HTTP, rather than git. It can provide a significant performance improvement, especially if the local copy of the git index is out-of-date or not yet cloned.
  `cargo -Z sparse-registry build`
  ```
  #.cargo/config.toml
  [registries.crates-io]
  protocol = "sparse"
  ```
-
- `cargo build --timings`
  build time profiling
- `#![deny(clippy::unnecessary_wraps)]`
  warn about unnecessary `Result<T,E>`: should be `T`
-
- cargo watch and build doc
  `RUSTDOCFLAGS="-D warnings" cargo watch -x 'doc --all --no-deps'`
-
- implied lifetimes lead to mismatched type error with seemingly identical types in the msg - async closure edition
  https://github.com/rust-lang/rust/issues/108114
- Build Postgres Extensions with Rust!
  https://github.com/tcdi/pgx
-
- profiling tutorial:
  https://www.p99conf.io/2022/08/02/async-rust-in-practice-performance-pitfalls-profiling/
-
- **cve-rs**
  Blazingly 🔥 fast 🚀 memory vulnerabilities, written in 100% safe Rust. 🦀
  https://github.com/Speykious/cve-rs