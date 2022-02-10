use std::{env, path::Path};

use cmake::Config;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_path = Config::new(Path::new(&manifest_dir).join("build"))
        .profile("Release")
        .build_target("CMP_Compressonator")
        .build();
    println!(
        "cargo:rustc-link-search=native={}",
        build_path.join("build").join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=CMP_Compressonator");
    #[cfg(any(target_os = "macos", target_os = "ios", target_os = "freebsd"))]
    println!("cargo:rustc-link-lib=c++");
    #[cfg(not(any(target_os = "macos", target_os = "ios", target_os = "freebsd")))]
    println!("cargo:rustc-link-lib=stdc++");
}
