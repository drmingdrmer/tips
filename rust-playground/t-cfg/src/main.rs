// To enable a `#[cfg(xx)]` compilation condition:
//
// cargo --config 'build.rustflags = ["--cfg", "foo_config"]' run
//
// RUSTFLAGS='--cfg foo_config' cargo run
//
// cat ./cargo/config.toml
// [build]
// rustflags = ["--cfg", "foo_config"]
//
// Under the hood it is:
//  rustc --cfg some_condition custom.rs
//
// Reference:
//  cargo config:
//  https://doc.rust-lang.org/cargo/reference/config.html
//
//  rustc CLI args:
//  https://doc.rust-lang.org/rustc/command-line-arguments.html#--cfg-configure-the-compilation-environment

fn main() {
    #[cfg(foo_config)]
    println!("foo_config is enabled!");
}
