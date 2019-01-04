// Copyright © Jeron Lau 2017 - 2019.
// Dual-licensed under either the MIT License or the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at https://www.boost.org/LICENSE_1_0.txt)

//! Crate for getting the user's username and realname.

extern crate libc;

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

/// Get the user's full name.  Format: `FIRST_NAME [MIDDLE_NAME] [LAST_NAME]`
pub fn user() -> String {
    native::realname()
}

/// Get the computer's pretty name.
pub fn host() -> String {
    native::computer()
}

/// Get the computer's hostname.
pub fn hostname() -> String {
    native::hostname()
}

/// Get the OS.  Example: "Windows 10" or "Fedora 26 (Workstation Edition)"
pub fn os() -> String {
    native::os()
}

/// Get the Desktop Environment.  Example: "gnome" or "windows"
#[inline(always)]
pub fn env() -> DesktopEnv {
    native::env()
}
