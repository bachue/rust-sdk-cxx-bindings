use super::credential::CredentialProvider;
use anyhow::Result as AnyResult;
use qiniu_sdk::upload_token::UploadTokenProviderExt;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug)]
pub struct UploadPolicy(qiniu_sdk::upload_token::UploadPolicy);

#[derive(Clone, Debug)]
pub struct UploadPolicyBuilder(qiniu_sdk::upload_token::UploadPolicyBuilder);

pub fn new_upload_policy_for_bucket(bucket: &str, lifetime: u64) -> Box<UploadPolicyBuilder> {
    Box::new(UploadPolicyBuilder(
        qiniu_sdk::upload_token::UploadPolicy::new_for_bucket(
            bucket,
            Duration::from_nanos(lifetime),
        ),
    ))
}

pub fn new_upload_policy_for_object(
    bucket: &str,
    object: &str,
    lifetime: u64,
) -> Box<UploadPolicyBuilder> {
    Box::new(UploadPolicyBuilder(
        qiniu_sdk::upload_token::UploadPolicy::new_for_object(
            bucket,
            object,
            Duration::from_nanos(lifetime),
        ),
    ))
}

pub fn new_upload_policy_for_objects_with_prefix(
    bucket: &str,
    prefix: &str,
    lifetime: u64,
) -> Box<UploadPolicyBuilder> {
    Box::new(UploadPolicyBuilder(
        qiniu_sdk::upload_token::UploadPolicy::new_for_objects_with_prefix(
            bucket,
            prefix,
            Duration::from_nanos(lifetime),
        ),
    ))
}

pub fn upload_policy_builder_set_token_lifetime(
    builder: &mut UploadPolicyBuilder,
    token_lifetime: u64,
) {
    builder
        .0
        .token_lifetime(Duration::from_nanos(token_lifetime));
}

pub fn upload_policy_builder_set_token_deadline(
    builder: &mut UploadPolicyBuilder,
    token_deadline: u64,
) {
    builder
        .0
        .token_deadline(SystemTime::UNIX_EPOCH + Duration::from_nanos(token_deadline));
}

pub fn upload_policy_builder_clone(builder: &UploadPolicyBuilder) -> Box<UploadPolicyBuilder> {
    Box::new(UploadPolicyBuilder(builder.0.to_owned()))
}

pub fn upload_policy_builder_set_insert_only(builder: &mut UploadPolicyBuilder) {
    builder.0.insert_only();
}

pub fn upload_policy_builder_enable_mime_detection(builder: &mut UploadPolicyBuilder) {
    builder.0.enable_mime_detection();
}

pub fn upload_policy_builder_disable_mime_detection(builder: &mut UploadPolicyBuilder) {
    builder.0.disable_mime_detection();
}

pub fn upload_policy_builder_set_file_type(builder: &mut UploadPolicyBuilder, file_type: u8) {
    builder.0.file_type(file_type.into());
}

pub fn upload_policy_builder_set_return_url(builder: &mut UploadPolicyBuilder, url: &str) {
    builder.0.return_url(url);
}

pub fn upload_policy_builder_set_return_body(builder: &mut UploadPolicyBuilder, body: &str) {
    builder.0.return_body(body);
}

pub fn upload_policy_builder_set_callback(
    builder: &mut UploadPolicyBuilder,
    urls: &[&str],
    host: &str,
    body: &str,
    body_type: &str,
) {
    builder.0.callback(urls, host, body, body_type);
}

pub fn upload_policy_builder_set_save_as(
    builder: &mut UploadPolicyBuilder,
    save_as: &str,
    force: bool,
) {
    builder.0.save_as(save_as, force);
}

pub fn upload_policy_builder_set_file_size_limitation(
    builder: &mut UploadPolicyBuilder,
    min_file_size: u64,
    max_file_size: u64,
) {
    match (min_file_size, max_file_size) {
        (0, 0) => builder.0.file_size_limitation(..),
        (min, 0) => builder.0.file_size_limitation(min..),
        (0, max) => builder.0.file_size_limitation(..max),
        (min, max) => builder.0.file_size_limitation(min..max),
    };
}

