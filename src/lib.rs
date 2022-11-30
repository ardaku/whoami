// Copyright © 2017-2022 The WhoAmI Contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).
//
//! Crate for getting the user's username, realname and environment.
//!
//! ## Getting Started
//! Using the whoami crate is super easy!  All of the public items are simple
//! functions with no parameters that return [`String`](std::string::String)s or
//! [`OsString`](std::ffi::OsString)s (with the exception of
//! [`desktop_env()`](crate::desktop_env), [`platform()`](crate::platform) and
//! [`arch()`](crate:arch), which return enums, and [`lang()`](crate::lang)
//! that returns an iterator of [`String`](std::string::String)s).  
//! The following example shows how to use all of the functions (except
//! those that return [`OsString`](std::ffi::OsString)):
//!
//! ```rust
//! fn main() {
//!     println!(
//!         "User's Name            whoami::realname():    {}",
//!         whoami::realname(),
//!     );
//!     println!(
//!         "User's Username        whoami::username():    {}",
//!         whoami::username(),
//!     );
//!     println!(
//!         "User's Language        whoami::lang():        {:?}",
//!         whoami::lang().collect::<Vec<String>>(),
//!     );
//!     println!(
//!         "Device's Pretty Name   whoami::devicename():  {}",
//!         whoami::devicename(),
//!     );
//!     println!(
//!         "Device's Hostname      whoami::hostname():    {}",
//!         whoami::hostname(),
//!     );
//!     println!(
//!         "Device's Platform      whoami::platform():    {}",
//!         whoami::platform(),
//!     );
//!     println!(
//!         "Device's OS Distro     whoami::distro():      {}",
//!         whoami::distro(),
//!     );
//!     println!(
//!         "Device's Desktop Env.  whoami::desktop_env(): {}",
//!         whoami::desktop_env(),
//!     );
//!     println!(
//!         "Device's CPU Arch      whoami::arch():        {}",
//!         whoami::arch(),
//!     );
//! }
//! ```

#![warn(missing_docs)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/ardaku/whoami/stable/res/icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/ardaku/whoami/stable/res/icon.svg"
)]

use std::{ffi::OsString, io};

/// Which Desktop Environment
#[derive(Debug, PartialEq, Eq, Clone)]
#[non_exhaustive]
pub enum DesktopEnv {
    /// Popular GTK-based desktop environment on Linux
    Gnome,
    /// One of the desktop environments for a specific version of Windows
    Windows,
    /// Linux desktop environment optimized for low resource requirements
    Lxde,
    /// Stacking window manager for X Windows on Linux
    Openbox,
    /// Desktop environment for Linux, BSD and Illumos
    Mate,
    /// Lightweight desktop enivornment for unix-like operating systems
    Xfce,
    /// KDE Plasma desktop enviroment
    // FIXME: Rename to 'Plasma' in whoami 2.0.0
    Kde,
    /// Default desktop environment on Linux Mint
    Cinnamon,
    /// Tiling window manager for Linux
    I3,
    /// Desktop environment for MacOS
    Aqua,
    /// Desktop environment for iOS
    Ios,
    /// Desktop environment for Android
    Android,
    /// Running as Web Assembly on a web page
    WebBrowser,
    /// A desktop environment for a video game console
    Console,
    /// Ubuntu-branded GNOME
    Ubuntu,
    /// Default shell for Fuchsia
    Ermine,
    /// Default desktop environment for Redox
    Orbital,
    /// Unknown desktop environment
    Unknown(String),
}

impl std::fmt::Display for DesktopEnv {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let DesktopEnv::Unknown(_) = self {
            write!(f, "Unknown: ")?;
        }

        write!(
            f,
            "{}",
            match self {
                DesktopEnv::Gnome => "Gnome",
                DesktopEnv::Windows => "Windows",
                DesktopEnv::Lxde => "LXDE",
                DesktopEnv::Openbox => "Openbox",
                DesktopEnv::Mate => "Mate",
                DesktopEnv::Xfce => "XFCE",
                DesktopEnv::Kde => "KDE",
                DesktopEnv::Cinnamon => "Cinnamon",
                DesktopEnv::I3 => "I3",
                DesktopEnv::Aqua => "Aqua",
                DesktopEnv::Ios => "IOS",
                DesktopEnv::Android => "Android",
                DesktopEnv::WebBrowser => "Web Browser",
                DesktopEnv::Console => "Console",
                DesktopEnv::Ubuntu => "Ubuntu",
                DesktopEnv::Ermine => "Ermine",
                DesktopEnv::Orbital => "Orbital",
                DesktopEnv::Unknown(a) => a,
            }
        )
    }
}

