use super::{
    ffi::{c_void, HttpMethod, HttpVersion},
    utils::SeekableReader,
};
use anyhow::Result as AnyResult;
use qiniu_sdk::http::{HeaderMap, HeaderName, HeaderValue};
use std::{
    borrow::Cow,
    net::{AddrParseError, IpAddr},
    str::FromStr,
    time::Duration,
};

#[derive(Debug)]
#[repr(transparent)]
pub struct HttpRequestPartsBuilder<'r>(qiniu_sdk::http::RequestPartsBuilder<'r>);

pub fn new_http_request_parts_builder<'r>() -> Box<HttpRequestPartsBuilder<'r>> {
    Box::new(HttpRequestPartsBuilder(
        qiniu_sdk::http::RequestPartsBuilder::new(),
    ))
}

pub fn http_request_parts_builder_set_url(
    builder: &mut HttpRequestPartsBuilder,
    url: &str,
) -> AnyResult<()> {
    builder.0.url(url.parse()?);
    Ok(())
}

pub fn http_request_parts_builder_set_version(
    builder: &mut HttpRequestPartsBuilder,
    version: HttpVersion,
) {
    builder.0.version(convert_http_version(version));
}

pub fn http_request_parts_builder_set_method(
    builder: &mut HttpRequestPartsBuilder,
    method: HttpMethod,
) {
    builder.0.method(convert_http_method(method));
}

pub fn http_request_parts_builder_set_headers(
    builder: &mut HttpRequestPartsBuilder,
    headers: &[[&str; 2]],
) -> AnyResult<()> {
    builder.0.headers(make_rust_http_headers(headers)?);
    Ok(())
}

pub fn http_request_parts_builder_set_header(
    builder: &mut HttpRequestPartsBuilder,
    name: &str,
    value: &str,
) -> AnyResult<()> {
    builder
        .0
        .header(HeaderName::try_from(name)?, HeaderValue::try_from(value)?);
    Ok(())
}

pub fn http_request_parts_builder_set_appended_user_agent(
    builder: &mut HttpRequestPartsBuilder,
    user_agent: &str,
) {
    builder.0.appended_user_agent(user_agent);
}

pub fn http_request_parts_builder_set_resolved_ip_addrs(
    builder: &mut HttpRequestPartsBuilder,
    ips: &[&str],
) -> AnyResult<()> {
    let ips = ips
        .iter()
        .map(|ip| IpAddr::from_str(ip))
        .collect::<Result<Vec<_>, _>>()?;
    builder.0.resolved_ip_addrs(ips);
    Ok(())
}

#[derive(Debug)]
#[repr(transparent)]
pub struct HttpRequestParts<'r>(qiniu_sdk::http::RequestParts<'r>);

pub fn http_request_parts_get_url(parts: &HttpRequestParts) -> String {
    parts.0.url().to_string()
}

pub fn http_request_parts_set_url(parts: &mut HttpRequestParts, url: &str) -> AnyResult<()> {
    *parts.0.url_mut() = url.parse()?;
    Ok(())
}

pub fn http_request_parts_get_version(parts: &HttpRequestParts) -> HttpVersion {
    convert_qiniu_sdk_http_version(parts.0.version())
}

pub fn http_request_parts_set_version(parts: &mut HttpRequestParts, version: HttpVersion) {
    *parts.0.version_mut() = convert_http_version(version);
}

pub fn http_request_parts_get_method(parts: &HttpRequestParts) -> HttpMethod {
    convert_qiniu_sdk_http_method(parts.0.method())
}

pub fn http_request_parts_set_method(parts: &mut HttpRequestParts, method: HttpMethod) {
    *parts.0.method_mut() = convert_http_method(method);
}

pub fn http_request_parts_get_header_keys(parts: &HttpRequestParts) -> Vec<String> {
    parts
        .0
        .headers()
        .keys()
        .map(|key| key.to_string())
        .collect()
}

