use std::ffi::OsStr;
use walkdir::WalkDir;

fn main() {
    cxx_build::bridge("src/lib.rs")
        .file("cpp/qiniu_sdk_ffi.cpp")
        .include(".")
        .flag_if_supported("-std=c++11")
        .compile("qiniu-sdk-ffi");

    for entry in WalkDir::new("src")
        .into_iter()
        .chain(WalkDir::new("cpp").into_iter())
        .chain(WalkDir::new("includes").into_iter())
        .chain(WalkDir::new("rust-sdk").into_iter())
        .filter_map(|entry| {
            if let Some(entry) = entry.ok() {
                let ext = entry.path().extension();
                if entry.file_type().is_file()
                    && (ext == Some(OsStr::new("rs")) || ext == Some(OsStr::new("cpp")))
                {
                    return Some(entry);
                }
            }
            None
        })
    {
        println!("cargo:rerun-if-changed={}", entry.path().display());
    }
}
