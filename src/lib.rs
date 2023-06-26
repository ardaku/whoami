//! Crate for getting the user's username, realname and environment.
//!
//! ## Getting Started
//! Using the whoami crate is super easy!  All of the public items are simple
//! functions with no parameters that return [`String`]s or [`OsString`]s (with
//! the exception of [`desktop_env()`], [`platform()`], and [`arch()`], which
//! return enums, and [`lang()`] that returns an iterator of [`String`]s).  The
//! following example shows how to use all of the functions (except those that
//! return [`OsString`]):
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

#![warn(
    anonymous_parameters,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_extern_crates,
    unused_qualifications,
    variant_size_differences,
    unsafe_code
)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/ardaku/whoami/stable/res/icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/ardaku/whoami/stable/res/icon.svg"
)]

#[allow(unsafe_code)]
// Unix
#[cfg_attr(
    not(any(target_os = "windows", target_arch = "wasm32")),
    path = "unix.rs"
)]
// Wasm32 (Daku) - FIXME: Currently routes to fake.rs
#[cfg_attr(all(target_arch = "wasm32", target_os = "daku"), path = "fake.rs")]
// Wasm32 (Wasi) - FIXME: Currently routes to fake.rs
#[cfg_attr(all(target_arch = "wasm32", target_os = "wasi"), path = "fake.rs")]
// Wasm32 (Web)
#[cfg_attr(
    all(
        target_arch = "wasm32",
        not(target_os = "wasi"),
        not(target_os = "daku"),
        feature = "web",
    ),
    path = "web.rs"
)]
// Wasm32 (Unknown)
#[cfg_attr(
    all(
        target_arch = "wasm32",
        not(target_os = "wasi"),
        not(target_os = "daku"),
        not(feature = "web"),
    ),
    path = "fake.rs"
)]
// Windows
#[cfg_attr(
    all(target_os = "windows", not(target_arch = "wasm32")),
    path = "windows.rs"
)]
mod platform;

use std::{
    ffi::OsString,
    fmt::{self, Display, Formatter},
    io::{Error, ErrorKind},
};

/// This crate's convenience type alias for [`Result`](std::result::Result)s
pub type Result<T = (), E = Error> = std::result::Result<T, E>;

// FIXME: V2: Move `Unknown` variants to the top of the enum.

/// The desktop environment of a system
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

impl Display for DesktopEnv {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let desktop_env = match self {
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
            DesktopEnv::Unknown(a) => {
                f.write_str("Unknown: ")?;
                a
            }
        };

        f.write_str(desktop_env)
    }
}

/// The underlying platform for a system
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
    Illumos,
    Ios,
    Android,
    Nintendo,
    Xbox,
    PlayStation,
    Fuchsia,
    Redox,
    Unknown(String),
}

impl Display for Platform {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let platform = match self {
            Platform::Linux => "Linux",
            Platform::Bsd => "BSD",
            Platform::Windows => "Windows",
            Platform::MacOS => "Mac OS",
            Platform::Illumos => "Illumos",
            Platform::Ios => "iOS",
            Platform::Android => "Android",
            Platform::Nintendo => "Nintendo",
            Platform::Xbox => "XBox",
            Platform::PlayStation => "PlayStation",
            Platform::Fuchsia => "Fuchsia",
            Platform::Redox => "Redox",
            Platform::Unknown(a) => {
                f.write_str("Unknown: ")?;
                a
            }
        };

        f.write_str(platform)
    }
}

/// The architecture of a CPU
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Arch {
    /// ARMv5
    ArmV5,
    /// ARMv6 (Sometimes just referred to as ARM)
    ArmV6,
    /// ARMv7 (May or may not support Neon/Thumb)
    ArmV7,
    /// ARM64 (aarch64)
    Arm64,
    /// i386 (x86)
    I386,
    /// i586 (x86)
    I586,
    /// i686 (x86)
    I686,
    /// X86_64 / Amd64
    X64,
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
    Riscv32,
    /// 64-bit RISC-V
    Riscv64,
    /// S390x
    S390x,
    /// SPARC
    Sparc,
    /// SPARC64
    Sparc64,
    /// 32-bit Web Assembly
    Wasm32,
    /// 64-bit Web Assembly
    Wasm64,
    /// Unknown Architecture
    Unknown(String),
}

impl Display for Arch {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let arch = match self {
            Arch::ArmV5 => "armv5",
            Arch::ArmV6 => "armv6",
            Arch::ArmV7 => "armv7",
            Arch::Arm64 => "arm64",
            Arch::I386 => "i386",
            Arch::I586 => "i586",
            Arch::I686 => "i686",
            Arch::Mips => "mips",
            Arch::MipsEl => "mipsel",
            Arch::Mips64 => "mips64",
            Arch::Mips64El => "mips64el",
            Arch::PowerPc => "powerpc",
            Arch::PowerPc64 => "powerpc64",
            Arch::PowerPc64Le => "powerpc64le",
            Arch::Riscv32 => "riscv32",
            Arch::Riscv64 => "riscv64",
            Arch::S390x => "s390x",
            Arch::Sparc => "sparc",
            Arch::Sparc64 => "sparc64",
            Arch::Wasm32 => "wasm32",
            Arch::Wasm64 => "wasm64",
            Arch::X64 => "x86_64",
            Arch::Unknown(arch) => {
                f.write_str("Unknown: ")?;
                arch
            }
        };

