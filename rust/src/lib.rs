mod utils;

mod initialize;
pub use initialize::initialize_user_agent;

mod etag;
pub use etag::{
    etag_of, etag_to_buf, etag_v1_finalize_to_buf, etag_v1_reset, etag_v1_update, new_etag_v1,
    EtagV1,
};

mod credential;
pub use credential::{
    chain_credentials_provider_builder_append_credential, chain_credentials_provider_builder_build,
    chain_credentials_provider_builder_prepend_credential, credential_authorization_v1_for_request,
    credential_authorization_v1_for_request_with_body_reader,
    credential_authorization_v2_for_request,
    credential_authorization_v2_for_request_with_body_reader, credential_clone,
    credential_get_access_key, credential_get_secret_key, credential_provider_clone,
    credential_provider_get, credential_sign, credential_sign_download_url, credential_sign_reader,
    credential_sign_with_data, env_credential_provider_clear, env_credential_provider_setup,
    get_credential_options_clone, global_credential_provider_clear,
    global_credential_provider_setup, got_credential_get_credential,
    new_chain_credentials_provider_builder, new_credential, new_env_credential_provider,
    new_get_credential_options, new_global_credential_provider, new_static_credential_provider,
    ChainCredentialsProviderBuilder, Credential, CredentialProvider, GetCredentialOptions,
    GotCredential,
};

#[cxx::bridge(namespace = "qiniu_sdk::_internal::rust_sdk_ffi")]
mod ffi {
    extern "Rust" {
        fn initialize_user_agent(cxx_compiler_info: &str);

        // mod etag

        unsafe fn etag_of(stream: *mut c_void) -> Result<String>;
        unsafe fn etag_to_buf(stream: *mut c_void, buf: *mut c_void) -> Result<()>;

        type EtagV1;
        fn new_etag_v1() -> Box<EtagV1>;
        fn etag_v1_update(etag: &mut EtagV1, data: &[u8]);
        unsafe fn etag_v1_finalize_to_buf(etag: &mut EtagV1, buf: *mut c_void) -> Result<()>;
        fn etag_v1_reset(etag: &mut EtagV1);

        // mod credential

        type Credential;
        fn new_credential(access_key: &str, secret_key: &str) -> Box<Credential>;
        fn credential_clone(credential: &Credential) -> Box<Credential>;
        fn credential_get_access_key(credential: &Credential) -> &str;
        fn credential_get_secret_key(credential: &Credential) -> &str;
        fn credential_sign(credential: &Credential, data: &[u8]) -> String;
        fn credential_sign_with_data(credential: &Credential, data: &[u8]) -> String;
        unsafe fn credential_sign_reader(
            credential: &Credential,
            stream: *mut c_void,
        ) -> Result<String>;
        fn credential_authorization_v1_for_request(
            credential: &Credential,
            url: &str,
            content_type: &str,
            body: &[u8],
        ) -> Result<String>;
        unsafe fn credential_authorization_v1_for_request_with_body_reader(
            credential: &Credential,
            url: &str,
            content_type: &str,
            stream: *mut c_void,
        ) -> Result<String>;
        fn credential_authorization_v2_for_request(
            credential: &Credential,
            method: &str,
            url: &str,
            headers: &[[&str; 2]],
            body: &[u8],
        ) -> Result<String>;
        unsafe fn credential_authorization_v2_for_request_with_body_reader(
            credential: &Credential,
            method: &str,
            url: &str,
            headers: &[[&str; 2]],
            stream: *mut c_void,
        ) -> Result<String>;
        fn credential_sign_download_url(
            credential: &Credential,
            url: &str,
            duration: u64,
        ) -> Result<String>;

        type CredentialProvider;
        type GetCredentialOptions;
        type GotCredential;
        fn credential_provider_get(
            provider: &CredentialProvider,
            options: &GetCredentialOptions,
        ) -> Result<Box<GotCredential>>;
        fn new_static_credential_provider(credential: Box<Credential>) -> Box<CredentialProvider>;
        fn credential_provider_clone(provider: &CredentialProvider) -> Box<CredentialProvider>;
        fn got_credential_get_credential(got_credential: &GotCredential) -> Box<Credential>;
        fn new_get_credential_options() -> Box<GetCredentialOptions>;
        fn get_credential_options_clone(
            options: &GetCredentialOptions,
        ) -> Box<GetCredentialOptions>;
        fn new_global_credential_provider() -> Box<CredentialProvider>;
        fn global_credential_provider_setup(credential: &Credential);
        fn global_credential_provider_clear();
        fn new_env_credential_provider() -> Box<CredentialProvider>;
        fn env_credential_provider_setup(credential: &Credential);
        fn env_credential_provider_clear();
        type ChainCredentialsProviderBuilder;
        fn new_chain_credentials_provider_builder(
            provider: Box<CredentialProvider>,
        ) -> Box<ChainCredentialsProviderBuilder>;
        fn chain_credentials_provider_builder_append_credential(
            builder: &mut ChainCredentialsProviderBuilder,
            provider: Box<CredentialProvider>,
        );
        fn chain_credentials_provider_builder_prepend_credential(
            builder: &mut ChainCredentialsProviderBuilder,
            provider: Box<CredentialProvider>,
        );
        fn chain_credentials_provider_builder_build(
            builder: &mut ChainCredentialsProviderBuilder,
        ) -> Box<CredentialProvider>;

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
