//! Crate for getting the user's username, realname and environment.
//!
//! ## Getting Started
//! Using the whoami crate is super easy!  All of the public items are simple functions with no parameters that return `String`s (with the exception of `env`, which returns an enum).  The following example shows how to use all of the functions:
//!
//! ```rust
//! fn main() {
//!     print!(
//!         "--------------------------------------\n\
//!          user's full name (user):              {}\n\
//!          username (username):                  {}\n\
//!          --------------------------------------\n\
//!          host's fancy name (host):             {}\n\
//!          hostname (hostname):                  {}\n\
//!          --------------------------------------\n\
//!          platform (platform):                  {}\n\
//!          operating system (os):                {}\n\
//!          desktop environment (env):            {}\n\
//!          --------------------------------------\n\
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
    html_logo_url = "https://free.plopgrizzly.com/whoami/icon.svg",
    html_favicon_url = "https://free.plopgrizzly.com/whoami/icon.svg",
    html_root_url = "http://free.plopgrizzly.com/whoami/"
)]

/// Which Desktop Environment
#[allow(missing_docs)]
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
    Fuchsia,
    Redox,
    Unknown(String),
}

impl ::std::fmt::Display for DesktopEnv {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use self::DesktopEnv::*;

        write!(
            f,
            "{}",
            match self {
                Gnome => "gnome".to_string(),
                Windows => "windows".to_string(),
                Lxde => "lxde".to_string(),
                Openbox => "openbox".to_string(),
                Mate => "mate".to_string(),
                Xfce => "xfce".to_string(),
                Kde => "kde".to_string(),
                Cinnamon => "cinnamon".to_string(),
                I3 => "i3".to_string(),
                Mac => "mac".to_string(),
                Ios => "ios".to_string(),
                Android => "android".to_string(),
                Wasm => "wasm".to_string(),
                Console => "console".to_string(),
                Ubuntu => "ubuntu".to_string(),
                Fuchsia => "fuchsia".to_string(),
                Redox => "redox".to_string(),
                Unknown(a) => format!("Unknown: \"{}\"", a),
            }
        )
    }
}

/// Which Platform
#[allow(missing_docs)]
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
    Web,
    Dive,
    Fuchsia,
    Redox,
    Unknown(String),
}

impl ::std::fmt::Display for Platform {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use self::Platform::*;

        write!(
            f,
            "{}",
            match self {
                Linux => "Linux".to_string(),
                FreeBsd => "Free BSD".to_string(),
                Windows => "Windows".to_string(),
                MacOS => "Mac OS".to_string(),
                Ios => "iOS".to_string(),
                Android => "Android".to_string(),
                Nintendo => "Nintendo".to_string(),
                Xbox => "XBox".to_string(),
                PlayStation => "PlayStation".to_string(),
                Web => "Web".to_string(),
                Dive => "Dive".to_string(),
                Fuchsia => "Fuchsia".to_string(),
                Redox => "Redox".to_string(),
                Unknown(a) => format!("Unknown: \"{}\"", a),
            }
        )
    }
}

#[cfg(not(target_os = "windows"))]
mod unix;
#[cfg(not(target_os = "windows"))]
use self::unix as native;
#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
use self::windows as native;

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
    native::os()
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
pub const fn platform() -> Platform {
    native::platform()
}
