//! Crate for getting the user's username, realname and environment.
//!
//! ## Getting Started
//! Using the whoami crate is super easy!  All of the public items are simple
//! functions with no parameters that return `String`s (with the exception of
//! [`env()`](fn.env.html), and [`platform()`](fn.platform.html) which return
//! enums).  The following example shows how to use all of the functions:
//!
//! ```rust
//! fn main() {
//!     print!(
//!         "user's full name     whoami::user():      {}\n\
//!          username             whoami::username():  {}\n\
//!          host's fancy name    whoami::host():      {}\n\
//!          hostname             whoami::hostname():  {}\n\
//!          platform             whoami::platform():  {}\n\
//!          operating system     whoami::os():        {}\n\
//!          desktop environment  whoami::env():       {}\n\
//!          ",
//!         whoami::user(),
//!         whoami::username(),
//!         whoami::host(),
//!         whoami::hostname(),
//!         whoami::platform(),
//!         whoami::os(),
//!         whoami::env(),
//!     );
//! }
//! ```

#![warn(missing_docs)]
#![doc(
    html_logo_url = "https://libcala.github.io/whoami/icon.svg",
    html_favicon_url = "https://libcala.github.io/whoami/icon.svg"
)]

/// Which Desktop Environment
#[allow(missing_docs)]
#[derive(Debug)]
#[non_exhaustive]
pub enum DesktopEnv {
    Gnome,
    Windows,
    Lxde,
    Openbox,
    Mate,
    Xfce,
    Kde,
    Cinnamon,
    I3,
    Mac,
    Ios,
    Android,
    Wasm,
    Console,
    Ubuntu,
    Dive,
    Fuchsia,
    Redox,
    Unknown(String),
}

impl std::fmt::Display for DesktopEnv {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use self::DesktopEnv::*;

        if let Unknown(_) = self {
            write!(f, "Unknown: ")?;
        }

        write!(
            f,
            "{}",
            match self {
                Gnome => "Gnome",
                Windows => "Windows",
                Lxde => "LXDE",
                Openbox => "Openbox",
                Mate => "Mate",
                Xfce => "XFCE",
                Kde => "KDE",
                Cinnamon => "Cinnamon",
                I3 => "I3",
                Mac => "Mac OS",
                Ios => "IOS",
                Android => "Android",
                Wasm => "Wasm",
                Console => "Console",
                Ubuntu => "Ubuntu",
                Dive => "Dive",
                Fuchsia => "Fuchsia",
                Redox => "Redox",
                Unknown(a) => &a,
            }
        )
    }
}

/// Which Platform
#[allow(missing_docs)]
#[derive(Debug)]
#[non_exhaustive]
pub enum Platform {
    Linux,
    FreeBsd,
    Windows,
    MacOS,
    Ios,
    Android,
    Nintendo,
    Xbox,
    PlayStation,
    Dive,
    Fuchsia,
    Redox,
    Unknown(String),
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use self::Platform::*;

        if let Unknown(_) = self {
            write!(f, "Unknown: ")?;
        }

        write!(
            f,
            "{}",
            match self {
                Linux => "Linux",
                FreeBsd => "Free BSD",
                Windows => "Windows",
                MacOS => "Mac OS",
                Ios => "iOS",
                Android => "Android",
                Nintendo => "Nintendo",
                Xbox => "XBox",
                PlayStation => "PlayStation",
                Dive => "Dive",
                Fuchsia => "Fuchsia",
                Redox => "Redox",
                Unknown(a) => a,
            }
        )
    }
}

#[cfg(all(target_os = "windows", not(target_arch = "wasm32")))]
mod windows;
#[cfg(all(target_os = "windows", not(target_arch = "wasm32")))]
use self::windows as native;
#[cfg(target_arch = "wasm32")]
mod wasm;
#[cfg(target_arch = "wasm32")]
use self::wasm as native;
#[cfg(not(any(target_os = "windows", target_arch = "wasm32")))]
mod unix;
#[cfg(not(any(target_os = "windows", target_arch = "wasm32")))]
use self::unix as native;

/// Get the user's username.
#[inline(always)]
pub fn username() -> String {
    native::username()
}

/// Get the user's full name.
#[inline(always)]
pub fn user() -> String {
    native::realname()
}

/// Get the host device's (pretty) name.
#[inline(always)]
pub fn host() -> String {
    native::computer()
}

/// Get the host device's hostname.
#[inline(always)]
pub fn hostname() -> String {
    native::hostname()
}

/// Get the the operating system name and version.
///
/// Example: "Windows 10" or "Fedora 26 (Workstation Edition)"
#[inline(always)]
pub fn os() -> String {
    native::os().unwrap_or_else(|| "Unknown".to_string())
}

/// Get the desktop environment.
///
/// Example: "gnome" or "windows"
#[inline(always)]
pub fn env() -> DesktopEnv {
    native::env()
}

/// Get the platform.
#[inline(always)]
pub fn platform() -> Platform {
    native::platform()
}
