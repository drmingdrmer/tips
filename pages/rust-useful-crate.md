- **match_template** #crate This crate provides a macro that can be used to append a match expression with multiple arms
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
- **rustc_version**: #crate get version of rustc
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
	  ```toml
	  [test-groups]
	  serial-integration = { max-threads = 1 }
	  
	  [[profile.default.overrides]]
	  filter = 'package(databend-meta)'
	  test-group = 'serial-integration'
	  ```
	  `cargo nextest show-config test-groups`
- **cargo-quickinstall** like Homebrew
  https://crates.io/crates/cargo-quickinstall
  #install `cargo install cargo-quickinstall`
  #usage `cargo quickinstall ripgrep`
- **cargo-binstall** like Homebrew, fallback to quickinstall
  https://crates.io/crates/cargo-binstall
  #install `cargo install cargo-binstall`
  #usage `cargo binstall xxx`
- **cargo-wipe** wipes all "target" or "node_modules"
  #install `cargo install cargo-wipe`
  #usage `cargo wipe rust`
- **yazi** Blazing fast terminal file manager written in Rust, based on async I/O
  https://github.com/sxyazi/yazi
  #install `cargo install --locked yazi-fm`
  dependency, see: https://yazi-rs.github.io/usage/installation/
  `brew install ffmpegthumbnailer unar jq poppler fd ripgrep fzf zoxide`
  `brew tap homebrew/cask-fonts && brew install --cask font-symbols-only-nerd-font`
  config preset: https://github.com/sxyazi/yazi/tree/main/yazi-config/preset
  #issue use-in-item: profiles::text::"use a different font for non-ascii text" and choose nerd font installed from the above step.
- **indoc** #crate un-indents multiline string at compile time
  https://docs.rs/indoc/latest/indoc/
- **utteranc.es** #comment lightweight comments widget built on GitHub issues
  https://utteranc.es
  https://github.com/utterance/utterances
  used by https://rustmagazine.github.io/rust_magazine_2021/index.html
  looks not banned by GFW
- **giscus** #comment A comment system powered by GitHub Discussions
  https://giscus.app/
  https://github.com/giscus/giscus
  used by https://blog.openacid.com/algo/mmp3/
  looks banned by GFW?
-