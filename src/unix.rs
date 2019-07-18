use crate::{DesktopEnv, Platform};

use std::ffi::c_void;
use std::mem;
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

fn string_from_cstring(string: *const c_void) -> String {
    if string.is_null() {
	return "".to_string();
	}

    // Get a byte slice of the c string.
    let slice = unsafe {
        let length = strlen(string);
        std::slice::from_raw_parts(string as *const u8, length)
    };

    // Turn byte slice into Rust String.
    String::from_utf8_lossy(slice).to_string()
}

// This function must return `String`s, because a slice or Cow<str> would still
// reference `passwd` which is dropped when this function returns.
#[inline(always)]
fn getpwuid() -> (String, String) {
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
    let a = string_from_cstring(passwd.pw_name);
    let b = string_from_cstring(passwd.pw_gecos);

    (a, b)
}

pub fn username() -> String {
    let pwent = getpwuid();

    pwent.0
}

fn fancy_fallback(mut computer: String, fallback_fn: fn() -> String) -> String {
    let mut cap = true;

    if computer.is_empty() {
        let fallback = fallback_fn();

        for c in fallback.chars() {
            match c {
                '.' | '-' | '_' => {
                    computer.push(' ');
                    cap = true;
                }
                a => {
                    if cap {
                        cap = false;
                        for i in a.to_uppercase() {
                            computer.push(i);
                        }
                    } else {
                        computer.push(a);
                    }
                }
            }
        }
    }

    computer
}

pub fn realname() -> String {
    let pwent = getpwuid();
    let realname = pwent.1;

    // If no real name is provided, guess based on username.
    fancy_fallback(realname, username)
}

pub fn computer() -> String {
    let mut computer = String::new();

    let program = if cfg!(not(target_os = "macos")) {
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

    computer.push_str(String::from_utf8(program.stdout).unwrap().as_str());

    //    let mut pretty = BufReader::new(program.stdout.as_mut().unwrap());

    //    pretty.read_to_string(&mut computer).unwrap();

    computer.pop();

    fancy_fallback(computer, hostname)
}

pub fn hostname() -> String {
    // Maximum hostname length = 255, plus a NULL byte.
    let mut string = mem::MaybeUninit::<[u8; 256]>::uninit();
    let string = unsafe {
        gethostname(string.as_mut_ptr() as *mut c_void, 255);
        &string.assume_init()[..strlen(string.as_ptr() as *const c_void)]
    };

    String::from_utf8_lossy(string).to_string()
}

#[cfg(target_os = "macos")]
pub fn os() -> String {
    let mut distro = String::new();

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

    distro.push_str(String::from_utf8(name.stdout).unwrap().as_str());
    distro.pop();
    distro.push(' ');
    distro.push_str(String::from_utf8(version.stdout).unwrap().as_str());
    distro.pop();
    distro.push(' ');
    distro.push_str(String::from_utf8(build.stdout).unwrap().as_str());
    distro.pop();

    distro
}

#[cfg(not(target_os = "macos"))]
pub fn os() -> String {
    let mut distro = String::new();

    let program = std::fs::read_to_string("/etc/os-release")
        .expect("Couldn't read file /etc/os-release")
        .into_bytes();

    distro.push_str(String::from_utf8(program).unwrap().as_str());

    let mut fallback = None;

    for i in distro.split('\n') {
        let mut j = i.split('=');

        match j.next().unwrap() {
            "PRETTY_NAME" => {
                return j.next().unwrap().trim_matches('"').to_string()
            }
            "NAME" => {
                fallback = Some(j.next().unwrap().trim_matches('"').to_string())
            }
            _ => {}
        }
    }

    if let Some(x) = fallback {
        return x;
    } else {
        return "unknown".to_string();
    }
}

#[cfg(target_os = "macos")]
#[inline(always)]
pub const fn env() -> DesktopEnv {
    DesktopEnv::Mac
}

#[cfg(not(target_os = "macos"))]
#[inline(always)]
pub fn env() -> DesktopEnv {
    match std::env::var_os("DESKTOP_SESSION") {
        Some(env) => {
            let env = env.to_str().unwrap().to_uppercase();

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
            } else {
                DesktopEnv::Unknown(env)
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
