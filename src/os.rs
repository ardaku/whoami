#![allow(unsafe_code)]

// Unix
#[cfg_attr(
    not(any(target_os = "windows", target_arch = "wasm32")),
    path = "os/unix.rs"
)]
// Wasm32 (Daku) - FIXME: Currently routes to fake.rs
#[cfg_attr(
    all(target_arch = "wasm32", target_os = "daku"),
    path = "os/fake.rs"
)]
// Wasm32 (Wasi) - FIXME: Currently routes to fake.rs
#[cfg_attr(
    all(target_arch = "wasm32", target_os = "wasi"),
    path = "os/fake.rs"
)]
// Wasm32 (Web)
#[cfg_attr(
    all(
        target_arch = "wasm32",
        not(target_os = "wasi"),
        not(target_os = "daku"),
        feature = "web",
    ),
    path = "os/web.rs"
)]
// Wasm32 (Unknown)
#[cfg_attr(
    all(
        target_arch = "wasm32",
        not(target_os = "wasi"),
        not(target_os = "daku"),
        not(feature = "web"),
    ),
    path = "os/fake.rs"
)]
// Windows
#[cfg_attr(
    all(target_os = "windows", not(target_arch = "wasm32")),
    path = "os/windows.rs"
)]
mod target;

use std::ffi::OsString;

pub(crate) use self::target::*;
use crate::{Arch, DesktopEnv, Language, Platform, Result};

/// Implement `Target for Os` to add platform support for a target.
pub(crate) struct Os;

/// Target platform support
pub(crate) trait Target {
    /// Return a list of languages.
    fn langs(self) -> Vec<Language>;
    /// Return the user's username.
    fn username(self) -> Result<OsString>;
    /// Return the user's "real" / "full" name.
    fn realname(self) -> Result<OsString>;
    /// Return the computer's "fancy" / "pretty" name.
    fn devicename(self) -> Result<OsString>;
    /// Return the OS distribution's name.
    fn distro(self) -> Result<OsString>;
    /// Return the computer's hostname.
    fn hostname(self) -> Result<String>;
    /// Return the desktop environment.
    fn desktop_env(self) -> DesktopEnv;
    /// Return the target platform.
    fn platform(self) -> Platform;
    /// Return the computer's CPU architecture.
    fn arch(self) -> Result<Arch>;
}