pub fn http_request_parts_get_header<'v>(
    parts: &'v HttpRequestParts,
    key: &'v str,
) -> AnyResult<&'v str> {
    let optional_value = parts
        .0
        .headers()
        .get(&key.parse::<HeaderName>()?)
        .map(|value| value.to_str())
        .transpose()?;
    Ok(optional_value.unwrap_or_default())
}

pub fn http_request_parts_set_headers(
    parts: &mut HttpRequestParts,
    headers: &[[&str; 2]],
) -> AnyResult<()> {
    *parts.0.headers_mut() = make_rust_http_headers(headers)?;
    Ok(())
}

pub fn http_request_parts_get_user_agent(parts: &HttpRequestParts) -> String {
    parts.0.user_agent().to_string()
}

pub fn http_request_parts_get_appended_user_agent(parts: &HttpRequestParts) -> String {
    parts.0.appended_user_agent().to_string()
}

pub fn http_request_parts_set_appended_user_agent(parts: &mut HttpRequestParts, user_agent: &str) {
    *parts.0.appended_user_agent_mut() = user_agent.into();
}

pub fn http_request_parts_get_resolved_ip_addrs(parts: &HttpRequestParts) -> Vec<String> {
    parts
        .0
        .resolved_ip_addrs()
        .unwrap_or_default()
        .iter()
        .map(|ip_addr| ip_addr.to_string())
        .collect()
}

pub fn http_request_parts_set_resolved_ip_addrs(
    parts: &mut HttpRequestParts,
    resolved_ip_addrs: &[&str],
) -> AnyResult<()> {
    *parts.0.resolved_ip_addrs_mut() = Some(Cow::Owned(
        resolved_ip_addrs
            .iter()
            .map(|ip| ip.parse())
            .collect::<Result<_, AddrParseError>>()?,
    ));
    Ok(())
}

pub fn new_request_parts_default<'r>() -> Box<HttpRequestParts<'r>> {
    Box::new(HttpRequestParts(Default::default()))
}

pub fn http_request_parts_builder_build<'r>(
    builder: &mut HttpRequestPartsBuilder<'r>,
) -> Box<HttpRequestParts<'r>> {
    Box::new(HttpRequestParts(builder.0.build()))
}

pub fn http_request_parts_builder_build_with_body<'r>(
    builder: &mut HttpRequestPartsBuilder<'r>,
    body: Box<HttpRequestBody<'r>>,
) -> Box<HttpRequest<'r>> {
    Box::new(HttpRequest(builder.0.build_with_body(body.0)))
}

#[derive(Debug)]
#[repr(transparent)]
pub struct HttpRequestBody<'r>(qiniu_sdk::http::SyncRequestBody<'r>);

pub fn new_http_request_body_from_reader<'r>(
    stream: *mut c_void,
    size: u64,
) -> Box<HttpRequestBody<'r>> {
    Box::new(HttpRequestBody(
        qiniu_sdk::http::SyncRequestBody::from_reader(SeekableReader::new(stream), size),
    ))
}

pub fn new_http_request_body_from_bytes<'r>(bytes: &[u8]) -> Box<HttpRequestBody<'r>> {
    Box::new(HttpRequestBody(
        qiniu_sdk::http::SyncRequestBody::from_bytes(bytes.to_owned()),
    ))
}

pub fn new_http_request_body_default<'r>() -> Box<HttpRequestBody<'r>> {
    Box::new(HttpRequestBody(qiniu_sdk::http::SyncRequestBody::default()))
}

pub fn http_request_body_get_size(body: &HttpRequestBody) -> u64 {
    body.0.size()
}

#[derive(Debug)]
#[repr(transparent)]
pub struct HttpRequestBuilder<'r>(qiniu_sdk::http::SyncRequestBuilder<'r>);

pub fn new_http_request_builder<'r>() -> Box<HttpRequestBuilder<'r>> {
    Box::new(HttpRequestBuilder(qiniu_sdk::http::SyncRequest::builder()))
}

