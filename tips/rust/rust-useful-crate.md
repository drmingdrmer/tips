- **pingora** #framework by cloudflare A library for building fast, reliable and evolvable network services.
  https://github.com/cloudflare/pingora
  https://github.com/cloudflare/pingora/blob/main/docs/quick_start.md
-
-
- https://docs.rs/matchit/0.8.0/matchit/
-
- https://docs.rs/test-harness/0.2.0/test_harness/
-
- **typos-cli** #crate check and fix typos
  https://crates.io/crates/typos-cli
  #install `cargo install typos-cli`
-
- Macro expansions are tested with [`macrotest`] crate.
- Macro compile errors are tested with [`trybuild`] crate.
- procedure-macro example:
  https://github.com/dtolnay/syn/tree/master/examples
-
-
- **async-channel** #crate mpmc; Async multi-producer multi-consumer channel
  https://crates.io/crates/async-channel
- **ctor** #crate Procedural macro for defining global constructor/destructor functions.
  https://docs.rs/ctor/0.2.7/ctor/index.html
  #usage https://github.com/drmingdrmer/databend/blob/9cd5e5ec9988b0afc9614c3fa47a25efb15879a3/src/bendpy/src/utils.rs#L21
- **gitoxide** #crate #cli An idiomatic, lean, fast & safe pure Rust implementation of Git
  https://github.com/Byron/gitoxide
- **nebari** #crate A pure Rust database implementation using an append-only B-Tree file format.
  https://github.com/khonsulabs/nebari
- **indoc** #crate un-indents multiline string at compile time
  https://docs.rs/indoc/latest/indoc/
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
- **ordered-multimap-rs** #crate multiple values can be associated with a given key
  https://crates.io/crates/ordered-multimap
  #in-databend
- **pollster** #crate #async A minimal async executor that lets you block on a future
  https://github.com/zesterer/pollster
- **pulldown-cmark** #crate An efficient, reliable parser for CommonMark, a standard dialect of Markdown
  https://github.com/pulldown-cmark/pulldown-cmark
- **stract** #crate web search done right
  https://github.com/StractOrg/stract
  use Openraft
  #LATER
- **trybuild** #crate #compile #test test rs compile output
  https://docs.rs/trybuild/latest/trybuild/
-
- ### Graph / Plot / Diagram
- **plotters** #crate A rust drawing library for high quality data plotting for both WASM and native, statically and realtimely
  https://github.com/plotters-rs/plotters
-
- ### Human-Readable
	- **humantime** #crate Human-friendly time parser and formatter
	  https://docs.rs/humantime/latest/humantime/
- ### Search-Engine
	- **Tantivy** #crate is a full-text search engine library inspired by Apache Lucene and written in Rust 
	  https://github.com/quickwit-oss/tantivy
	  #in-databend #read
-
- **serde_stacker** This crate provides a Serde adapter that avoids stack overflow by dynamically growing the stack.
  https://crates.io/crates/serde_stacker
-
- **serde_repr** Derive `Serialize` and `Deserialize` that delegates to the underlying repr of a C-like enum.
  https://docs.rs/serde_repr/latest/serde_repr/index.html
- **dprint** Pluggable and configurable code formatting platform written in Rust.
  https://github.com/dprint/dprint
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
- **cargo-binstall** like Homebrew, fallback to quickinstall
  https://crates.io/crates/cargo-binstall
  #install `cargo install cargo-binstall`
  #usage `cargo binstall xxx`
- **cargo-machete** Remove unused dep
  https://github.com/bnjbvr/cargo-machete
  #install `cargo install cargo-machete`
  #usage `cargo machete`
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
- **cargo-watch** Watches over your Cargo project's source.
  https://github.com/watchexec/cargo-watch
  #install `cargo install cargo-watch --locked`
  #usage `cargo watch -x 'doc --all --no-deps'`
  #usage `RUSTDOCFLAGS="-D warnings" cargo watch -x 'doc --all --no-deps'`
- **cargo-wipe** wipes all "target" or "node_modules"
  https://github.com/mihai-dinculescu/cargo-wipe
  #install `cargo install cargo-wipe`
  #usage `cargo wipe rust`
- **cargo-fix** Automatically fix lint warnings reported by rustc
  https://doc.rust-lang.org/cargo/commands/cargo-fix.html
-
- Several cargo commands:
  https://doc.rust-lang.org/cargo/commands/cargo-fetch.html
-
-
- **jj** A Git-compatible VCS that is both simple and powerful
  https://github.com/martinvonz/jj?tab=readme-ov-file
  #install `cargo install --locked --bin jj jj-cli`
- **dust** A more intuitive version of du in rust
  https://github.com/bootandy/dust
  #install `cargo install du-dust`
- **yazi** Blazing fast terminal file manager written in Rust, based on async I/O
  https://github.com/sxyazi/yazi
  #install `cargo install --locked yazi-fm`
  dependency, see: https://yazi-rs.github.io/usage/installation/
  `brew install ffmpegthumbnailer unar jq poppler fd ripgrep fzf zoxide`
  `brew tap homebrew/cask-fonts && brew install --cask font-symbols-only-nerd-font`
  config preset: https://github.com/sxyazi/yazi/tree/main/yazi-config/preset
  #issue use-in-item: profiles::text::"use a different font for non-ascii text" and choose nerd font installed from the above step.
- **hickory-dns** #crate DNS client, server, and resolver
  https://github.com/hickory-dns/hickory-dns
  previous name: trust-dns-resolver
  #TODO
- **limits-rs** #crate A Rust library for determining the limits that an operating system enforces on a given particular process
  https://github.com/aesedepece/limits-rs
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
- **git-delta** A syntax-highlighting pager for git, diff, grep, and blame output
  https://github.com/dandavison/delta
  #install `cargo install git-delta`
-
- **ripgrep**: ripgrep recursively searches directories for a regex pattern while respecting your gitignore
  #install `cargo install ripgrep`
  https://github.com/BurntSushi/ripgrep
-
- ### Cmmand line util
	- **topgrade** #cli Upgrade all the things
	  https://github.com/topgrade-rs/topgrade
	  #install `cargo install topgrade`
	- **tokei** #cli Count your code, quickly.
	  https://github.com/XAMPPRocky/tokei
	  #install `cargo install --git https://github.com/XAMPPRocky/tokei.git tokei`
	- **minimal-versions** #cli check all crates with minimal version dependencies
	  https://github.com/taiki-e/cargo-minimal-versions
	  #install `rustup toolchain add nightly; cargo +stable install cargo-hack --locked; cargo +stable install cargo-minimal-versions --locked`