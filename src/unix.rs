use crate::{DesktopEnv, Platform};

use std::ffi::{c_void, OsString};
use std::mem;
use std::os::unix::ffi::OsStringExt;
use std::process::Command;
use std::process::Stdio;

#[repr(C)]
struct PassWd {
    pw_name: *const c_void,
    pw_passwd: *const c_void,
    pw_uid: u32,
    pw_gid: u32,
    #[cfg(target_os = "macos")]
    pw_change: isize,
    #[cfg(target_os = "macos")]
    pw_class: *const c_void,
    pw_gecos: *const c_void,
    pw_dir: *const c_void,
    pw_shell: *const c_void,
    #[cfg(target_os = "macos")]
    pw_expire: isize,
    #[cfg(target_os = "macos")]
    pw_fields: i32,
}

extern "system" {
    fn getpwuid_r(
        uid: u32,
        pwd: *mut PassWd,
        buf: *mut c_void,
        buflen: usize,
        result: *mut *mut PassWd,
    ) -> i32;
    fn geteuid() -> u32;
    fn strlen(cs: *const c_void) -> usize;
    fn gethostname(name: *mut c_void, len: usize) -> i32;
}

// Convert an OsString into a String
fn string_from_os(string: OsString) -> String {
    match string.into_string() {
        Ok(string) => string,
        Err(string) => string.to_string_lossy().to_string(),
    }
}

fn os_from_cstring(string: *const c_void) -> OsString {
    if string.is_null() {
        return "".to_string().into();
    }

    // Get a byte slice of the c string.
    let slice = unsafe {
        let length = strlen(string);
        std::slice::from_raw_parts(string as *const u8, length)
    };

    // Turn byte slice into Rust String.
    OsString::from_vec(slice.to_vec())
}

// This function must allocate, because a slice or Cow<OsStr> would still
// reference `passwd` which is dropped when this function returns.
#[inline(always)]
fn getpwuid(real: bool) -> Result<OsString, OsString> {
    const BUF_SIZE: usize = 16_384; // size from the man page
    let mut buffer = mem::MaybeUninit::<[u8; BUF_SIZE]>::uninit();
    let mut passwd = mem::MaybeUninit::<PassWd>::uninit();
    let mut _passwd = mem::MaybeUninit::<*mut PassWd>::uninit();

    // Get PassWd `struct`.
    let passwd = unsafe {
        getpwuid_r(
            geteuid(),
            passwd.as_mut_ptr(),
            buffer.as_mut_ptr() as *mut c_void,
            BUF_SIZE,
            _passwd.as_mut_ptr(),
        );

        passwd.assume_init()
    };

    // Extract names.
    if real {
        let string = os_from_cstring(passwd.pw_gecos);
        if string.is_empty() {
            Err(os_from_cstring(passwd.pw_name))
        } else {
            Ok(string)
        }
    } else {
        Ok(os_from_cstring(passwd.pw_name))
    }
}

pub fn username() -> String {
    string_from_os(username_os())
}

pub fn username_os() -> OsString {
    // Unwrap never fails
    getpwuid(false).unwrap()
}

fn fancy_fallback(result: Result<OsString, OsString>) -> OsString {
    match result {
        Ok(success) => success,
        Err(fallback) => {
            let mut cap = true;
            let mut new = String::new();
            let cs = match fallback.to_str() {
                Some(a) => Ok(a),
                None => Err(fallback.to_string_lossy().to_string()),
            };
            let iter = match cs {
                Ok(a) => a.chars(),
                Err(ref b) => b.chars(),
            };

            for c in iter {
                match c {
                    '.' | '-' | '_' => {
                        new.push(' ');
                        cap = true;
                    }
                    a => {
                        if cap {
                            cap = false;
                            for i in a.to_uppercase() {
                                new.push(i);
                            }
                        } else {
                            new.push(a);
                        }
                    }
                }
            }
            new.into()
        }
    }
}

pub fn realname() -> String {
    string_from_os(realname_os())
}

pub fn realname_os() -> OsString {
    // If no real name is provided, guess based on username.
    fancy_fallback(getpwuid(true))
}

pub fn devicename() -> String {
    string_from_os(devicename_os())
}

