#![no_std]
#[macro_use]
extern crate digest;
use md5;

use digest::dev::{digest_test, one_million_a};

new_test!(md5_main, "md5", md5::Md5, digest_test);

#[test]
fn md5_1million_a() {
    let output = include_bytes!("data/one_million_a.bin");
    one_million_a::<md5::Md5>(output);
}
