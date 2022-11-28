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
    bucket_upload_token_provider_builder_build, get_access_key_options_clone,
    get_policy_options_clone, got_upload_policy_get_upload_policy,
    new_bucket_upload_token_provider_builder, new_get_access_key_options, new_get_policy_options,
    new_object_upload_token_provider_builder, new_static_upload_token_provider,
    new_to_upload_token_string_options, new_upload_policy_for_bucket, new_upload_policy_for_object,
    new_upload_policy_for_objects_with_prefix, new_upload_token_provider_from_upload_policy,
    object_upload_token_provider_builder_build, to_upload_token_string_options_clone,
    upload_policy_as_json, upload_policy_builder_build, upload_policy_builder_clone,
    upload_policy_builder_disable_mime_detection, upload_policy_builder_enable_mime_detection,
    upload_policy_builder_reset, upload_policy_builder_set_bool,
    upload_policy_builder_set_callback, upload_policy_builder_set_file_size_limitation,
    upload_policy_builder_set_file_type, upload_policy_builder_set_insert_only,
    upload_policy_builder_set_integer, upload_policy_builder_set_mime_types,
    upload_policy_builder_set_object_lifetime, upload_policy_builder_set_return_body,
    upload_policy_builder_set_return_url, upload_policy_builder_set_save_as,
    upload_policy_builder_set_string, upload_policy_builder_set_token_deadline,
    upload_policy_builder_set_token_lifetime, upload_policy_builder_unset, upload_policy_clone,
    upload_policy_from_json, upload_policy_get_bool, upload_policy_get_bucket,
    upload_policy_get_callback_body, upload_policy_get_callback_body_type,
    upload_policy_get_callback_host, upload_policy_get_callback_urls, upload_policy_get_file_type,
    upload_policy_get_integer, upload_policy_get_key, upload_policy_get_max_file_size,
    upload_policy_get_mime_types, upload_policy_get_min_file_size,
    upload_policy_get_object_lifetime, upload_policy_get_return_body, upload_policy_get_return_url,
    upload_policy_get_save_key, upload_policy_get_string, upload_policy_get_token_deadline,
    upload_policy_has_key, upload_policy_has_prefixal_object_key, upload_policy_is_bool,
    upload_policy_is_insert_only, upload_policy_is_integer,
    upload_policy_is_mime_detection_enabled, upload_policy_is_save_key_forced,
    upload_policy_is_string, upload_token_provider_clone, upload_token_provider_get_access_key,
    upload_token_provider_get_bucket_name, upload_token_provider_get_policy,
    upload_token_provider_to_token_string, BucketUploadTokenProviderBuilder, GetAccessKeyOptions,
    GetPolicyOptions, GotUploadPolicy, ObjectUploadTokenProviderBuilder,
    ToUploadTokenStringOptions, UploadPolicy, UploadPolicyBuilder, UploadTokenProvider,
};

mod http;
pub use http::{
    http_request_body_get_size, http_request_builder_build, http_request_builder_reset,
    http_request_builder_set_appended_user_agent, http_request_builder_set_body,
    http_request_builder_set_header, http_request_builder_set_headers,
    http_request_builder_set_method, http_request_builder_set_resolved_ip_addrs,
    http_request_builder_set_url, http_request_builder_set_version, http_request_get_body,
    http_request_get_parts, http_request_parts_builder_build,
    http_request_parts_builder_build_with_body, http_request_parts_builder_set_appended_user_agent,
    http_request_parts_builder_set_header, http_request_parts_builder_set_headers,
    http_request_parts_builder_set_method, http_request_parts_builder_set_resolved_ip_addrs,
    http_request_parts_builder_set_url, http_request_parts_builder_set_version,
    http_request_parts_get_appended_user_agent, http_request_parts_get_header,
    http_request_parts_get_header_keys, http_request_parts_get_method,
    http_request_parts_get_resolved_ip_addrs, http_request_parts_get_url,
    http_request_parts_get_user_agent, http_request_parts_get_version,
    http_request_parts_set_appended_user_agent, http_request_parts_set_headers,
    http_request_parts_set_method, http_request_parts_set_resolved_ip_addrs,
    http_request_parts_set_url, http_request_parts_set_version, http_request_set_body,
    http_request_set_parts, metrics_builder_set_connect_duration,
    metrics_builder_set_name_lookup_duration, metrics_builder_set_redirect_duration,
    metrics_builder_set_secure_connect_duration, metrics_builder_set_total_duration,
    metrics_builder_set_transfer_duration, metrics_get_connect_duration,
    metrics_get_name_lookup_duration, metrics_get_redirect_duration,
    metrics_get_secure_connect_duration, metrics_get_total_duration, metrics_get_transfer_duration,
    metrics_set_connect_duration, metrics_set_name_lookup_duration, metrics_set_redirect_duration,
    metrics_set_secure_connect_duration, metrics_set_total_duration, metrics_set_transfer_duration,
    new_http_request_body_default, new_http_request_body_from_bytes,
    new_http_request_body_from_reader, new_http_request_builder, new_http_request_default,
    new_http_request_parts_builder, new_metrics_builder, new_request_parts_default, HttpRequest,
    HttpRequestBody, HttpRequestBuilder, HttpRequestParts, HttpRequestPartsBuilder, Metrics,
    MetricsBuilder,
};