pub fn devicename_os() -> OsString {
    let mut program = if cfg!(not(target_os = "macos")) {
        Command::new("hostnamectl")
            .arg("--pretty")
            .stdout(Stdio::piped())
            .output()
            .expect("Couldn't Find `hostnamectl`")
    } else {
        Command::new("scutil")
            .arg("--get")
            .arg("ComputerName")
            .output()
            .expect("Couldn't find `scutil`")
    };

    program.stdout.pop();

    let computer = if program.stdout.is_empty() {
        Err(hostname_os())
    } else {
        Ok(OsString::from_vec(program.stdout))
    };
    fancy_fallback(computer)
}

pub fn hostname() -> String {
    string_from_os(hostname_os())
}

pub fn hostname_os() -> OsString {
    // Maximum hostname length = 255, plus a NULL byte.
    let mut string = Vec::<u8>::with_capacity(256);
    unsafe {
        gethostname(string.as_mut_ptr() as *mut c_void, 255);
        string.set_len(strlen(string.as_ptr() as *const c_void));
    };
    OsString::from_vec(string)
}

#[cfg(target_os = "macos")]
pub fn distro() -> Option<String> {
    distro_os().map(|a| string_from_os(a))
}

#[cfg(target_os = "macos")]
pub fn distro_os() -> Option<OsString> {
    let mut distro = Vec::new();

    let name = Command::new("sw_vers")
        .arg("-productName")
        .output()
        .expect("Couldn't find `sw_vers`");

    let version = Command::new("sw_vers")
        .arg("-productVersion")
        .output()
        .expect("Couldn't find `sw_vers`");

    let build = Command::new("sw_vers")
        .arg("-buildVersion")
        .output()
        .expect("Couldn't find `sw_vers`");

    distro.extend(&name.stdout);
    distro.pop();
    distro.push(b' ');
    distro.extend(&version.stdout);
    distro.pop();
    distro.push(b' ');
    distro.extend(&build.stdout);
    distro.pop();

    Some(OsString::from_vec(distro))
}

#[cfg(not(target_os = "macos"))]
pub fn distro_os() -> Option<OsString> {
    distro().map(|a| a.into())
}

#[cfg(not(target_os = "macos"))]
pub fn distro() -> Option<String> {
    let mut distro = String::new();

    let program = std::fs::read_to_string("/etc/os-release")
        .expect("Couldn't read file /etc/os-release")
        .into_bytes();

    distro.push_str(&String::from_utf8_lossy(&program));

    let mut fallback = None;

    for i in distro.split('\n') {
        let mut j = i.split('=');

        match j.next()? {
            "PRETTY_NAME" => {
                return Some(j.next()?.trim_matches('"').to_string())
            }
            "NAME" => fallback = Some(j.next()?.trim_matches('"').to_string()),
            _ => {}
        }
    }

    if let Some(x) = fallback {
        Some(x)
    } else {
        None
    }
}

#[cfg(target_os = "macos")]
#[inline(always)]
pub const fn desktop_env() -> DesktopEnv {
    DesktopEnv::Aqua
}

#[cfg(not(target_os = "macos"))]
#[inline(always)]
pub fn desktop_env() -> DesktopEnv {
    match std::env::var_os("DESKTOP_SESSION")
        .map(|env| env.to_string_lossy().to_string())
    {
        Some(env_orig) => {
            let env = env_orig.to_uppercase();

            if env.contains("GNOME") {
                DesktopEnv::Gnome
            } else if env.contains("LXDE") {
                DesktopEnv::Lxde
            } else if env.contains("OPENBOX") {
                DesktopEnv::Openbox
            } else if env.contains("I3") {
                DesktopEnv::I3
            } else if env.contains("UBUNTU") {
                DesktopEnv::Ubuntu
            } else if env.contains("PLASMA5") {
                DesktopEnv::Kde
            } else {
                DesktopEnv::Unknown(env_orig)
            }
        }
        // TODO: Other Linux Desktop Environments
        None => DesktopEnv::Unknown("Unknown".to_string()),
    }
}

#[cfg(target_os = "macos")]
#[inline(always)]
pub const fn platform() -> Platform {
    Platform::MacOS
}

#[cfg(not(target_os = "macos"))]
#[inline(always)]
pub const fn platform() -> Platform {
    Platform::Linux
}
