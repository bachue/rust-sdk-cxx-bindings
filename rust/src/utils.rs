use super::ffi::{c_void, new_seekable_reader};
use cxx::UniquePtr;
use std::{
    fmt::{self, Debug},
    io::{Error as IoError, ErrorKind as IoErrorKind, Read, Result as IoResult, Seek, SeekFrom},
};

pub(crate) struct SeekableReader(UniquePtr<super::ffi::SeekableReader>);

impl SeekableReader {
    pub(crate) fn new(stream: *mut c_void) -> Self {
        Self(unsafe { new_seekable_reader(stream.cast()) })
    }
}

impl Debug for SeekableReader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("SeekableReader").finish()
    }
}

impl Read for SeekableReader {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        let mut errbit = 0u8;
        unsafe { self.0.read(buf.as_mut_ptr(), buf.len(), &mut errbit) }
            .map_err(|err| IoError::new(IoErrorKind::Other, err))
            .and_then(|have_read| {
                check_errbit(errbit)?;
                Ok(have_read)
            })
    }
}

impl Seek for SeekableReader {
    fn seek(&mut self, pos: SeekFrom) -> IoResult<u64> {
        let (off, pos) = match pos {
            SeekFrom::Start(off) => (off as i64, 0),
            SeekFrom::Current(off) => (off, 1),
            SeekFrom::End(off) => (off, 2),
        };
        let mut errbit = 0u8;
        unsafe { self.0.seek(off, pos, &mut errbit) }
            .map_err(|err| IoError::new(IoErrorKind::Other, err))
            .and_then(|pos| {
                check_errbit(errbit)?;
                Ok(pos)
            })
    }
}

unsafe impl Send for SeekableReader {}
unsafe impl Sync for SeekableReader {}

fn check_errbit(errbit: u8) -> IoResult<()> {
    if errbit > 0 {
        if errbit & 1 > 0 {
            return Err(IoError::new(
                IoErrorKind::Other,
                "Read/writing error on i/o operation",
            ));
        } else if errbit & 2 > 0 {
            return Err(IoError::new(
                IoErrorKind::Other,
                "Logical error on i/o operation",
            ));
        }
    }
    Ok(())
}
