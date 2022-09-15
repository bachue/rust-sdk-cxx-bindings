mod utils;

mod initialize;
pub use initialize::initialize_user_agent;

mod credential;
pub use credential::{
    credential_get_access_key, credential_get_secret_key, credential_sign, credential_sign_reader,
    credential_sign_with_data, new_credential, Credential,
};

#[cxx::bridge(namespace = "qiniu_sdk_ffi::rust")]
mod ffi {
    extern "Rust" {
        fn initialize_user_agent(cxx_compiler_info: &str);

        // mod credential

        type Credential;
        fn new_credential(access_key: &str, secret_key: &str) -> Box<Credential>;
        fn credential_get_access_key(credential: &Credential) -> &str;
        fn credential_get_secret_key(credential: &Credential) -> &str;
        fn credential_sign(credential: &Credential, data: &[u8]) -> String;
        fn credential_sign_with_data(credential: &Credential, data: &[u8]) -> String;
        unsafe fn credential_sign_reader(
            credential: &Credential,
            stream: *mut c_void,
        ) -> Result<String>;

        // mod upload_token
    }

    unsafe extern "C++" {
        include!("includes/qiniu_sdk_ffi.h");
        type c_void;
        type SeekableReader;
        unsafe fn new_seekable_reader(ptr: *mut c_void) -> UniquePtr<SeekableReader>;
        unsafe fn read(
            self: &SeekableReader,
            data: *mut u8,
            size: usize,
            errbit: *mut u8,
        ) -> Result<usize>;
        unsafe fn seek(self: &SeekableReader, off: i64, pos: u8, errbit: *mut u8) -> Result<u64>;
    }
}
