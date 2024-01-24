#[cfg(all(not(target_os = "windows"), not(target_os = "wasi")))]
use std::os::unix::ffi::OsStringExt;
#[cfg(all(not(target_os = "windows"), target_os = "wasi"))]
use std::os::wasi::ffi::OsStringExt;
use std::{
    ffi::OsString,
    io::{Error, ErrorKind},
};

use crate::Result;

pub(crate) fn string_from_os(string: OsString) -> Result<String> {
    #[cfg(not(target_os = "windows"))]
    {
        String::from_utf8(string.into_vec())
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))
    }

    #[cfg(target_os = "windows")]
    {
        string.into_string().map_err(|_| {
            Error::new(ErrorKind::InvalidData, "Not valid unicode")
        })
    }
}