#[cxx::bridge(namespace = "qiniu_bindings::_internal::rust_sdk_ffi")]
mod ffi {
    #[derive(Debug, Clone)]
    enum HttpVersion {
        HTTP_VERSION_09,
        HTTP_VERSION_10,
        HTTP_VERSION_11,
        HTTP_VERSION_2,
        HTTP_VERSION_3,
    }

    #[derive(Debug, Clone)]
    enum HttpMethod {
        HTTP_METHOD_OPTIONS,
        HTTP_METHOD_GET,
        HTTP_METHOD_POST,
        HTTP_METHOD_PUT,
        HTTP_METHOD_DELETE,
        HTTP_METHOD_HEAD,
        HTTP_METHOD_TRACE,
        HTTP_METHOD_CONNECT,
        HTTP_METHOD_PATCH,
    }

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
        unsafe fn upload_policy_get_string<'a>(policy: &'a UploadPolicy, key: &'a str) -> &'a str;
        unsafe fn upload_policy_get_integer<'a>(policy: &'a UploadPolicy, key: &'a str) -> i64;
        unsafe fn upload_policy_get_bool<'a>(policy: &'a UploadPolicy, key: &'a str) -> bool;
        unsafe fn upload_policy_has_key<'a>(policy: &'a UploadPolicy, key: &'a str) -> bool;
        unsafe fn upload_policy_is_string<'a>(policy: &'a UploadPolicy, key: &'a str) -> bool;
        unsafe fn upload_policy_is_integer<'a>(policy: &'a UploadPolicy, key: &'a str) -> bool;
        unsafe fn upload_policy_is_bool<'a>(policy: &'a UploadPolicy, key: &'a str) -> bool;

        type UploadTokenProvider;
        type GetAccessKeyOptions;
        type GetPolicyOptions;
        type ToUploadTokenStringOptions;
        type GotUploadPolicy<'a>;

