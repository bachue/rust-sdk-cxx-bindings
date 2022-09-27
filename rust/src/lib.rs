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

mod upload_token;
pub use upload_token::{
    new_upload_policy_for_bucket, new_upload_policy_for_object,
    new_upload_policy_for_objects_with_prefix, upload_policy_as_json, upload_policy_builder_build,
    upload_policy_builder_clone, upload_policy_builder_disable_mime_detection,
    upload_policy_builder_enable_mime_detection, upload_policy_builder_reset,
    upload_policy_builder_set_bool, upload_policy_builder_set_callback,
    upload_policy_builder_set_file_size_limitation, upload_policy_builder_set_file_type,
    upload_policy_builder_set_insert_only, upload_policy_builder_set_integer,
    upload_policy_builder_set_mime_types, upload_policy_builder_set_object_lifetime,
    upload_policy_builder_set_return_body, upload_policy_builder_set_return_url,
    upload_policy_builder_set_save_as, upload_policy_builder_set_string,
    upload_policy_builder_set_token_deadline, upload_policy_builder_set_token_lifetime,
    upload_policy_builder_unset, upload_policy_clone, upload_policy_from_json,
    upload_policy_get_bool, upload_policy_get_bucket, upload_policy_get_callback_body,
    upload_policy_get_callback_body_type, upload_policy_get_callback_host,
    upload_policy_get_callback_urls, upload_policy_get_file_type, upload_policy_get_integer,
    upload_policy_get_key, upload_policy_get_max_file_size, upload_policy_get_mime_types,
    upload_policy_get_min_file_size, upload_policy_get_object_lifetime,
    upload_policy_get_return_body, upload_policy_get_return_url, upload_policy_get_save_key,
    upload_policy_get_string, upload_policy_get_token_deadline, upload_policy_has_key,
    upload_policy_has_prefixal_object_key, upload_policy_is_bool, upload_policy_is_insert_only,
    upload_policy_is_integer, upload_policy_is_mime_detection_enabled,
    upload_policy_is_save_key_forced, upload_policy_is_string, UploadPolicy, UploadPolicyBuilder,
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

        type UploadPolicy;
        type UploadPolicyBuilder;
        fn upload_policy_builder_clone(builder: &UploadPolicyBuilder) -> Box<UploadPolicyBuilder>;
        fn new_upload_policy_for_bucket(bucket: &str, lifetime: u64) -> Box<UploadPolicyBuilder>;
        fn new_upload_policy_for_object(
            bucket: &str,
            object: &str,
            lifetime: u64,
        ) -> Box<UploadPolicyBuilder>;
        fn new_upload_policy_for_objects_with_prefix(
            bucket: &str,
            prefix: &str,
            lifetime: u64,
        ) -> Box<UploadPolicyBuilder>;
        fn upload_policy_builder_set_token_lifetime(
            builder: &mut UploadPolicyBuilder,
            token_lifetime: u64,
        );
        fn upload_policy_builder_set_token_deadline(
            builder: &mut UploadPolicyBuilder,
            token_deadline: u64,
        );
        fn upload_policy_builder_set_insert_only(builder: &mut UploadPolicyBuilder);
        fn upload_policy_builder_enable_mime_detection(builder: &mut UploadPolicyBuilder);
        fn upload_policy_builder_disable_mime_detection(builder: &mut UploadPolicyBuilder);
        fn upload_policy_builder_set_file_type(builder: &mut UploadPolicyBuilder, file_type: u8);
        fn upload_policy_builder_set_return_url(builder: &mut UploadPolicyBuilder, url: &str);
        fn upload_policy_builder_set_return_body(builder: &mut UploadPolicyBuilder, body: &str);
        fn upload_policy_builder_set_callback(
            builder: &mut UploadPolicyBuilder,
            urls: &[&str],
            host: &str,
            body: &str,
            body_type: &str,
        );
        fn upload_policy_builder_set_save_as(
            builder: &mut UploadPolicyBuilder,
            save_as: &str,
            force: bool,
        );
        fn upload_policy_builder_set_file_size_limitation(
            builder: &mut UploadPolicyBuilder,
            min_file_size: u64,
            max_file_size: u64,
        );
        fn upload_policy_builder_set_mime_types(
            builder: &mut UploadPolicyBuilder,
            mime_types: &[&str],
        );
        fn upload_policy_builder_set_object_lifetime(
            builder: &mut UploadPolicyBuilder,
            object_lifetime: u64,
        );
        fn upload_policy_builder_set_string(
            builder: &mut UploadPolicyBuilder,
            key: String,
            value: String,
        );
        fn upload_policy_builder_set_integer(
            builder: &mut UploadPolicyBuilder,
            key: String,
            value: i64,
        );
        fn upload_policy_builder_set_bool(
            builder: &mut UploadPolicyBuilder,
            key: String,
            value: bool,
        );
        fn upload_policy_builder_unset(builder: &mut UploadPolicyBuilder, key: &str);
        fn upload_policy_builder_reset(builder: &mut UploadPolicyBuilder);
        fn upload_policy_builder_build(builder: &UploadPolicyBuilder) -> Box<UploadPolicy>;

        fn upload_policy_clone(policy: &UploadPolicy) -> Box<UploadPolicy>;
        fn upload_policy_get_bucket(policy: &UploadPolicy) -> &str;
        fn upload_policy_get_key(policy: &UploadPolicy) -> &str;
        fn upload_policy_has_prefixal_object_key(policy: &UploadPolicy) -> bool;
        fn upload_policy_is_insert_only(policy: &UploadPolicy) -> bool;
        fn upload_policy_is_mime_detection_enabled(policy: &UploadPolicy) -> bool;
        fn upload_policy_get_token_deadline(policy: &UploadPolicy) -> Result<u64>;
        fn upload_policy_get_return_url(policy: &UploadPolicy) -> &str;
        fn upload_policy_get_return_body(policy: &UploadPolicy) -> &str;
        fn upload_policy_get_callback_urls(policy: &UploadPolicy) -> Vec<String>;
        fn upload_policy_get_callback_host(policy: &UploadPolicy) -> &str;
        fn upload_policy_get_callback_body(policy: &UploadPolicy) -> &str;
        fn upload_policy_get_callback_body_type(policy: &UploadPolicy) -> &str;
        fn upload_policy_get_save_key(policy: &UploadPolicy) -> &str;
        fn upload_policy_is_save_key_forced(policy: &UploadPolicy) -> bool;
        fn upload_policy_get_max_file_size(policy: &UploadPolicy) -> u64;
        fn upload_policy_get_min_file_size(policy: &UploadPolicy) -> u64;
        fn upload_policy_get_mime_types(policy: &UploadPolicy) -> Vec<String>;
        fn upload_policy_get_file_type(policy: &UploadPolicy) -> u8;
        fn upload_policy_get_object_lifetime(policy: &UploadPolicy) -> u64;
        fn upload_policy_as_json(policy: &UploadPolicy) -> String;
        fn upload_policy_from_json(json: &str) -> Result<Box<UploadPolicy>>;
        unsafe fn upload_policy_get_string<'a>(policy: &'a UploadPolicy, key: &'a str) -> &str;
        unsafe fn upload_policy_get_integer<'a>(policy: &'a UploadPolicy, key: &'a str) -> i64;
        unsafe fn upload_policy_get_bool<'a>(policy: &'a UploadPolicy, key: &'a str) -> bool;
        unsafe fn upload_policy_has_key<'a>(policy: &'a UploadPolicy, key: &'a str) -> bool;
        unsafe fn upload_policy_is_string<'a>(policy: &'a UploadPolicy, key: &'a str) -> bool;
        unsafe fn upload_policy_is_integer<'a>(policy: &'a UploadPolicy, key: &'a str) -> bool;
        unsafe fn upload_policy_is_bool<'a>(policy: &'a UploadPolicy, key: &'a str) -> bool;
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
