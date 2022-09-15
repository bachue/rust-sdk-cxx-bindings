use super::{ffi::c_void, utils::SeekableReader};
use std::io::Result as IoResult;

pub struct Credential(qiniu_sdk::credential::Credential);

pub fn new_credential(access_key: &str, secret_key: &str) -> Box<Credential> {
    Box::new(Credential(qiniu_sdk::credential::Credential::new(
        access_key, secret_key,
    )))
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
