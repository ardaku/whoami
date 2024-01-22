//! Currently used for WebAssembly unknown (non-web) only

#[cfg(not(any(target_pointer_width = "32", target_pointer_width = "64")))]
compile_error!("Unexpected pointer width for target platform");

use std::ffi::OsString;

use crate::{Arch, DesktopEnv, Platform, Result};

#[inline(always)]
pub(crate) fn lang() -> impl Iterator<Item = String> {
    std::iter::once("en-US".to_string())
}

#[inline(always)]
pub(crate) fn username_os() -> Result<OsString> {
    Ok(username()?.into())
}

#[inline(always)]
pub(crate) fn realname_os() -> Result<OsString> {
    Ok(realname()?.into())
}

#[inline(always)]
pub(crate) fn devicename_os() -> Result<OsString> {
    Ok(devicename()?.into())
}

#[inline(always)]
pub(crate) fn distro_os() -> Result<OsString> {
    Ok(distro()?.into())
}

#[inline(always)]
pub(crate) fn username() -> Result<String> {
    Ok("anonymous".to_string())
}

#[inline(always)]
pub(crate) fn realname() -> Result<String> {
    Ok("Anonymous".to_string())
}

#[inline(always)]
pub(crate) fn devicename() -> Result<String> {
    Ok("Unknown".to_string())
}

#[inline(always)]
pub(crate) fn hostname() -> Result<String> {
    Ok("localhost".to_string())
}

#[inline(always)]
pub(crate) fn distro() -> Result<String> {
    Ok("Emulated".to_string())
}

#[inline(always)]
pub(crate) fn desktop_env() -> DesktopEnv {
    DesktopEnv::Unknown("WebAssembly".to_string())
}

pub(crate) fn platform() -> Platform {
    Platform::Unknown("Unknown".to_string())
}

pub(crate) fn arch() -> Arch {
    if cfg!(target_pointer_width = "64") {
        Arch::Wasm64
    } else {
        Arch::Wasm32
    }
}
