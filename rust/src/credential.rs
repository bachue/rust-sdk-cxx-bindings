use super::{ffi::c_void, utils::SeekableReader};
use anyhow::Result as AnyResult;
use qiniu_sdk::http::{HeaderMap, HeaderName, HeaderValue, Method, Uri};
use std::{io::Result as IoResult, time::Duration};

#[derive(Clone, Debug)]
pub struct Credential(qiniu_sdk::credential::Credential);

pub fn new_credential(access_key: &str, secret_key: &str) -> Box<Credential> {
    Box::new(Credential(qiniu_sdk::credential::Credential::new(
        access_key, secret_key,
    )))
}

pub fn credential_clone(credential: &Credential) -> Box<Credential> {
    Box::new(credential.to_owned())
}

pub fn credential_get_access_key(credential: &Credential) -> &str {
    credential.0.access_key().as_ref()
}

pub fn credential_get_secret_key(credential: &Credential) -> &str {
    credential.0.secret_key().as_ref()
}

pub fn credential_sign(credential: &Credential, data: &[u8]) -> String {
    credential.0.sign(data)
}

pub fn credential_sign_with_data(credential: &Credential, data: &[u8]) -> String {
    credential.0.sign_with_data(data)
}

pub fn credential_sign_reader(credential: &Credential, stream: *mut c_void) -> IoResult<String> {
    credential.0.sign_reader(&mut SeekableReader::new(stream))
}

pub fn credential_authorization_v1_for_request(
    credential: &Credential,
    url: &str,
    content_type: &str,
    body: &[u8],
) -> AnyResult<String> {
    let url = parse_uri(url)?;
    let content_type = parse_content_type(content_type)?;
    Ok(credential
        .0
        .authorization_v1_for_request(&url, content_type.as_ref(), body))
}

pub fn credential_authorization_v1_for_request_with_body_reader(
    credential: &Credential,
    url: &str,
    content_type: &str,
    stream: *mut c_void,
) -> AnyResult<String> {
    let url = parse_uri(url)?;
    let content_type = parse_content_type(content_type)?;
    let authorization = credential.0.authorization_v1_for_request_with_body_reader(
        &url,
        content_type.as_ref(),
        &mut SeekableReader::new(stream),
    )?;
    Ok(authorization)
}

pub fn credential_authorization_v2_for_request(
    credential: &Credential,
    method: &str,
    url: &str,
    headers: &[[&str; 2]],
    body: &[u8],
) -> AnyResult<String> {
    let method = parse_method(method)?;
    let url = parse_uri(url)?;
    let headers = parse_header_map(headers)?;
    Ok(credential
        .0
        .authorization_v2_for_request(&method, &url, &headers, body))
}

pub fn credential_authorization_v2_for_request_with_body_reader(
    credential: &Credential,
    method: &str,
    url: &str,
    headers: &[[&str; 2]],
    stream: *mut c_void,
) -> AnyResult<String> {
    let method = parse_method(method)?;
    let url = parse_uri(url)?;
    let headers = parse_header_map(headers)?;
    let authorization = credential.0.authorization_v2_for_request_with_body_reader(
        &method,
        &url,
        &headers,
        &mut SeekableReader::new(stream),
    )?;
    Ok(authorization)
}

pub fn credential_sign_download_url(
    credential: &Credential,
    url: &str,
    duration: u64,
) -> AnyResult<String> {
    let url = parse_uri(url)?;
    let duration = Duration::from_nanos(duration);
    Ok(credential.0.sign_download_url(url, duration).to_string())
}

#[derive(Clone, Debug)]
pub struct CredentialProvider(Box<dyn qiniu_sdk::credential::CredentialProvider>);

impl qiniu_sdk::credential::CredentialProvider for CredentialProvider {
    fn get(
        &self,
        opts: qiniu_sdk::credential::GetOptions,
    ) -> IoResult<qiniu_sdk::credential::GotCredential> {
        self.0.get(opts)
    }
}

#[derive(Clone, Debug)]
pub struct GetCredentialOptions(qiniu_sdk::credential::GetOptions);

#[derive(Clone, Debug)]
pub struct GotCredential(qiniu_sdk::credential::GotCredential);

