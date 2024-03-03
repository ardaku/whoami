//! This is mostly the same as fake.rs for now

#[cfg(not(any(target_pointer_width = "32", target_pointer_width = "64")))]
compile_error!("Unexpected pointer width for target platform");

use std::ffi::OsString;

use crate::{
    os::{Os, Target},
    Arch, DesktopEnv, Language, Platform, Result,
};

#[inline(always)]
pub(crate) fn lang() -> impl Iterator<Item = String> {
    std::iter::once("en/US".to_string())
}

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
        DesktopEnv::Unknown("Unknown Daku".to_string())
    }

    #[inline(always)]
    fn platform(self) -> Platform {
        Platform::Unknown("Daku".to_string())
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
