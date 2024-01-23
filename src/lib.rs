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
//! println!(
//!     "User's Name            whoami::realname():    {}",
//!     whoami::realname(),
//! );
//! println!(
//!     "User's Username        whoami::username():    {}",
//!     whoami::username(),
//! );
//! println!(
//!     "User's Language        whoami::lang():        {:?}",
//!     whoami::lang().collect::<Vec<String>>(),
//! );
//! println!(
//!     "Device's Pretty Name   whoami::devicename():  {}",
//!     whoami::devicename(),
//! );
//! println!(
//!     "Device's Hostname      whoami::hostname():    {}",
//!     whoami::hostname(),
//! );
//! println!(
//!     "Device's Platform      whoami::platform():    {}",
//!     whoami::platform(),
//! );
//! println!(
//!     "Device's OS Distro     whoami::distro():      {}",
//!     whoami::distro(),
//! );
//! println!(
//!     "Device's Desktop Env.  whoami::desktop_env(): {}",
//!     whoami::desktop_env(),
//! );
//! println!(
//!     "Device's CPU Arch      whoami::arch():        {}",
//!     whoami::arch(),
//! );
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

const DEFAULT_USERNAME: &str = "Unknown";
const DEFAULT_HOSTNAME: &str = "LocalHost";

#[allow(missing_docs)]
pub mod disp;
pub mod fallible;

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

use is_terminal::IsTerminal;

/// This crate's convenience type alias for [`Result`](std::result::Result)s
pub type Result<T = (), E = Error> = std::result::Result<T, E>;

/// Region code for a [`Language`] dialect
///
/// Uses <https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2>
#[non_exhaustive]
#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Region {
    // FIXME: V2: u32::from_ne_bytes for region codes, with `\0` for unused
    // FIXME: Add aliases up to 3-4 letters, but hidden
    /// Any dialect
    Any,
    /// `US`: United States of America
    #[doc(hidden)]
    Us,
}

impl Display for Region {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Any => "**",
            Self::Us => "US",
        })
    }
}

/// A spoken language
///
/// Use [`ToString::to_string()`] to convert to string of two letter lowercase
/// language code followed and underscore and uppercase region code (example:
/// `en_US`).
///
/// Uses <https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes>
#[non_exhaustive]
#[derive(Clone, Eq, PartialEq, Debug)]
// #[allow(variant_size_differences)]
pub enum Language {
    #[doc(hidden)]
    __(Box<String>),
    /// `en`: English
    #[doc(hidden)]
    En(Region),
    /// `es`: Spanish
    #[doc(hidden)]
    Es(Region),
}

impl Language {
    /// Retrieve the region code for this language dialect.
    pub fn region(&self) -> Region {
        match self {
            Self::__(_) => Region::Any,
            Self::En(region) | Self::Es(region) => *region,
        }
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::__(code) => f.write_str(code.as_str()),
            Self::En(region) => {
                if *region != Region::Any {
                    f.write_str("en_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("en")
                }
            }
            Self::Es(region) => {
                if *region != Region::Any {
                    f.write_str("es_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("es")
                }
            }
        }
    }
}

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
        if let Self::Unknown(_) = self {
            f.write_str("Unknown: ")?;
        }

        f.write_str(match self {
            Self::Gnome => "Gnome",
            Self::Windows => "Windows",
            Self::Lxde => "LXDE",
            Self::Openbox => "Openbox",
            Self::Mate => "Mate",
            Self::Xfce => "XFCE",
            Self::Kde => "KDE",
            Self::Cinnamon => "Cinnamon",
            Self::I3 => "I3",
            Self::Aqua => "Aqua",
            Self::Ios => "IOS",
            Self::Android => "Android",
            Self::WebBrowser => "Web Browser",
            Self::Console => "Console",
            Self::Ubuntu => "Ubuntu",
            Self::Ermine => "Ermine",
            Self::Orbital => "Orbital",
            Self::Unknown(a) => a,
        })
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
        if std::io::stdout().is_terminal() {
            f.write_str(match self {
                Self::Linux => disp::display_linux(),
                Self::Bsd => disp::display_bsd(),
                Self::Windows => disp::display_windows(),
                Self::MacOS => disp::display_macos(),
                Self::Illumos => disp::display_illumos(),
                Self::Ios => disp::display_ios(),
                Self::Android => disp::display_android(),
                Self::Nintendo => disp::display_nintendo(),
                Self::Xbox => disp::display_xbox(),
                Self::PlayStation => disp::display_playstation(),
                Self::Fuchsia => disp::display_fuchsia(),
                Self::Redox => disp::display_redox(),
                Self::Unknown(a) => a,
            })
        } else {
            write!(f, "{:?}", self)
        }
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
        if let Self::Unknown(_) = self {
            f.write_str("Unknown: ")?;
        }