/// Which Platform
#[allow(missing_docs)]
#[derive(Debug, PartialEq, Eq, Clone)]
#[non_exhaustive]
pub enum Platform {
    Linux,
    Bsd,
    Windows,
    // FIXME: Non-standard casing; Rename to 'Mac' rather than 'MacOs' in
    // whoami 2.0.0
    MacOS,
    Ios,
    Android,
    Nintendo,
    Xbox,
    PlayStation,
    Fuchsia,
    Redox,
    Unknown(String),
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Platform::Unknown(_) = self {
            write!(f, "Unknown: ")?;
        }

        write!(
            f,
            "{}",
            match self {
                Platform::Linux => "Linux",
                Platform::Bsd => "BSD",
                Platform::Windows => "Windows",
                Platform::MacOS => "Mac OS",
                Platform::Ios => "iOS",
                Platform::Android => "Android",
                Platform::Nintendo => "Nintendo",
                Platform::Xbox => "XBox",
                Platform::PlayStation => "PlayStation",
                Platform::Fuchsia => "Fuchsia",
                Platform::Redox => "Redox",
                Platform::Unknown(a) => a,
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
///
/// On unix-systems this differs from `realname()` most notably in that spaces
/// are not allowed.
#[inline(always)]
pub fn username() -> String {
    native::username()
}

/// Get the user's username.
///
/// On unix-systems this differs from `realname()` most notably in that spaces
/// are not allowed.
#[inline(always)]
pub fn username_os() -> OsString {
    native::username_os()
}

/// Get the user's real (full) name.
#[inline(always)]
pub fn realname() -> String {
    native::realname()
}

/// Get the user's real (full) name.
#[inline(always)]
pub fn realname_os() -> OsString {
    native::realname_os()
}

/// Get the device name (also known as "Pretty Name").
///
/// Often used to identify device for bluetooth pairing.
#[inline(always)]
pub fn devicename() -> String {
    native::devicename()
}

/// Get the device name (also known as "Pretty Name").
///
/// Often used to identify device for bluetooth pairing.
#[inline(always)]
pub fn devicename_os() -> OsString {
    native::devicename_os()
}

/// Get the host device's hostname.
///
/// Limited to a-z (case insensitve), 0-9, and dashes.  This limit also applies
/// to `devicename()` when targeting Windows.  Since the hostname is
/// case-insensitive, this method normalizes to lowercase (unlike
/// `devicename()`).
#[inline(always)]
pub fn hostname() -> String {
    let mut hostname = native::hostname();
    hostname.make_ascii_lowercase();
    hostname
}

/// Get the host device's hostname.
///
/// Limited to a-z (case insensitve), 0-9, and dashes.  This limit also applies
/// to `devicename()` when targeting Windows.  Since the hostname is
/// case-insensitive, this method normalizes to lowercase (unlike
/// `devicename()`).
#[inline(always)]
pub fn hostname_os() -> OsString {
    hostname().into()
}

/// Get the name of the operating system distribution and (possibly) version.
///
/// Example: "Windows 10" or "Fedora 26 (Workstation Edition)"
#[inline(always)]
pub fn distro() -> String {
    native::distro().unwrap_or_else(|| format!("Unknown {}", platform()))
}

/// Get the name of the operating system distribution and (possibly) version.
///
/// Example: "Windows 10" or "Fedora 26 (Workstation Edition)"
#[inline(always)]
pub fn distro_os() -> OsString {
    native::distro_os()
        .unwrap_or_else(|| format!("Unknown {}", platform()).into())
}

/// Get the desktop environment.
///
/// Example: "gnome" or "windows"
#[inline(always)]
pub fn desktop_env() -> DesktopEnv {
    native::desktop_env()
}

/// Get the platform.
#[inline(always)]
pub fn platform() -> Platform {
    native::platform()
}

/// Get the user's preferred language(s).
///
/// Returned as iterator of two letter language codes (lowercase), optionally
/// followed by a dash (-) and a two letter region code (uppercase).  The most
/// preferred language is returned first, followed by next preferred, and so on.
#[inline(always)]
pub fn lang() -> impl Iterator<Item = String> {
    native::lang()
}

/// Which CPU Architecture
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Arch {
    /// ARM64
    Aarch64,
    /// ARM
    Arm,
    /// ARM BE8
    ArmEb,
    /// ARMv4T
    ArmV4T,
    /// ARMv5TE
    ArmV5Te,
    /// ARMv6
    ArmV6,
    /// ARMv7
    ArmV7,
    /// Qualcomm Hexagon
    Hexagon,
    /// i386
    I386,
    /// i586
    I586,
    /// i686
    I686,
    /// Motorola 68000 series
    M68k,
    /// MIPS
    Mips,
    /// MIPS (LE)
    MipsEl,
    /// MIPS64
    Mips64,
    /// MIPS64 (LE)
    Mips64El,
    /// PowerPC
    PowerPc,
    /// PowerPC64
    PowerPc64,
    /// PowerPC64LE
    PowerPc64Le,
    /// 32-bit RISC-V
    Riscv32Gc,
    /// 64-bit RISC-V
    Riscv64Gc,
    /// S390x
    S390x,
    /// SPARC
    Sparc,
    /// SPARC64
    Sparc64,
    /// Thumbv7neon
    ThumbV7Neon,
    /// 32-bit Web Assembly
    Wasm32,
    /// 64-bit Web Assembly
    Wasm64,
    /// X86_64/Amd64
    X64,
    /// Unknown Architecture
    Unknown(String),
}

impl std::fmt::Display for Arch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let arch_str = match self {
            Arch::Aarch64 => "aarch64",
            Arch::Arm => "arm",
            Arch::ArmEb => "armeb",
            Arch::ArmV4T => "armv4t",
            Arch::ArmV5Te => "armv5te",
            Arch::ArmV6 => "armv6",
            Arch::ArmV7 => "armv7",
            Arch::Hexagon => "hexagon",
            Arch::I386 => "i386",
            Arch::I586 => "i586",
            Arch::I686 => "i686",
            Arch::M68k => "m68k",
            Arch::Mips => "mips",
            Arch::MipsEl => "mipsel",
            Arch::Mips64 => "mips64",
            Arch::Mips64El => "mips64el",
            Arch::PowerPc => "powerpc",
            Arch::PowerPc64 => "powerpc64",
            Arch::PowerPc64Le => "powerpc64le",
            Arch::Riscv32Gc => "riscv32gc",
            Arch::Riscv64Gc => "riscv64gc",
            Arch::S390x => "s390x",
            Arch::Sparc => "sparc",
            Arch::Sparc64 => "sparc64",
            Arch::ThumbV7Neon => "thumbv7neon",
            Arch::Wasm32 => "wasm32",
            Arch::Wasm64 => "wasm64",
            Arch::X64 => "x86_64",
            Arch::Unknown(arch_str) => arch_str,
        };

        if let Arch::Unknown(_) = self {
            write!(f, "Unknown: ")?;
        }

        write!(f, "{}", arch_str)
    }
}

/// Get the CPU Architecture.
#[inline(always)]
pub fn arch() -> Arch {
    native::arch()
}

/// Which Width.
#[derive(Debug)]
pub enum Width {
    /// 32 bits
    Bits32,
    /// 64 bits
    Bits64,
}

impl std::fmt::Display for Width {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bits = match self {
            Width::Bits32 => "32 bits",
            Width::Bits64 => "64 bits",
        };
        write!(f, "{}", bits)
    }
}

impl Arch {
    /// Get the width of this architecture.
    pub fn width(&self) -> io::Result<Width> {
        match self {
            Arch::Arm
            | Arch::ArmEb
            | Arch::ArmV4T
            | Arch::ArmV5Te
            | Arch::ArmV6
            | Arch::ArmV7
            | Arch::Hexagon
            | Arch::I386
            | Arch::I586
            | Arch::I686
            | Arch::M68k
            | Arch::Mips
            | Arch::MipsEl
            | Arch::PowerPc
            | Arch::Riscv32Gc
            | Arch::Sparc
            | Arch::ThumbV7Neon
            | Arch::Wasm32 => Ok(Width::Bits32),
            Arch::Aarch64
            | Arch::Mips64
            | Arch::Mips64El
            | Arch::PowerPc64
            | Arch::PowerPc64Le
            | Arch::Riscv64Gc
            | Arch::S390x
            | Arch::Sparc64
            | Arch::Wasm64
            | Arch::X64 => Ok(Width::Bits64),
            Arch::Unknown(unknown_arch) => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Tried getting width of unknown arch ({})",
                    unknown_arch
                ),
            )),
        }
    }
}
