use super::{ffi::c_void, utils::SeekableReader};
use anyhow::Result as AnyResult;
use qiniu_sdk::etag::{FixedOutput, Reset, Update};
use std::slice::from_raw_parts_mut;

pub fn etag_of(stream: *mut c_void) -> AnyResult<String> {
    let etag = qiniu_sdk::etag::etag_of(SeekableReader::new(stream))?;
    Ok(etag)
}

pub fn etag_to_buf(stream: *mut c_void, buf: *mut c_void) -> AnyResult<()> {
    let buf = unsafe { from_raw_parts_mut(buf.cast::<u8>(), qiniu_sdk::etag::ETAG_SIZE) };
    qiniu_sdk::etag::etag_to_buf(SeekableReader::new(stream), buf.try_into()?)?;
    Ok(())
}

#[derive(Debug)]
pub struct EtagV1(qiniu_sdk::etag::EtagV1);

pub fn new_etag_v1() -> Box<EtagV1> {
    Box::new(EtagV1(Default::default()))
}

pub fn etag_v1_update(etag: &mut EtagV1, data: &[u8]) {
    etag.0.update(data);
}

pub fn etag_v1_finalize_to_buf(etag: &mut EtagV1, buf: *mut c_void) -> AnyResult<()> {
    let buf = unsafe { from_raw_parts_mut(buf.cast::<u8>(), qiniu_sdk::etag::ETAG_SIZE) };
    etag.0.finalize_into_reset(buf.try_into()?);
    Ok(())
}

pub fn etag_v1_reset(etag: &mut EtagV1) {
    etag.0.reset();
}
