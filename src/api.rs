use std::{env, ffi::OsString};

use crate::{
    conversions,
    os::{Os, Target},
    Arch, DesktopEnv, LanguagePrefs, Platform, Result,
};

macro_rules! report_message {
    () => {
        "Please report this issue at https://github.com/ardaku/whoami/issues"
    };
}

/// Get the CPU Architecture.
#[inline(always)]
pub fn arch() -> Arch {
    Target::arch(Os).expect(concat!("arch() failed.  ", report_message!()))
}

/// Get the user's account name; usually just the username, but may include an
/// account server hostname.
///
/// If you don't want the account server hostname, use [`username()`].
///
/// Example: `username@example.com`
#[inline(always)]
pub fn account() -> Result<String> {
    account_os().and_then(conversions::string_from_os)
}

/// Get the user's account name; usually just the username, but may include an
/// account server hostname.
///
/// If you don't want the account server hostname, use [`username()`].
///
/// Example: `username@example.com`
#[inline(always)]
pub fn account_os() -> Result<OsString> {
    Target::account(Os)
}

/// Get the user's username.
///
/// On unix-systems this differs from [`realname()`] most notably in that spaces
/// are not allowed in the username.
#[inline(always)]
pub fn username() -> Result<String> {
    username_os().and_then(conversions::string_from_os)
}

/// Get the user's username.
///
/// On unix-systems this differs from [`realname_os()`] most notably in that
/// spaces are not allowed in the username.
#[inline(always)]
pub fn username_os() -> Result<OsString> {
    Target::username(Os)
}

/// Get the user's real (full) name.
#[inline(always)]
pub fn realname() -> Result<String> {
    realname_os().and_then(conversions::string_from_os)
}

/// Get the user's real (full) name.
#[inline(always)]
pub fn realname_os() -> Result<OsString> {
    Target::realname(Os)
}

/// Get the host device's hostname.
///
/// Usually hostnames are case-insensitive, but it's not a hard requirement.
///
/// FIXME: Document platform-specific character limitations
#[inline(always)]
pub fn hostname() -> Result<String> {
    Target::hostname(Os)
}

/// Get the device name (also known as "Pretty Name").
///
/// Often used to identify device for bluetooth pairing.
#[inline(always)]
pub fn devicename() -> Result<String> {
    devicename_os().and_then(conversions::string_from_os)
}

/// Get the device name (also known as "Pretty Name").
///
/// Often used to identify device for bluetooth pairing.
#[inline(always)]
pub fn devicename_os() -> Result<OsString> {
    Target::devicename(Os)
}

/// Get the name of the operating system distribution and (possibly) version.
///
/// Example: "Windows 10" or "Fedora 26 (Workstation Edition)"
#[inline(always)]
pub fn distro() -> Result<String> {
    Target::distro(Os)
}

/// Get the desktop environment (if any).
///
/// Example: "gnome" or "windows"
///
/// Returns `None` if a desktop environment is not available (for example in a
/// TTY or over SSH)
#[inline(always)]
pub fn desktop_env() -> Option<DesktopEnv> {
    if env::var_os("SSH_CLIENT").is_some()
        || env::var_os("SSH_TTY").is_some()
        || env::var_os("SSH_CONNECTION").is_some()
    {
        return None;
    }

    Target::desktop_env(Os)
}

/// Get the platform.
#[inline(always)]
pub fn platform() -> Platform {
    Target::platform(Os)
}

/// Get the user's preferred language(s).
///
/// Returned as a [`LanguagePrefs`].  Unrecognized languages may
/// either return an error or be skipped.
#[inline(always)]
pub fn lang_prefs() -> Result<LanguagePrefs> {
    Target::lang_prefs(Os)
}
