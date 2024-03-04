use std::ffi::OsString;

use crate::{
    conversions,
    os::{Os, Target},
    Arch, DesktopEnv, Language, Platform, Result,
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
/// Limited to a-z, A-Z, 0-9, and dashes.  This limit also applies to
/// [`devicename()`] when targeting Windows.  Usually hostnames are
/// case-insensitive, but it's not a hard requirement.
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

/// Get the desktop environment.
///
/// Example: "gnome" or "windows"
#[inline(always)]
pub fn desktop_env() -> DesktopEnv {
    Target::desktop_env(Os)
}

/// Get the platform.
#[inline(always)]
pub fn platform() -> Platform {
    Target::platform(Os)
}

/// Get the user's preferred language(s).
///
/// Returned as iterator of [`Language`]s.  The most preferred language is
/// returned first, followed by next preferred, and so on.  Unrecognized
/// languages may either return an error or be skipped.
#[inline(always)]
pub fn langs() -> Result<impl Iterator<Item = Language>> {
    // FIXME: Could do less allocation
    let langs = Target::langs(Os)?;
    let langs = langs
        .split(';')
        .map(ToString::to_string)
        .collect::<Vec<_>>();

    Ok(langs.into_iter().filter_map(|lang| {
        let lang = lang
            .split_terminator('.')
            .next()
            .unwrap_or_default()
            .replace(|x| ['_', '-'].contains(&x), "/");

        if lang == "C" {
            return None;
        }

        Some(Language::__(Box::new(lang)))
    }))
}