pub fn http_request_builder_set_url(builder: &mut HttpRequestBuilder, url: &str) -> AnyResult<()> {
    builder.0.url(url.parse()?);
    Ok(())
}

pub fn http_request_builder_set_version(builder: &mut HttpRequestBuilder, version: HttpVersion) {
    builder.0.version(convert_http_version(version));
}

pub fn http_request_builder_set_method(builder: &mut HttpRequestBuilder, method: HttpMethod) {
    builder.0.method(convert_http_method(method));
}

pub fn http_request_builder_set_headers(
    builder: &mut HttpRequestBuilder,
    headers: &[[&str; 2]],
) -> AnyResult<()> {
    builder.0.headers(make_rust_http_headers(headers)?);
    Ok(())
}

pub fn http_request_builder_set_header(
    builder: &mut HttpRequestBuilder,
    name: &str,
    value: &str,
) -> AnyResult<()> {
    builder
        .0
        .header(HeaderName::try_from(name)?, HeaderValue::try_from(value)?);
    Ok(())
}

pub fn http_request_builder_set_appended_user_agent(
    builder: &mut HttpRequestBuilder,
    user_agent: &str,
) {
    builder.0.appended_user_agent(user_agent);
}

pub fn http_request_builder_set_resolved_ip_addrs(
    builder: &mut HttpRequestBuilder,
    ips: &[&str],
) -> AnyResult<()> {
    let ips = ips
        .iter()
        .map(|ip| IpAddr::from_str(ip))
        .collect::<Result<Vec<_>, _>>()?;
    builder.0.resolved_ip_addrs(ips);
    Ok(())
}

pub fn http_request_builder_set_body<'r>(
    builder: &'r mut HttpRequestBuilder<'r>,
    body: Box<HttpRequestBody<'r>>,
) {
    builder.0.body(body.0);
}

pub fn http_request_builder_build<'r>(
    builder: &'r mut HttpRequestBuilder<'r>,
) -> Box<HttpRequest<'r>> {
    Box::new(HttpRequest(builder.0.build()))
}

pub fn http_request_builder_reset(builder: &mut HttpRequestBuilder) {
    builder.0.reset();
}

#[derive(Debug)]
#[repr(transparent)]
pub struct HttpRequest<'r>(qiniu_sdk::http::SyncRequest<'r>);

pub fn new_http_request_default<'r>() -> Box<HttpRequestBody<'r>> {
    Box::new(HttpRequestBody(Default::default()))
}

pub fn http_request_get_body<'r>(req: &'r HttpRequest<'r>) -> &'r HttpRequestBody<'r> {
    unsafe { &*(req.0.body() as *const qiniu_sdk::http::SyncRequestBody as *const HttpRequestBody) }
}

pub fn http_request_set_body<'r>(req: &'r mut HttpRequest<'r>, body: Box<HttpRequestBody<'r>>) {
    *req.0.body_mut() = body.0;
}

pub fn http_request_get_parts<'r>(req: &'r HttpRequest<'r>) -> &'r HttpRequestParts<'r> {
    unsafe { &*(req.0.parts() as *const qiniu_sdk::http::RequestParts as *const HttpRequestParts) }
}