pub fn upload_policy_builder_set_mime_types(
    builder: &mut UploadPolicyBuilder,
    mime_types: &[&str],
) {
    builder.0.mime_types(mime_types);
}

pub fn upload_policy_builder_set_object_lifetime(
    builder: &mut UploadPolicyBuilder,
    object_lifetime: u64,
) {
    builder
        .0
        .object_lifetime(Duration::from_nanos(object_lifetime));
}

pub fn upload_policy_builder_set_string(
    builder: &mut UploadPolicyBuilder,
    key: String,
    value: String,
) {
    builder.0.set(key, value.into());
}

pub fn upload_policy_builder_set_integer(
    builder: &mut UploadPolicyBuilder,
    key: String,
    value: i64,
) {
    builder.0.set(key, value.into());
}

pub fn upload_policy_builder_set_bool(builder: &mut UploadPolicyBuilder, key: String, value: bool) {
    builder.0.set(key, value.into());
}

pub fn upload_policy_builder_unset(builder: &mut UploadPolicyBuilder, key: &str) {
    builder.0.unset(key);
}

pub fn upload_policy_builder_reset(builder: &mut UploadPolicyBuilder) {
    builder.0.reset();
}

pub fn upload_policy_builder_build(builder: &UploadPolicyBuilder) -> Box<UploadPolicy> {
    Box::new(UploadPolicy(builder.0.build()))
}

pub fn upload_policy_clone(policy: &UploadPolicy) -> Box<UploadPolicy> {
    Box::new(UploadPolicy(policy.0.to_owned()))
}

pub fn upload_policy_get_bucket(policy: &UploadPolicy) -> &str {
    policy.0.bucket().unwrap_or_default()
}

pub fn upload_policy_get_key(policy: &UploadPolicy) -> &str {
    policy.0.key().unwrap_or_default()
}

pub fn upload_policy_has_prefixal_object_key(policy: &UploadPolicy) -> bool {
    policy.0.use_prefixal_object_key()
}

pub fn upload_policy_is_insert_only(policy: &UploadPolicy) -> bool {
    policy.0.is_insert_only()
}

pub fn upload_policy_is_mime_detection_enabled(policy: &UploadPolicy) -> bool {
    policy.0.mime_detection_enabled()
}

pub fn upload_policy_get_token_deadline(policy: &UploadPolicy) -> AnyResult<u64> {
    let d = policy
        .0
        .token_deadline()
        .map(|d| d.duration_since(UNIX_EPOCH))
        .transpose()?
        .map(|d| d.as_secs())
        .unwrap_or_default();
    Ok(d)
}

pub fn upload_policy_get_return_url(policy: &UploadPolicy) -> &str {
    policy.0.return_url().unwrap_or_default()
}

pub fn upload_policy_get_return_body(policy: &UploadPolicy) -> &str {
    policy.0.return_body().unwrap_or_default()
}

pub fn upload_policy_get_callback_urls(policy: &UploadPolicy) -> Vec<String> {
    policy
        .0
        .callback_urls()
        .map(|urls| urls.map(|url| url.to_owned()).collect())
        .unwrap_or_default()
}

pub fn upload_policy_get_callback_host(policy: &UploadPolicy) -> &str {
    policy.0.callback_host().unwrap_or_default()
}

pub fn upload_policy_get_callback_body(policy: &UploadPolicy) -> &str {
    policy.0.callback_body().unwrap_or_default()
}

pub fn upload_policy_get_callback_body_type(policy: &UploadPolicy) -> &str {
    policy.0.callback_body_type().unwrap_or_default()
}

pub fn upload_policy_get_save_key(policy: &UploadPolicy) -> &str {
    policy.0.save_key().unwrap_or_default()
}

pub fn upload_policy_is_save_key_forced(policy: &UploadPolicy) -> bool {
    policy.0.is_save_key_forced()
}

pub fn upload_policy_get_min_file_size(policy: &UploadPolicy) -> u64 {
    policy.0.file_size_limitation().0.unwrap_or_default()
}

pub fn upload_policy_get_max_file_size(policy: &UploadPolicy) -> u64 {
    policy.0.file_size_limitation().1.unwrap_or_default()
}

