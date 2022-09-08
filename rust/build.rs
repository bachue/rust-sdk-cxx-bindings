fn main() {
    cxx_build::bridge("src/lib.rs")
        .flag_if_supported("-std=c++11")
        .compile("qiniu-sdk-ffi");

    println!("cargo:rerun-if-changed=src/lib.rs");
}