        f.write_str(match self {
            Self::ArmV5 => "armv5",
            Self::ArmV6 => "armv6",
            Self::ArmV7 => "armv7",
            Self::Arm64 => "arm64",
            Self::I386 => "i386",
            Self::I586 => "i586",
            Self::I686 => "i686",
            Self::Mips => "mips",
            Self::MipsEl => "mipsel",
            Self::Mips64 => "mips64",
            Self::Mips64El => "mips64el",
            Self::PowerPc => "powerpc",
            Self::PowerPc64 => "powerpc64",
            Self::PowerPc64Le => "powerpc64le",
            Self::Riscv32 => "riscv32",
            Self::Riscv64 => "riscv64",
            Self::S390x => "s390x",
            Self::Sparc => "sparc",
            Self::Sparc64 => "sparc64",
            Self::Wasm32 => "wasm32",
            Self::Wasm64 => "wasm64",
            Self::X64 => "x86_64",
            Self::Unknown(arch) => arch,
        })
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
        f.write_str(match self {
            Width::Bits32 => "32 bits",
            Width::Bits64 => "64 bits",
        })
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
                    unknown_arch,
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
/// are not allowed in the username.
#[inline(always)]
pub fn username() -> String {
    fallible::username().unwrap_or_else(|_| DEFAULT_USERNAME.to_lowercase())
}

/// Get the user's username.
///
/// On unix-systems this differs from [`realname_os()`] most notably in that
/// spaces are not allowed in the username.
#[inline(always)]
pub fn username_os() -> OsString {
    fallible::username_os()
        .unwrap_or_else(|_| DEFAULT_USERNAME.to_lowercase().into())
}

/// Get the user's real (full) name.
#[inline(always)]
pub fn realname() -> String {
    fallible::realname()
        .or_else(|_| fallible::username())
        .unwrap_or_else(|_| DEFAULT_USERNAME.to_owned())
}

/// Get the user's real (full) name.
#[inline(always)]
pub fn realname_os() -> OsString {
    fallible::realname_os()
        .or_else(|_| fallible::username_os())
        .unwrap_or_else(|_| DEFAULT_USERNAME.to_owned().into())
}

/// Get the device name (also known as "Pretty Name").
///
/// Often used to identify device for bluetooth pairing.
#[inline(always)]
pub fn devicename() -> String {
    fallible::devicename()
        .or_else(|_| fallible::hostname())
        .unwrap_or_else(|_| DEFAULT_HOSTNAME.to_string())
}

/// Get the device name (also known as "Pretty Name").
///
/// Often used to identify device for bluetooth pairing.
#[inline(always)]
pub fn devicename_os() -> OsString {
    fallible::devicename_os()
        .or_else(|_| fallible::hostname_os())
        .unwrap_or_else(|_| DEFAULT_HOSTNAME.to_string().into())
}

/// Get the host device's hostname.
///
/// Limited to a-z (case insensitve), 0-9, and dashes.  This limit also applies
/// to `devicename()` when targeting Windows.  Since the hostname is
/// case-insensitive, this method normalizes to lowercase (unlike
/// [`devicename()`]).
#[inline(always)]
pub fn hostname() -> String {
    fallible::hostname().unwrap_or_else(|_| DEFAULT_HOSTNAME.to_lowercase())
}

/// Get the host device's hostname.
///
/// Limited to a-z (case insensitve), 0-9, and dashes.  This limit also applies
/// to `devicename()` when targeting Windows.  Since the hostname is
/// case-insensitive, this method normalizes to lowercase (unlike
/// [`devicename()`]).
#[inline(always)]
pub fn hostname_os() -> OsString {
    fallible::hostname_os()
        .unwrap_or_else(|_| DEFAULT_HOSTNAME.to_lowercase().into())
}

/// Get the name of the operating system distribution and (possibly) version.
///
/// Example: "Windows 10" or "Fedora 26 (Workstation Edition)"
#[inline(always)]
pub fn distro() -> String {
    fallible::distro().unwrap_or_else(|_| format!("Unknown {}", platform()))
}

/// Get the name of the operating system distribution and (possibly) version.
///
/// Example: "Windows 10" or "Fedora 26 (Workstation Edition)"
#[inline(always)]
pub fn distro_os() -> OsString {
    fallible::distro_os()
        .unwrap_or_else(|_| format!("Unknown {}", platform()).into())
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
#[deprecated(note = "use `langs()` instead", since = "1.5.0")]
pub fn lang() -> impl Iterator<Item = String> {
    platform::lang()
}

/// Get the user's preferred language(s).
///
/// Returned as iterator of [`Language`]s wrapped in [`Result`]s.  The most
/// preferred language is returned first, followed by next preferred, and so on.
/// Unrecognized languages may return an error.
#[inline(always)]
pub fn langs() -> impl Iterator<Item = Result<Language>> {
    #[allow(deprecated)]
    lang().map(|string| Ok(Language::__(Box::new(string))))
}