        f.write_str(arch)
    }
}

/// The address width of a CPU architecture
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[non_exhaustive]
pub enum Width {
    /// 32 bits
    Bits32,
    /// 64 bits
    Bits64,
}

impl Display for Width {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let bits = match self {
            Width::Bits32 => "32 bits",
            Width::Bits64 => "64 bits",
        };

        f.write_str(bits)
    }
}

impl Arch {
    /// Get the width of this architecture.
    pub fn width(&self) -> Result<Width> {
        match self {
            Arch::ArmV5
            | Arch::ArmV6
            | Arch::ArmV7
            | Arch::I386
            | Arch::I586
            | Arch::I686
            | Arch::Mips
            | Arch::MipsEl
            | Arch::PowerPc
            | Arch::Riscv32
            | Arch::Sparc
            | Arch::Wasm32 => Ok(Width::Bits32),
            Arch::Arm64
            | Arch::Mips64
            | Arch::Mips64El
            | Arch::PowerPc64
            | Arch::PowerPc64Le
            | Arch::Riscv64
            | Arch::S390x
            | Arch::Sparc64
            | Arch::Wasm64
            | Arch::X64 => Ok(Width::Bits64),
            Arch::Unknown(unknown_arch) => Err(Error::new(
                ErrorKind::InvalidData,
                format!(
                    "Tried getting width of unknown arch ({})",
                    unknown_arch
                ),
            )),
        }
    }
}

/// Get the CPU Architecture.
#[inline(always)]
pub fn arch() -> Arch {
    platform::arch()
}

/// Get the user's username.
///
/// On unix-systems this differs from [`realname()`] most notably in that spaces
/// are not allowed.
#[inline(always)]
pub fn username() -> String {
    platform::username()
}

/// Get the user's username.
///
/// On unix-systems this differs from [`realname()`] most notably in that spaces
/// are not allowed.
#[inline(always)]
pub fn username_os() -> OsString {
    platform::username_os()
}

/// Get the user's real (full) name.
#[inline(always)]
pub fn realname() -> String {
    platform::realname()
}

/// Get the user's real (full) name.
#[inline(always)]
pub fn realname_os() -> OsString {
    platform::realname_os()
}

/// Get the device name (also known as "Pretty Name").
///
/// Often used to identify device for bluetooth pairing.
#[inline(always)]
pub fn devicename() -> String {
    platform::devicename()
}

/// Get the device name (also known as "Pretty Name").
///
/// Often used to identify device for bluetooth pairing.
#[inline(always)]
pub fn devicename_os() -> OsString {
    platform::devicename_os()
}

/// Get the host device's hostname.
///
/// Limited to a-z (case insensitve), 0-9, and dashes.  This limit also applies
/// to `devicename()` when targeting Windows.  Since the hostname is
/// case-insensitive, this method normalizes to lowercase (unlike
/// [`devicename()`]).
#[inline(always)]
pub fn hostname() -> String {
    let mut hostname = platform::hostname();
    hostname.make_ascii_lowercase();
    hostname
}

/// Get the host device's hostname.
///
/// Limited to a-z (case insensitve), 0-9, and dashes.  This limit also applies
/// to `devicename()` when targeting Windows.  Since the hostname is
/// case-insensitive, this method normalizes to lowercase (unlike
/// [`devicename()`]).
#[inline(always)]
pub fn hostname_os() -> OsString {
    hostname().into()
}

/// Get the name of the operating system distribution and (possibly) version.
///
/// Example: "Windows 10" or "Fedora 26 (Workstation Edition)"
#[inline(always)]
pub fn distro() -> String {
    platform::distro().unwrap_or_else(|| format!("Unknown {}", platform()))
}

/// Get the name of the operating system distribution and (possibly) version.
///
/// Example: "Windows 10" or "Fedora 26 (Workstation Edition)"
#[inline(always)]
pub fn distro_os() -> OsString {
    platform::distro_os()
        .unwrap_or_else(|| format!("Unknown {}", platform()).into())
}

/// Get the desktop environment.
///
/// Example: "gnome" or "windows"
#[inline(always)]
pub fn desktop_env() -> DesktopEnv {
    platform::desktop_env()
}

/// Get the platform.
#[inline(always)]
pub fn platform() -> Platform {
    platform::platform()
}

/// Get the user's preferred language(s).
///
/// Returned as iterator of two letter language codes (lowercase), optionally
/// followed by a dash (-) and a two letter region code (uppercase).  The most
/// preferred language is returned first, followed by next preferred, and so on.
#[inline(always)]
pub fn lang() -> impl Iterator<Item = String> {
    platform::lang()
}
