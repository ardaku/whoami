#![allow(unsafe_code)]

// Daku
#[cfg_attr(all(target_arch = "wasm32", daku), path = "os/daku.rs")]
// Redox
#[cfg_attr(
    all(target_os = "redox", not(target_arch = "wasm32")),
    path = "os/redox.rs"
)]
// Unix
#[cfg_attr(
    all(
        any(
            target_os = "linux",
            target_os = "macos",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd",
            target_os = "illumos",
            target_os = "hurd",
        ),
        not(target_arch = "wasm32")
    ),
    path = "os/unix.rs"
)]
// Wasi
#[cfg_attr(
    all(target_arch = "wasm32", target_os = "wasi"),
    path = "os/wasi.rs"
)]
// Web
#[cfg_attr(
    all(
        target_arch = "wasm32",
        not(target_os = "wasi"),
        not(daku),
        feature = "web",
    ),
    path = "os/web.rs"
)]
// Windows
#[cfg_attr(
    all(target_os = "windows", not(target_arch = "wasm32")),
    path = "os/windows.rs"
)]
mod target;

use std::{
    env::{self, VarError},
    ffi::OsString,
    io::{Error, ErrorKind},
};

use crate::{Arch, DesktopEnv, Language, LanguagePrefs, Platform, Result};

/// Implement `Target for Os` to add platform support for a target.
pub(crate) struct Os;

/// Target platform support
pub(crate) trait Target: Sized {
    /// Return a semicolon-delimited string of language/COUNTRY codes.
    fn lang_prefs(self) -> Result<LanguagePrefs>;
    /// Return the user's "real" / "full" name.
    fn realname(self) -> Result<OsString>;
    /// Return the user's username.
    fn username(self) -> Result<OsString>;
    /// Return the computer's "fancy" / "pretty" name.
    fn devicename(self) -> Result<OsString>;
    /// Return the computer's hostname.
    fn hostname(self) -> Result<String>;
    /// Return the OS distribution's name.
    fn distro(self) -> Result<String>;
    /// Return the desktop environment.
    fn desktop_env(self) -> Option<DesktopEnv>;
    /// Return the target platform.
    fn platform(self) -> Platform;
    /// Return the computer's CPU architecture.
    fn arch(self) -> Result<Arch>;

    /// Return the user's account name (usually just the username, but may
    /// include an account server hostname).
    fn account(self) -> Result<OsString> {
        self.username()
    }
}

// This is only used on some platforms
#[allow(dead_code)]
fn err_missing_record() -> Error {
    Error::new(ErrorKind::NotFound, "Missing record")
}

// This is only used on some platforms
#[allow(dead_code)]
fn err_null_record() -> Error {
    Error::new(ErrorKind::NotFound, "Null record")
}

// This is only used on some platforms
#[allow(dead_code)]
fn err_empty_record() -> Error {
    Error::new(ErrorKind::NotFound, "Empty record")
}

// This is only used on some platforms
#[allow(dead_code)]
fn unix_lang() -> Result<LanguagePrefs> {
    use std::str::FromStr;

    let env_var = |var: &str| match env::var(var) {
        Ok(value) => Ok(if value.is_empty() { None } else { Some(value) }),
        Err(VarError::NotPresent) => Ok(None),
        Err(VarError::NotUnicode(_)) => Err(ErrorKind::InvalidData),
    };

    // Uses priority defined in
    // <https://www.gnu.org/software/gettext/manual/html_node/Locale-Environment-Variables.html>
    let lc_all = env_var("LC_ALL")?;
    let lang = env_var("LANG")?;

    if lang.is_none() && lc_all.is_none() {
        return Err(err_empty_record());
    }

    // Standard locales that have a higher global precedence than their specific
    // counterparts, indicating that one should not perform any localization.
    // https://www.gnu.org/software/libc/manual/html_node/Standard-Locales.html
    if let Some(l) = &lang {
        if l == "C" || l == "POSIX" {
            return Ok(LanguagePrefs {
                fallbacks: Vec::new(),
                ..Default::default()
            });
        }
    }

    // The LANGUAGE environment variable takes precedence if and only if
    // localization is enabled, i.e., LC_ALL / LANG is not "C" or "POSIX".
    // <https://www.gnu.org/software/gettext/manual/html_node/The-LANGUAGE-variable.html>
    if let Some(language) = env_var("LANGUAGE")? {
        return Ok(LanguagePrefs {
            fallbacks: language
                .split(":")
                .map(Language::from_str)
                .collect::<Result<_>>()?,
            ..Default::default()
        });
    }

    // All fields other than LANGUAGE can only contain a single value, so we
    // don't need to perform any splitting at this point.
    let lang_from_var = |var| -> Result<Option<Language>, Error> {
        env_var(var)?.as_deref().map(Language::from_str).transpose()
    };

    Ok(LanguagePrefs {
        fallbacks: lang
            .as_ref()
            .map(|l| -> Result<_> { Ok([Language::from_str(l)?].to_vec()) })
            .transpose()?
            .unwrap_or(Vec::new()),
        collation: lang_from_var("LC_COLLATE")?,
        char_classes: lang_from_var("LC_CTYPE")?,
        monetary: lang_from_var("LC_MONTEARY")?,
        messages: lang_from_var("LC_MESSAGES")?,
        numeric: lang_from_var("LC_NUMERIC")?,
        time: lang_from_var("LC_TIME")?,
    })
}