        fn new_get_access_key_options() -> Box<GetAccessKeyOptions>;
        fn get_access_key_options_clone(options: &GetAccessKeyOptions) -> Box<GetAccessKeyOptions>;
        fn new_get_policy_options() -> Box<GetPolicyOptions>;
        fn get_policy_options_clone(options: &GetPolicyOptions) -> Box<GetPolicyOptions>;
        fn new_to_upload_token_string_options() -> Box<ToUploadTokenStringOptions>;
        fn to_upload_token_string_options_clone(
            options: &ToUploadTokenStringOptions,
        ) -> Box<ToUploadTokenStringOptions>;
        fn got_upload_policy_get_upload_policy(
            got_upload_policy: &GotUploadPolicy,
        ) -> Box<UploadPolicy>;
        fn new_static_upload_token_provider(upload_token: &str) -> Box<UploadTokenProvider>;
        fn new_upload_token_provider_from_upload_policy(
            upload_policy: Box<UploadPolicy>,
            credential: Box<CredentialProvider>,
        ) -> Box<UploadTokenProvider>;
        fn upload_token_provider_clone(provider: &UploadTokenProvider) -> Box<UploadTokenProvider>;
        fn upload_token_provider_get_access_key(
            provider: &UploadTokenProvider,
            opts: &GetAccessKeyOptions,
        ) -> Result<String>;
        unsafe fn upload_token_provider_get_policy<'a>(
            provider: &'a UploadTokenProvider,
            opts: &'a GetPolicyOptions,
        ) -> Result<Box<GotUploadPolicy<'a>>>;
        fn upload_token_provider_to_token_string(
            provider: &UploadTokenProvider,
            opts: &ToUploadTokenStringOptions,
        ) -> Result<String>;
        fn upload_token_provider_get_bucket_name(
            provider: &UploadTokenProvider,
            opts: &GetPolicyOptions,
        ) -> Result<String>;

        type BucketUploadTokenProviderBuilder;
        fn new_bucket_upload_token_provider_builder(
            bucket: &str,
            upload_token_lifetime: u64,
            provider: Box<CredentialProvider>,
        ) -> Box<BucketUploadTokenProviderBuilder>;
        fn bucket_upload_token_provider_builder_build(
            builder: Box<BucketUploadTokenProviderBuilder>,
        ) -> Box<UploadTokenProvider>;

        type ObjectUploadTokenProviderBuilder;
        fn new_object_upload_token_provider_builder(
            bucket: &str,
            object: &str,
            upload_token_lifetime: u64,
            provider: Box<CredentialProvider>,
        ) -> Box<ObjectUploadTokenProviderBuilder>;
        fn object_upload_token_provider_builder_build(
            builder: Box<ObjectUploadTokenProviderBuilder>,
        ) -> Box<UploadTokenProvider>;

        // mod http

        type HttpRequestParts<'r>;
        type HttpRequestPartsBuilder<'r>;
        unsafe fn new_http_request_parts_builder<'r>() -> Box<HttpRequestPartsBuilder<'r>>;
        fn http_request_parts_builder_set_url(
            builder: &mut HttpRequestPartsBuilder,
            url: &str,
        ) -> Result<()>;
        fn http_request_parts_builder_set_version(
            builder: &mut HttpRequestPartsBuilder,
            version: HttpVersion,
        );
        fn http_request_parts_builder_set_method(
            builder: &mut HttpRequestPartsBuilder,
            method: HttpMethod,
        );
        fn http_request_parts_builder_set_headers(
            builder: &mut HttpRequestPartsBuilder,
            headers: &[[&str; 2]],
        ) -> Result<()>;
        fn http_request_parts_builder_set_header(
            builder: &mut HttpRequestPartsBuilder,
            name: &str,
            value: &str,
        ) -> Result<()>;
        fn http_request_parts_builder_set_appended_user_agent(
            builder: &mut HttpRequestPartsBuilder,
            user_agent: &str,
        );
        fn http_request_parts_builder_set_resolved_ip_addrs(
            builder: &mut HttpRequestPartsBuilder,
            ips: &[&str],
        ) -> Result<()>;
        unsafe fn new_request_parts_default<'r>() -> Box<HttpRequestParts<'r>>;
        unsafe fn http_request_parts_builder_build<'r>(
            builder: &mut HttpRequestPartsBuilder<'r>,
        ) -> Box<HttpRequestParts<'r>>;
        unsafe fn http_request_parts_builder_build_with_body<'r>(
            builder: &mut HttpRequestPartsBuilder<'r>,
            body: Box<HttpRequestBody<'r>>,
        ) -> Box<HttpRequest<'r>>;
        fn http_request_parts_get_url(parts: &HttpRequestParts) -> String;
        fn http_request_parts_set_url(parts: &mut HttpRequestParts, url: &str) -> Result<()>;
        fn http_request_parts_get_version(parts: &HttpRequestParts) -> HttpVersion;
        fn http_request_parts_set_version(parts: &mut HttpRequestParts, version: HttpVersion);
        fn http_request_parts_get_method(parts: &HttpRequestParts) -> HttpMethod;
        fn http_request_parts_set_method(parts: &mut HttpRequestParts, method: HttpMethod);
        fn http_request_parts_get_header_keys(parts: &HttpRequestParts) -> Vec<String>;
        unsafe fn http_request_parts_get_header<'v>(
            parts: &'v HttpRequestParts,
            key: &'v str,
        ) -> Result<&'v str>;
        fn http_request_parts_set_headers(
            parts: &mut HttpRequestParts,
            headers: &[[&str; 2]],
        ) -> Result<()>;
        fn http_request_parts_get_user_agent(parts: &HttpRequestParts) -> String;
        fn http_request_parts_get_appended_user_agent(parts: &HttpRequestParts) -> String;
        fn http_request_parts_set_appended_user_agent(
            parts: &mut HttpRequestParts,
            user_agent: &str,
        );
        fn http_request_parts_get_resolved_ip_addrs(parts: &HttpRequestParts) -> Vec<String>;
        fn http_request_parts_set_resolved_ip_addrs(
            parts: &mut HttpRequestParts,
            resolved_ip_addrs: &[&str],
        ) -> Result<()>;

        type HttpRequestBody<'r>;
        unsafe fn new_http_request_body_from_reader<'r>(
            stream: *mut c_void,
            size: u64,
        ) -> Box<HttpRequestBody<'r>>;
        unsafe fn new_http_request_body_from_bytes<'r>(bytes: &[u8]) -> Box<HttpRequestBody<'r>>;
        unsafe fn new_http_request_body_default<'r>() -> Box<HttpRequestBody<'r>>;
        fn http_request_body_get_size(body: &HttpRequestBody) -> u64;

        type HttpRequestBuilder<'r>;
        unsafe fn new_http_request_builder<'r>() -> Box<HttpRequestBuilder<'r>>;
        fn http_request_builder_set_url(builder: &mut HttpRequestBuilder, url: &str) -> Result<()>;
        fn http_request_builder_set_version(builder: &mut HttpRequestBuilder, version: HttpVersion);
        fn http_request_builder_set_method(builder: &mut HttpRequestBuilder, method: HttpMethod);
        fn http_request_builder_set_headers(
            builder: &mut HttpRequestBuilder,
            headers: &[[&str; 2]],
        ) -> Result<()>;
        fn http_request_builder_set_header(
            builder: &mut HttpRequestBuilder,
            name: &str,
            value: &str,
        ) -> Result<()>;
        fn http_request_builder_set_appended_user_agent(
            builder: &mut HttpRequestBuilder,
            user_agent: &str,
        );
        fn http_request_builder_set_resolved_ip_addrs(
            builder: &mut HttpRequestBuilder,
            ips: &[&str],
        ) -> Result<()>;
        unsafe fn http_request_builder_set_body<'r>(
            builder: &'r mut HttpRequestBuilder<'r>,
            body: Box<HttpRequestBody<'r>>,
        );
        unsafe fn http_request_builder_build<'r>(
            builder: &'r mut HttpRequestBuilder<'r>,
        ) -> Box<HttpRequest<'r>>;
        fn http_request_builder_reset(builder: &mut HttpRequestBuilder);

        type HttpRequest<'r>;
        unsafe fn new_http_request_default<'r>() -> Box<HttpRequestBody<'r>>;
        unsafe fn http_request_get_body<'r>(req: &'r HttpRequest<'r>) -> &'r HttpRequestBody<'r>;
        unsafe fn http_request_set_body<'r>(
            req: &'r mut HttpRequest<'r>,
            body: Box<HttpRequestBody<'r>>,
        );
        unsafe fn http_request_get_parts<'r>(req: &'r HttpRequest<'r>) -> &'r HttpRequestParts<'r>;
        unsafe fn http_request_set_parts<'r>(
            req: &'r mut HttpRequest<'r>,
            parts: Box<HttpRequestParts<'r>>,
        );

        type Metrics;
        type MetricsBuilder;
        fn new_metrics_builder() -> Box<MetricsBuilder>;
        fn metrics_get_total_duration(metrics: &Metrics) -> u64;
        fn metrics_set_total_duration(metrics: &mut Metrics, duration: u64);
        fn metrics_get_name_lookup_duration(metrics: &Metrics) -> u64;
        fn metrics_set_name_lookup_duration(metrics: &mut Metrics, duration: u64);
        fn metrics_get_connect_duration(metrics: &Metrics) -> u64;
        fn metrics_set_connect_duration(metrics: &mut Metrics, duration: u64);
        fn metrics_get_secure_connect_duration(metrics: &Metrics) -> u64;
        fn metrics_set_secure_connect_duration(metrics: &mut Metrics, duration: u64);
        fn metrics_get_redirect_duration(metrics: &Metrics) -> u64;
        fn metrics_set_redirect_duration(metrics: &mut Metrics, duration: u64);
        fn metrics_get_transfer_duration(metrics: &Metrics) -> u64;
        fn metrics_set_transfer_duration(metrics: &mut Metrics, duration: u64);
        fn metrics_builder_set_total_duration(builder: &mut MetricsBuilder, duration: u64);
        fn metrics_builder_set_name_lookup_duration(builder: &mut MetricsBuilder, duration: u64);
        fn metrics_builder_set_connect_duration(builder: &mut MetricsBuilder, duration: u64);
        fn metrics_builder_set_secure_connect_duration(builder: &mut MetricsBuilder, duration: u64);
        fn metrics_builder_set_redirect_duration(builder: &mut MetricsBuilder, duration: u64);
        fn metrics_builder_set_transfer_duration(builder: &mut MetricsBuilder, duration: u64);
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
