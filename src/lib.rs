// Copyright © Jeron Lau 2017 - 2019.
// Dual-licensed under either the MIT License or the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at https://www.boost.org/LICENSE_1_0.txt)

//! Crate for getting the user's username, realname and environment.
//!
//! ## Getting Started
//! Using the whoami crate is super easy!  All of the public items are simple functions with no parameters that return `String`s (with the exception of `env`, which returns an enum).  The following example shows how to use all of the functions:
//! 
//! ```rust
//! use whoami;
//! 
//! fn main() {
//!     print!(
//!         "--------------------------------------\n\
//!          user's full name (user):              {}\n\
//!          username (username):                  {}\n\
//!          --------------------------------------\n\
//!          host's fancy name (host):             {}\n\
//!          hostname (hostname):                  {}\n\
//!          --------------------------------------\n\
//!          operating system (os):                {}\n\
//!          desktop environment (env):            {}\n\
//!          --------------------------------------\n\
//!          ",
//!         whoami::user(),
//!         whoami::username(),
//!         whoami::host(),
//!         whoami::hostname(),
//!         whoami::os(),
//!         whoami::env(),
//!     );
//! }
//! ```

#![warn(missing_docs)]

extern crate libc;

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
                Unknown(a) => format!("Unknown: \"{}\"", a),
            }
        )
    }
}

#[cfg(not(target_os = "windows"))]
mod linux;
#[cfg(not(target_os = "windows"))]
use self::linux as native;
#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
use self::windows as native;

/// Get the user's username.
pub fn username() -> String {
    native::username()
}

/// Get the user's full name.
pub fn user() -> String {
    native::realname()
}

/// Get the host device's (pretty) name.
pub fn host() -> String {
    native::computer()
}

/// Get the host device's hostname.
pub fn hostname() -> String {
    native::hostname()
}

/// Get the the operating system name and version.
///
/// Example: "Windows 10" or "Fedora 26 (Workstation Edition)"
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
