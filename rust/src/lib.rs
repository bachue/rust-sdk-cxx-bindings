#[cxx::bridge(namespace = "qiniu_sdk_ffi::rust")]
mod ffi {
    extern "Rust" {
        fn initialize_user_agent(cxx_compiler_info: &str);
    }
}

pub fn initialize_user_agent(cxx_compiler_info: &str) {
    qiniu_sdk::http::set_library_user_agent(
        format!(
            "/qiniu-sdk-cxx-bindings-{}/{}",
            env!("CARGO_PKG_VERSION"),
            cxx_compiler_info,
        )
        .into(),
    )
    .ok();
}
