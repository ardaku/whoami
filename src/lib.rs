// "whoami" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>
//
//! Crate for getting the user's username and realname.

extern crate libc;

pub enum DesktopEnv {
	Gnome,
	Windows,
	Unknown,
}

impl ::std::fmt::Display for DesktopEnv {
	fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
		use self::DesktopEnv::*;

		write!(f, "{}", match *self {
			Gnome => "gnome",
			Windows => "windows",
			Unknown => "unknown",
		})
	}
}

#[cfg(not(target_os = "windows"))] mod linux;
#[cfg(not(target_os = "windows"))] use self::linux as native;
#[cfg(target_os = "windows")] mod windows;
#[cfg(target_os = "windows")] use self::windows as native;

/// Get the user's username.
pub fn username() -> String {
	native::username()
}

/// Get the user's full name.  Format: `FIRST_NAME [MIDDLE_NAME] [LAST_NAME]`
pub fn realname() -> String {
	native::realname()
}

/// Get the computer's pretty name.
pub fn computer() -> String {
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
