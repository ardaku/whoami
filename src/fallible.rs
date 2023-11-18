//! Fallible versions of the whoami APIs.
//!
//! Some of the functions in the root module will return "Unknown" or
//! "localhost" on error.  This might not be desirable in some situations.  The
//! functions in this module all return a [`Result`].

use std::ffi::OsString;

use crate::{os, Result};

/// Get the user's username.
///
/// On unix-systems this differs from [`realname()`] most notably in that spaces
/// are not allowed in the username.
#[inline(always)]
pub fn username() -> Result<String> {
    os::username()
}

/// Get the user's username.
///
/// On unix-systems this differs from [`realname_os()`] most notably in that
/// spaces are not allowed in the username.
#[inline(always)]
pub fn username_os() -> Result<OsString> {
    os::username_os()
}

/// Get the user's real (full) name.
#[inline(always)]
pub fn realname() -> Result<String> {
    os::realname()
}

/// Get the user's real (full) name.
#[inline(always)]
pub fn realname_os() -> Result<OsString> {
    os::realname_os()
}

/// Get the name of the operating system distribution and (possibly) version.
///
/// Example: "Windows 10" or "Fedora 26 (Workstation Edition)"
#[inline(always)]
pub fn distro() -> Result<String> {
    os::distro()
}

/// Get the name of the operating system distribution and (possibly) version.
///
/// Example: "Windows 10" or "Fedora 26 (Workstation Edition)"
#[inline(always)]
pub fn distro_os() -> Result<OsString> {
    os::distro_os()
}

/// Get the device name (also known as "Pretty Name").
///
/// Often used to identify device for bluetooth pairing.
#[inline(always)]
pub fn devicename() -> Result<String> {
    os::devicename()
}

/// Get the device name (also known as "Pretty Name").
///
/// Often used to identify device for bluetooth pairing.
#[inline(always)]
pub fn devicename_os() -> Result<OsString> {
    os::devicename_os()
}

/// Get the host device's hostname.
///
/// Limited to a-z (case insensitve), 0-9, and dashes.  This limit also applies
/// to `devicename()` when targeting Windows.  Since the hostname is
/// case-insensitive, this method normalizes to lowercase (unlike
/// [`devicename()`]).
#[inline(always)]
pub fn hostname() -> Result<String> {
    let mut hostname = os::hostname()?;

    hostname.make_ascii_lowercase();

    Ok(hostname)
}

/// Get the host device's hostname.
///
/// Limited to a-z (case insensitve), 0-9, and dashes.  This limit also applies
/// to `devicename()` when targeting Windows.  Since the hostname is
/// case-insensitive, this method normalizes to lowercase (unlike
/// [`devicename()`]).
#[inline(always)]
pub fn hostname_os() -> Result<OsString> {
    Ok(hostname()?.into())
}
