- **match_template** This crate provides a macro that can be used to append a match expression with multiple arms
  ```
  match_template! {
    T = [Int, Real, Double],
    match Foo {
        EvalType::T => { panic!("{}", EvalType::T); },
        EvalType::Other => unreachable!(),
    }
  }
  ```
  generates
  ```
  match Foo {
    EvalType::Int => { panic!("{}", EvalType::Int); },
    EvalType::Real => { panic!("{}", EvalType::Real); },
    EvalType::Double => { panic!("{}", EvalType::Double); },
    EvalType::Other => unreachable!(),
  }
  ```
  https://tikv.github.io/doc/match_template/macro.match_template.html
-
- **cargo-public-api** compare the public API, find breaking changes and semver violations
  https://crates.io/crates/cargo-public-api
  #install `cargo install --locked cargo-public-api`
  #usage `cargo +nightly public-api diff 0.7.3`  Add `+nightly` if it complains about rustdoc issue.
- **typos** Finds and corrects spelling mistakes among source code:
  https://crates.io/crates/typos-cli
  #install `cargo install typos-cli`
  #usage `typos` to find typos
  #usage `typos --write-changes` or `typos -w` to fix typos
  github-action: https://github.com/crate-ci/typos/blob/HEAD/docs/github-action.md
- **rustc_version**: get version of rustc
  https://docs.rs/rustc_version/latest/rustc_version/
  ```rust
  use rustc_version::version_meta;
  use rustc_version::Channel;
  fn main() {
      if version_meta().unwrap().channel == Channel::Nightly {
          println!("cargo:rustc-cfg=any_error_nightly");
          println!("cargo:rustc-cfg=feature=\"any_error_nightly\"");
      }
  }
  
  ```
- **cargo-nextest** A next-generation test runner
  https://github.com/nextest-rs/nextest
  https://nexte.st/
  #install `cargo install cargo-nextest --locked`
  #usage `cargo nextest run`
  #usage `cargo nextest run --no-fail-fast --hide-progress-bar`
  #usage `cargo nextest run -p my-package`
  #usage `cargo nextest list`
	- #case customize test group
	  `cat .config/nextest.toml`
	  ```
	  [test-groups]
	  serial-integration = { max-threads = 1 }
	  
	  [[profile.default.overrides]]
	  filter = 'package(databend-meta)'
	  test-group = 'serial-integration'
	  ```
	  `cargo nextest show-config test-groups`
-