// Copyright © 2017-2022 The WhoAmI Contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

//! Currently used for WebAssembly unknown (non-web) only

#[cfg(not(any(target_pointer_width = "32", target_pointer_width = "64")))]
compile_error!("Unexpected pointer width for target platform");

use std::ffi::OsString;

use crate::{Arch, DesktopEnv, Platform};

#[inline(always)]
pub fn lang() -> impl Iterator<Item = String> {
    std::iter::once("en-US".to_string())
}

#[inline(always)]
pub fn username_os() -> OsString {
    username().into()
}

#[inline(always)]
pub fn realname_os() -> OsString {
    realname().into()
}

#[inline(always)]
pub fn devicename_os() -> OsString {
    devicename().into()
}

#[inline(always)]
pub fn distro_os() -> Option<OsString> {
    distro().map(|a| a.into())
}

#[inline(always)]
pub fn username() -> String {
    "anonymous".to_string()
}

#[inline(always)]
pub fn realname() -> String {
    "Anonymous".to_string()
}

#[inline(always)]
pub fn devicename() -> String {
    "Unknown".to_string()
}

#[inline(always)]
pub fn hostname() -> String {
    "localhost".to_string()
}

#[inline(always)]
pub fn distro() -> Option<String> {
    None
}

#[inline(always)]
pub fn desktop_env() -> DesktopEnv {
    DesktopEnv::Unknown("WebAssembly".to_string())
}

pub fn platform() -> Platform {
    Platform::Unknown("Unknown".to_string())
}

pub fn arch() -> Arch {
    if cfg!(target_pointer_width = "64") {
        Arch::Wasm64
    } else {
        Arch::Wasm32
    }
}
