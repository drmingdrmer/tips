#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
#[allow(dead_code)]
fn main() {
    let body = async {};
    #[allow(clippy::expect_used)]
    monoio::RuntimeBuilder::<monoio::FusionDriver>::new()
        .build()
        .expect("Failed building the Runtime")
        .block_on(body)
}
#[rustc_main]
#[coverage(off)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[])
}