pub fn upload_policy_get_mime_types(policy: &UploadPolicy) -> Vec<String> {
    policy
        .0
        .mime_types()
        .map(|mime_types| mime_types.map(|mime_type| mime_type.to_owned()).collect())
        .unwrap_or_default()
}

pub fn upload_policy_get_file_type(policy: &UploadPolicy) -> u8 {
    policy.0.file_type().map(u8::from).unwrap_or_default()
}

pub fn upload_policy_get_object_lifetime(policy: &UploadPolicy) -> u64 {
    policy
        .0
        .object_lifetime()
        .map(|d| d.as_secs())
        .unwrap_or_default()
}

pub fn upload_policy_as_json(policy: &UploadPolicy) -> String {
    policy.0.as_json()
}

pub fn upload_policy_from_json(json: &str) -> AnyResult<Box<UploadPolicy>> {
    Ok(Box::new(UploadPolicy(
        qiniu_sdk::upload_token::UploadPolicy::from_json(json)?,
    )))
}

pub fn upload_policy_get_string<'a>(policy: &'a UploadPolicy, key: &'a str) -> &'a str {
    policy
        .0
        .get(key)
        .and_then(|v| v.as_str())
        .unwrap_or_default()
}

pub fn upload_policy_get_integer<'a>(policy: &'a UploadPolicy, key: &'a str) -> i64 {
    policy
        .0
        .get(key)
        .and_then(|v| v.as_i64())
        .unwrap_or_default()
}

pub fn upload_policy_get_bool<'a>(policy: &'a UploadPolicy, key: &'a str) -> bool {
    policy
        .0
        .get(key)
        .and_then(|v| v.as_bool())
        .unwrap_or_default()
}

pub fn upload_policy_has_key<'a>(policy: &'a UploadPolicy, key: &'a str) -> bool {
    policy.0.get(key).is_some()
}

pub fn upload_policy_is_string<'a>(policy: &'a UploadPolicy, key: &'a str) -> bool {
    policy.0.get(key).map(|v| v.is_string()).unwrap_or_default()
}

pub fn upload_policy_is_integer<'a>(policy: &'a UploadPolicy, key: &'a str) -> bool {
    policy.0.get(key).map(|v| v.is_i64()).unwrap_or_default()
}

pub fn upload_policy_is_bool<'a>(policy: &'a UploadPolicy, key: &'a str) -> bool {
    policy
        .0
        .get(key)
        .map(|v| v.is_boolean())
        .unwrap_or_default()
}

#[derive(Clone, Debug)]
pub struct UploadTokenProvider(Box<dyn qiniu_sdk::upload_token::UploadTokenProvider>);

#[derive(Clone, Debug)]
pub struct GetAccessKeyOptions(qiniu_sdk::upload_token::GetAccessKeyOptions);

#[derive(Clone, Debug)]
pub struct GetPolicyOptions(qiniu_sdk::upload_token::GetPolicyOptions);

#[derive(Clone, Debug)]
pub struct ToUploadTokenStringOptions(qiniu_sdk::upload_token::ToStringOptions);

#[derive(Clone, Debug)]
pub struct GotUploadPolicy<'a>(qiniu_sdk::upload_token::GotUploadPolicy<'a>);

pub fn new_get_access_key_options() -> Box<GetAccessKeyOptions> {
    Box::new(GetAccessKeyOptions(Default::default()))
}

pub fn get_access_key_options_clone(options: &GetAccessKeyOptions) -> Box<GetAccessKeyOptions> {
    Box::new(options.to_owned())
}

pub fn new_get_policy_options() -> Box<GetPolicyOptions> {
    Box::new(GetPolicyOptions(Default::default()))
}

pub fn get_policy_options_clone(options: &GetPolicyOptions) -> Box<GetPolicyOptions> {
    Box::new(options.to_owned())
}

pub fn new_to_upload_token_string_options() -> Box<ToUploadTokenStringOptions> {
    Box::new(ToUploadTokenStringOptions(Default::default()))
}

pub fn to_upload_token_string_options_clone(
    options: &ToUploadTokenStringOptions,
) -> Box<ToUploadTokenStringOptions> {
    Box::new(options.to_owned())
}

