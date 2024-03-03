//! Currently used for WebAssembly unknown (non-web) only

#[cfg(not(any(target_pointer_width = "32", target_pointer_width = "64")))]
compile_error!("Unexpected pointer width for target platform");

use std::ffi::OsString;

use crate::{
    os::{Os, Target},
    Arch, DesktopEnv, Platform, Result,
};

impl Target for Os {
    #[inline(always)]
    fn langs(self) -> Result<String> {
        Ok("en/US".to_string())
    }

    #[inline(always)]
    fn realname(self) -> Result<OsString> {
        Ok("Anonymous".to_string().into())
    }

    #[inline(always)]
    fn username(self) -> Result<OsString> {
        Ok("anonymous".to_string().into())
    }

    #[inline(always)]
    fn devicename(self) -> Result<OsString> {
        Ok("Unknown".to_string().into())
    }

    #[inline(always)]
    fn hostname(self) -> Result<String> {
        Ok("localhost".to_string())
    }

    #[inline(always)]
    fn distro(self) -> Result<String> {
        Ok("Emulated".to_string())
    }

    #[inline(always)]
    fn desktop_env(self) -> DesktopEnv {
        DesktopEnv::Unknown("WebAssembly".to_string())
    }

    #[inline(always)]
    fn platform(self) -> Platform {
        Platform::Unknown("Unknown".to_string())
    }

    #[inline(always)]
    fn arch(self) -> Result<Arch> {
        Ok(if cfg!(target_pointer_width = "64") {
            Arch::Wasm64
        } else {
            Arch::Wasm32
        })
    }
}
