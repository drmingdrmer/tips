tags:: tips, rust-programming, toolchaine, nightly, stable

使用环境变量 `RUSTUP_TOOLCHAIN`
```rust
// build.rs
fn main() {
    // Copied from anyhow
    let rustc = env::var_os("RUSTC").unwrap();
    let output = Command::new(rustc).arg("--version").output().unwrap();
    let version = str::from_utf8(&output.stdout).unwrap();
    let is_nightly = version.contains("nightly") || version.contains("dev")
    if is_nightly {
        println!("cargo:rustc-cfg=rustc_nightly");
    }
}

// main.rs
fn main() {
    #[cfg(rustc_nightly)]
    println!("rustc_nightly");
}
```

使用 `rustc --version`
```rust
// build.rs
fn main() {
    // With RUSTUP_TOOLCHAIN
    // https://stackoverflow.com/questions/59542378/conditional-compilation-for-nightly-vs-stable-rust-or-compiler-version
    let rust_toolchain = env::var("RUSTUP_TOOLCHAIN").unwrap();
    if rust_toolchain.starts_with("stable") {
        // do nothing
    } else if rust_toolchain.starts_with("nightly") {
        //enable the 'nightly-features' feature flag
        println!("cargo:rustc-cfg=feature=\"env_nightly\"");
    } else {
        panic!("Unexpected value for rustc toolchain")
    }
}

// main.rs
fn main() {
    #[cfg(feature = "env_nightly")]
    println!("env_nightly");
}
```


Code: [detect-nightly](../rust-playground/t-detect-nightly)
