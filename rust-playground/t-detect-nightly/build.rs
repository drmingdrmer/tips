use std::env;
use std::process::Command;
use std::str;

fn main() {
    // from anyhow
    if is_nightly() {
        println!("cargo:rustc-cfg=foo_nightly");
    }

    // With RUSTUP_TOOLCHAIN
    // https://stackoverflow.com/questions/59542378/conditional-compilation-for-nightly-vs-stable-rust-or-compiler-version
    let rust_toolchain = env::var("RUSTUP_TOOLCHAIN").unwrap();
    if rust_toolchain.starts_with("stable") {
        // do nothing
    } else if rust_toolchain.starts_with("nightly") {
        //enable the 'nightly-features' feature flag
        println!("cargo:rustc-cfg=feature=\"bar_nightly\"");
    } else {
        panic!("Unexpected value for rustc toolchain")
    }
}


fn is_nightly() -> bool {
    let rustc = env::var_os("RUSTC").unwrap();
    let output = Command::new(rustc).arg("--version").output().unwrap();
    let version = str::from_utf8(&output.stdout).unwrap();

    version.contains("nightly") || version.contains("dev")
}

