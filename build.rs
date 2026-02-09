fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();

    if target_os == "solaris" {
        println!("cargo:rustc-link-search=native=/usr/lib/64");
        println!("cargo:rustc-link-search=native=/usr/gnu/lib");
        println!("cargo:rustc-link-lib=socket");
        println!("cargo:rustc-link-lib=nsl");
        println!("cargo:rustc-env=CFLAGS=-I/usr/include -D__EXTENSIONS__");
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");
}