pub fn http_request_set_parts<'r>(req: &'r mut HttpRequest<'r>, parts: Box<HttpRequestParts<'r>>) {
    *req.0.parts_mut() = parts.0;
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Metrics(qiniu_sdk::http::Metrics);

#[derive(Debug)]
#[repr(transparent)]
pub struct MetricsBuilder(qiniu_sdk::http::MetricsBuilder);

pub fn new_metrics_builder() -> Box<MetricsBuilder> {
    Box::new(MetricsBuilder(Default::default()))
}

pub fn metrics_get_total_duration(metrics: &Metrics) -> u64 {
    metrics
        .0
        .total_duration()
        .map(|d| d.as_nanos() as u64)
        .unwrap_or_default()
}

pub fn metrics_set_total_duration(metrics: &mut Metrics, duration: u64) {
    *metrics.0.total_duration_mut() = make_optional_duration(duration);
}

pub fn metrics_get_name_lookup_duration(metrics: &Metrics) -> u64 {
    metrics
        .0
        .name_lookup_duration()
        .map(|d| d.as_nanos() as u64)
        .unwrap_or_default()
}

pub fn metrics_set_name_lookup_duration(metrics: &mut Metrics, duration: u64) {
    *metrics.0.name_lookup_duration_mut() = make_optional_duration(duration);
}

pub fn metrics_get_connect_duration(metrics: &Metrics) -> u64 {
    metrics
        .0
        .connect_duration()
        .map(|d| d.as_nanos() as u64)
        .unwrap_or_default()
}

pub fn metrics_set_connect_duration(metrics: &mut Metrics, duration: u64) {
    *metrics.0.connect_duration_mut() = make_optional_duration(duration);
}

pub fn metrics_get_secure_connect_duration(metrics: &Metrics) -> u64 {
    metrics
        .0
        .secure_connect_duration()
        .map(|d| d.as_nanos() as u64)
        .unwrap_or_default()
}

pub fn metrics_set_secure_connect_duration(metrics: &mut Metrics, duration: u64) {
    *metrics.0.secure_connect_duration_mut() = make_optional_duration(duration);
}

pub fn metrics_get_redirect_duration(metrics: &Metrics) -> u64 {
    metrics
        .0
        .redirect_duration()
        .map(|d| d.as_nanos() as u64)
        .unwrap_or_default()
}

pub fn metrics_set_redirect_duration(metrics: &mut Metrics, duration: u64) {
    *metrics.0.redirect_duration_mut() = make_optional_duration(duration);
}

pub fn metrics_get_transfer_duration(metrics: &Metrics) -> u64 {
    metrics
        .0
        .transfer_duration()
        .map(|d| d.as_nanos() as u64)
        .unwrap_or_default()
}

pub fn metrics_set_transfer_duration(metrics: &mut Metrics, duration: u64) {
    *metrics.0.transfer_duration_mut() = make_optional_duration(duration);
}

pub fn metrics_builder_set_total_duration(builder: &mut MetricsBuilder, duration: u64) {
    builder
        .0
        .total_duration(make_optional_duration(duration).unwrap_or_default());
}

pub fn metrics_builder_set_name_lookup_duration(builder: &mut MetricsBuilder, duration: u64) {
    builder
        .0
        .name_lookup_duration(make_optional_duration(duration).unwrap_or_default());
}

pub fn metrics_builder_set_connect_duration(builder: &mut MetricsBuilder, duration: u64) {
    builder
        .0
        .connect_duration(make_optional_duration(duration).unwrap_or_default());
}

pub fn metrics_builder_set_secure_connect_duration(builder: &mut MetricsBuilder, duration: u64) {
    builder
        .0
        .secure_connect_duration(make_optional_duration(duration).unwrap_or_default());
}

pub fn metrics_builder_set_redirect_duration(builder: &mut MetricsBuilder, duration: u64) {
    builder
        .0
        .redirect_duration(make_optional_duration(duration).unwrap_or_default());
}

pub fn metrics_builder_set_transfer_duration(builder: &mut MetricsBuilder, duration: u64) {
    builder
        .0
        .transfer_duration(make_optional_duration(duration).unwrap_or_default());
}

fn make_optional_duration(duration: u64) -> Option<Duration> {
    if duration == 0 {
        None
    } else {
        Some(Duration::from_nanos(duration))
    }
}

fn convert_http_version(version: HttpVersion) -> qiniu_sdk::http::Version {
    match version {
        HttpVersion::HTTP_VERSION_09 => qiniu_sdk::http::Version::HTTP_09,
        HttpVersion::HTTP_VERSION_10 => qiniu_sdk::http::Version::HTTP_10,
        HttpVersion::HTTP_VERSION_11 => qiniu_sdk::http::Version::HTTP_11,
        HttpVersion::HTTP_VERSION_2 => qiniu_sdk::http::Version::HTTP_2,
        HttpVersion::HTTP_VERSION_3 => qiniu_sdk::http::Version::HTTP_3,
        _ => panic!("Unrecognized http version: {:?}", version),
    }
}

fn convert_qiniu_sdk_http_version(version: qiniu_sdk::http::Version) -> HttpVersion {
    match version {
        qiniu_sdk::http::Version::HTTP_09 => HttpVersion::HTTP_VERSION_09,
        qiniu_sdk::http::Version::HTTP_10 => HttpVersion::HTTP_VERSION_10,
        qiniu_sdk::http::Version::HTTP_11 => HttpVersion::HTTP_VERSION_11,
        qiniu_sdk::http::Version::HTTP_2 => HttpVersion::HTTP_VERSION_2,
        qiniu_sdk::http::Version::HTTP_3 => HttpVersion::HTTP_VERSION_3,
        _ => panic!("Unrecognized http version: {:?}", version),
    }
}

fn convert_http_method(method: HttpMethod) -> qiniu_sdk::http::Method {
    match method {
        HttpMethod::HTTP_METHOD_OPTIONS => qiniu_sdk::http::Method::OPTIONS,
        HttpMethod::HTTP_METHOD_GET => qiniu_sdk::http::Method::GET,
        HttpMethod::HTTP_METHOD_POST => qiniu_sdk::http::Method::POST,
        HttpMethod::HTTP_METHOD_PUT => qiniu_sdk::http::Method::PUT,
        HttpMethod::HTTP_METHOD_DELETE => qiniu_sdk::http::Method::DELETE,
        HttpMethod::HTTP_METHOD_HEAD => qiniu_sdk::http::Method::HEAD,
        HttpMethod::HTTP_METHOD_TRACE => qiniu_sdk::http::Method::TRACE,
        HttpMethod::HTTP_METHOD_CONNECT => qiniu_sdk::http::Method::CONNECT,
        HttpMethod::HTTP_METHOD_PATCH => qiniu_sdk::http::Method::PATCH,
        _ => panic!("Unrecognized http method: {:?}", method),
    }
}

fn convert_qiniu_sdk_http_method(method: &qiniu_sdk::http::Method) -> HttpMethod {
    match method {
        &qiniu_sdk::http::Method::OPTIONS => HttpMethod::HTTP_METHOD_OPTIONS,
        &qiniu_sdk::http::Method::GET => HttpMethod::HTTP_METHOD_GET,
        &qiniu_sdk::http::Method::POST => HttpMethod::HTTP_METHOD_POST,
        &qiniu_sdk::http::Method::PUT => HttpMethod::HTTP_METHOD_PUT,
        &qiniu_sdk::http::Method::DELETE => HttpMethod::HTTP_METHOD_DELETE,
        &qiniu_sdk::http::Method::HEAD => HttpMethod::HTTP_METHOD_HEAD,
        &qiniu_sdk::http::Method::TRACE => HttpMethod::HTTP_METHOD_TRACE,
        &qiniu_sdk::http::Method::CONNECT => HttpMethod::HTTP_METHOD_CONNECT,
        &qiniu_sdk::http::Method::PATCH => HttpMethod::HTTP_METHOD_PATCH,
        _ => panic!("Unrecognized http method: {:?}", method),
    }
}

fn make_rust_http_headers(headers: &[[&str; 2]]) -> AnyResult<HeaderMap> {
    headers
        .iter()
        .map::<AnyResult<(HeaderName, HeaderValue)>, _>(|[key, value]| {
            Ok((key.parse()?, value.parse()?))
        })
        .collect()
}