pub fn got_upload_policy_get_upload_policy(
    got_upload_policy: &GotUploadPolicy,
) -> Box<UploadPolicy> {
    Box::new(UploadPolicy(got_upload_policy.0.upload_policy().to_owned()))
}

pub fn new_static_upload_token_provider(upload_token: &str) -> Box<UploadTokenProvider> {
    Box::new(UploadTokenProvider(Box::new(
        qiniu_sdk::upload_token::StaticUploadTokenProvider::new(upload_token),
    )))
}

pub fn new_upload_token_provider_from_upload_policy(
    upload_policy: Box<UploadPolicy>,
    credential: Box<CredentialProvider>,
) -> Box<UploadTokenProvider> {
    Box::new(UploadTokenProvider(Box::new(
        qiniu_sdk::upload_token::FromUploadPolicy::new(upload_policy.0, credential),
    )))
}

pub fn upload_token_provider_clone(provider: &UploadTokenProvider) -> Box<UploadTokenProvider> {
    Box::new(UploadTokenProvider(provider.0.to_owned()))
}

pub fn upload_token_provider_get_access_key(
    provider: &UploadTokenProvider,
    opts: &GetAccessKeyOptions,
) -> AnyResult<String> {
    Ok(provider
        .0
        .access_key(opts.0.to_owned())?
        .into_access_key()
        .to_string())
}

pub fn upload_token_provider_get_policy<'a>(
    provider: &'a UploadTokenProvider,
    opts: &'a GetPolicyOptions,
) -> AnyResult<Box<GotUploadPolicy<'a>>> {
    let got_policy = provider.0.policy(opts.0.to_owned())?;
    Ok(Box::new(GotUploadPolicy(got_policy)))
}

pub fn upload_token_provider_to_token_string(
    provider: &UploadTokenProvider,
    opts: &ToUploadTokenStringOptions,
) -> AnyResult<String> {
    let token_string = provider.0.to_token_string(opts.0.to_owned())?;
    Ok(token_string.into_owned())
}

pub fn upload_token_provider_get_bucket_name(
    provider: &UploadTokenProvider,
    opts: &GetPolicyOptions,
) -> AnyResult<String> {
    let bucket_name = provider.0.bucket_name(opts.0.to_owned())?;
    Ok(bucket_name.to_string())
}

pub struct BucketUploadTokenProviderBuilder(
    qiniu_sdk::upload_token::BucketUploadTokenProviderBuilder<CredentialProvider>,
);

pub fn new_bucket_upload_token_provider_builder(
    bucket: &str,
    upload_token_lifetime: u64,
    provider: Box<CredentialProvider>,
) -> Box<BucketUploadTokenProviderBuilder> {
    Box::new(BucketUploadTokenProviderBuilder(
        qiniu_sdk::upload_token::BucketUploadTokenProvider::builder(
            bucket,
            Duration::from_nanos(upload_token_lifetime),
            *provider,
        ),
    ))
}

pub fn bucket_upload_token_provider_builder_build(
    builder: Box<BucketUploadTokenProviderBuilder>,
) -> Box<UploadTokenProvider> {
    Box::new(UploadTokenProvider(Box::new(builder.0.build())))
}

pub struct ObjectUploadTokenProviderBuilder(
    qiniu_sdk::upload_token::ObjectUploadTokenProviderBuilder<CredentialProvider>,
);

pub fn new_object_upload_token_provider_builder(
    bucket: &str,
    object: &str,
    upload_token_lifetime: u64,
    provider: Box<CredentialProvider>,
) -> Box<ObjectUploadTokenProviderBuilder> {
    Box::new(ObjectUploadTokenProviderBuilder(
        qiniu_sdk::upload_token::ObjectUploadTokenProvider::builder(
            bucket,
            object,
            Duration::from_nanos(upload_token_lifetime),
            *provider,
        ),
    ))
}

pub fn object_upload_token_provider_builder_build(
    builder: Box<ObjectUploadTokenProviderBuilder>,
) -> Box<UploadTokenProvider> {
    Box::new(UploadTokenProvider(Box::new(builder.0.build())))
}