pub fn credential_provider_get(
    provider: &CredentialProvider,
    options: &GetCredentialOptions,
) -> IoResult<Box<GotCredential>> {
    provider
        .0
        .get(options.0)
        .map(|r| Box::new(GotCredential(r)))
}

pub fn new_static_credential_provider(credential: Box<Credential>) -> Box<CredentialProvider> {
    Box::new(CredentialProvider(Box::new(credential.0)))
}

pub fn credential_provider_clone(provider: &CredentialProvider) -> Box<CredentialProvider> {
    Box::new(provider.to_owned())
}

pub fn got_credential_get_credential(got_credential: &GotCredential) -> Box<Credential> {
    Box::new(Credential(got_credential.0.credential().to_owned()))
}

pub fn new_get_credential_options() -> Box<GetCredentialOptions> {
    Box::new(GetCredentialOptions(
        qiniu_sdk::credential::GetOptions::default(),
    ))
}

pub fn get_credential_options_clone(options: &GetCredentialOptions) -> Box<GetCredentialOptions> {
    Box::new(options.to_owned())
}

pub fn new_global_credential_provider() -> Box<CredentialProvider> {
    Box::new(CredentialProvider(Box::new(
        qiniu_sdk::credential::GlobalCredentialProvider,
    )))
}

pub fn global_credential_provider_setup(credential: &Credential) {
    qiniu_sdk::credential::GlobalCredentialProvider::setup(credential.0.to_owned())
}

pub fn global_credential_provider_clear() {
    qiniu_sdk::credential::GlobalCredentialProvider::clear()
}

pub fn new_env_credential_provider() -> Box<CredentialProvider> {
    Box::new(CredentialProvider(Box::new(
        qiniu_sdk::credential::EnvCredentialProvider,
    )))
}

pub fn env_credential_provider_setup(credential: &Credential) {
    qiniu_sdk::credential::EnvCredentialProvider::setup(&credential.0)
}

pub fn env_credential_provider_clear() {
    qiniu_sdk::credential::EnvCredentialProvider::clear()
}

#[derive(Clone, Debug)]
pub struct ChainCredentialsProviderBuilder(
    Box<qiniu_sdk::credential::ChainCredentialsProviderBuilder>,
);

pub fn new_chain_credentials_provider_builder(
    provider: Box<CredentialProvider>,
) -> Box<ChainCredentialsProviderBuilder> {
    Box::new(ChainCredentialsProviderBuilder(Box::new(
        qiniu_sdk::credential::ChainCredentialsProviderBuilder::new(provider.0),
    )))
}

pub fn chain_credentials_provider_builder_append_credential(
    builder: &mut ChainCredentialsProviderBuilder,
    provider: Box<CredentialProvider>,
) {
    builder.0.append_credential(provider.0);
}

pub fn chain_credentials_provider_builder_prepend_credential(
    builder: &mut ChainCredentialsProviderBuilder,
    provider: Box<CredentialProvider>,
) {
    builder.0.prepend_credential(provider.0);
}

pub fn chain_credentials_provider_builder_build(
    builder: &mut ChainCredentialsProviderBuilder,
) -> Box<CredentialProvider> {
    Box::new(CredentialProvider(Box::new(builder.0.build())))
}

fn parse_uri(url: &str) -> AnyResult<Uri> {
    let url = url.parse::<Uri>()?;
    Ok(url)
}

fn parse_method(method: &str) -> AnyResult<Method> {
    let method = method.parse::<Method>()?;
    Ok(method)
}

fn parse_content_type(content_type: &str) -> AnyResult<Option<HeaderValue>> {
    let content_type = if content_type.is_empty() {
        None
    } else {
        Some(content_type.parse::<HeaderValue>()?)
    };
    Ok(content_type)
}

fn parse_header_name(header_name: &str) -> AnyResult<HeaderName> {
    let header_name = header_name.parse::<HeaderName>()?;
    Ok(header_name)
}

fn parse_header_value(header_value: &str) -> AnyResult<HeaderValue> {
    let header_value = header_value.parse::<HeaderValue>()?;
    Ok(header_value)
}

fn parse_header_map(headers: &[[&str; 2]]) -> AnyResult<HeaderMap> {
    headers
        .into_iter()
        .map(|[header_name, header_value]| {
            let header_name = parse_header_name(header_name)?;
            let header_value = parse_header_value(header_value)?;
            Ok((header_name, header_value))
        })
        .collect()
}
