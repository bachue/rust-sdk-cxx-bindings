fn main() {
    cxx_build::bridge("src/lib.rs")
        .flag_if_supported("-std=c++20")
        .compile("rust-sdk");

    println!("cargo:rerun-if-changed=src/lib.rs");
}
