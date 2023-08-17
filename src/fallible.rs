//! Fallible versions of the whoami APIs.
//!
//! Some of the functions in the root module will return "Unknown" on error.
//! This might not be desirable in some situations.  The functions in this
//! module all return a [`Result`].

use std::ffi::OsString;

use crate::{platform, Result};

/// Get the user's username.
///
/// On unix-systems this differs from [`realname()`] most notably in that spaces
/// are not allowed.
#[inline(always)]
pub fn username() -> Result<String> {
    platform::username()
}

/// Get the user's username.
///
/// On unix-systems this differs from [`realname()`] most notably in that spaces
/// are not allowed.
#[inline(always)]
pub fn username_os() -> Result<OsString> {
    platform::username_os()
}

/// Get the user's real (full) name.
#[inline(always)]
pub fn realname() -> Result<String> {
    platform::realname()
}

/// Get the user's real (full) name.
#[inline(always)]
pub fn realname_os() -> Result<OsString> {
    platform::realname_os()
}
