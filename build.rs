extern crate cmake;
use cmake::Config;
use std::env;

fn main() {
    let mut dst = Config::new("my_lib")
                                .define("CMAKE_INSTALL_PREFIX", env::var("OUT_DIR").unwrap() + "\\..\\..\\..")
                                .build();
    dst.push("build");

    println!("cargo:rustc-link-lib=dylib=my_lib");
    println!("cargo:rustc-link-search=native={}", dst.display());
}