#[cxx::bridge(namespace = "com::qiniu")]
mod ffi {
    extern "Rust" {
        type RustStruct;
        fn new_rust_struct(vvvv: u8) -> Box<RustStruct>;
        fn show_version(_: Box<RustStruct>, delimiter: &str) -> String;
    }
}

#[derive(Clone)]
pub struct RustStruct {
    vvvv: u8,
}

impl Drop for RustStruct {
    fn drop(&mut self) {
        println!("drop: {}", self.vvvv);
    }
}

pub fn new_rust_struct(vvvv: u8) -> Box<RustStruct> {
    Box::new(RustStruct { vvvv })
}

pub fn show_version(_: Box<RustStruct>, delimiter: &str) -> String {
    format!(
        "{}{}{}",
        env!("CARGO_PKG_NAME"),
        delimiter,
        env!("CARGO_PKG_VERSION")
    )
}
