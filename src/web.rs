#[cfg(not(any(target_pointer_width = "32", target_pointer_width = "64")))]
compile_error!("Unexpected pointer width for target platform");

use std::ffi::OsString;

use wasm_bindgen::JsValue;
use web_sys::window;

use crate::{Arch, DesktopEnv, Platform};

// Get the user agent
fn user_agent() -> Option<String> {
    window()?.navigator().user_agent().ok()
}

// Get the document domain
fn document_domain() -> Option<String> {
    window()?.document()?.location()?.hostname().ok()
}

struct LangIter {
    array: Vec<JsValue>,
    index: usize,
}

impl Iterator for LangIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(value) = self.array.get(self.index) {
            self.index += 1;
            if let Some(lang) = value.as_string() {
                Some(lang)
            } else {
                self.next()
            }
        } else {
            None
        }
    }
}

#[inline(always)]
pub(crate) fn lang() -> impl Iterator<Item = String> {
    let array = if let Some(window) = window() {
        window.navigator().languages().to_vec()
    } else {
        Vec::new()
    };
    let index = 0;

    LangIter { array, index }
}

#[inline(always)]
pub(crate) fn username_os() -> OsString {
    username().into()
}

#[inline(always)]
pub(crate) fn realname_os() -> OsString {
    realname().into()
}

#[inline(always)]
pub(crate) fn devicename_os() -> OsString {
    devicename().into()
}

#[inline(always)]
pub(crate) fn distro_os() -> Option<OsString> {
    distro().map(|a| a.into())
}

#[inline(always)]
pub(crate) fn username() -> String {
    "anonymous".to_string()
}

#[inline(always)]
pub(crate) fn realname() -> String {
    "Anonymous".to_string()
}

pub(crate) fn devicename() -> String {
    let orig_string = user_agent().unwrap_or_default();

    let start = if let Some(s) = orig_string.rfind(' ') {
        s
    } else {
        return "Unknown Browser".to_string();
    };

    let string = orig_string
        .get(start + 1..)
        .unwrap_or("Unknown Browser")
        .replace('/', " ");

    if let Some(s) = string.rfind("Safari") {
        if let Some(s) = orig_string.rfind("Chrome") {
            if let Some(e) = orig_string.get(s..).unwrap_or_default().find(' ')
            {
                orig_string
                    .get(s..)
                    .unwrap_or("Chrome")
                    .get(..e)
                    .unwrap_or("Chrome")
                    .replace('/', " ")
            } else {
                "Chrome".to_string()
            }
        } else if orig_string.contains("Linux") {
            "GNOME Web".to_string()
        } else {
            string.get(s..).unwrap_or("Safari").replace('/', " ")
        }
    } else if string.contains("Edg ") {
        string.replace("Edg ", "Edge ")
    } else if string.contains("OPR ") {
        string.replace("OPR ", "Opera ")
    } else {
        string
    }
}

#[inline(always)]
pub(crate) fn hostname() -> String {
    document_domain()
        .filter(|x| !x.is_empty())
        .unwrap_or_else(|| "localhost".to_string())
}

pub(crate) fn distro() -> Option<String> {
    let string = user_agent()?;

    let begin = string.find('(')?;
    let end = string.find(')')?;
    let string = &string[begin + 1..end];

    if string.contains("Win32") || string.contains("Win64") {
        let begin = if let Some(b) = string.find("NT") {
            b
        } else {
            return Some("Windows".to_string());
        };
        let end = if let Some(e) = string.find('.') {
            e
        } else {
            return Some("Windows".to_string());
        };
        let string = &string[begin + 3..end];

        Some(format!("Windows {}", string))
    } else if string.contains("Linux") {
        let string = if string.contains("X11") || string.contains("Wayland") {
            let begin = if let Some(b) = string.find(';') {
                b
            } else {
                return Some("Unknown Linux".to_string());
            };
            &string[begin + 2..]
        } else {
            string
        };

        if string.starts_with("Linux") {
            Some("Unknown Linux".to_string())
        } else {
            let end = if let Some(e) = string.find(';') {
                e
            } else {
                return Some("Unknown Linux".to_string());
            };
            Some(string[..end].to_string())
        }
    } else if let Some(begin) = string.find("Mac OS X") {
        Some(if let Some(end) = string[begin..].find(';') {
            string[begin..begin + end].to_string()
        } else {
            string[begin..].to_string().replace('_', ".")
        })
    } else {
        // TODO:
        // Platform::FreeBsd,
        // Platform::Ios,
        // Platform::Android,
        // Platform::Nintendo,
        // Platform::Xbox,
        // Platform::PlayStation,
        // Platform::Dive,
        // Platform::Fuchsia,
        // Platform::Redox
        Some(string.to_string())
    }
}

pub(crate) const fn desktop_env() -> DesktopEnv {
    DesktopEnv::WebBrowser
}

pub(crate) fn platform() -> Platform {
    let string = user_agent().unwrap_or_default();

    let begin = if let Some(b) = string.find('(') {
        b
    } else {
        return Platform::Unknown("Unknown".to_string());
    };
    let end = if let Some(e) = string.find(')') {
        e
    } else {
        return Platform::Unknown("Unknown".to_string());
    };
    let string = &string[begin + 1..end];

    if string.contains("Win32") || string.contains("Win64") {
        Platform::Windows
    } else if string.contains("Linux") {
        Platform::Linux
    } else if string.contains("Mac OS X") {
        Platform::MacOS
    } else {
        // TODO:
        // Platform::FreeBsd,
        // Platform::Ios,
        // Platform::Android,
        // Platform::Nintendo,
        // Platform::Xbox,
        // Platform::PlayStation,
        // Platform::Dive,
        // Platform::Fuchsia,
        // Platform::Redox,
        Platform::Unknown(string.to_string())
    }
}

pub(crate) fn arch() -> Arch {
    if cfg!(target_pointer_width = "64") {
        Arch::Wasm64
    } else {
        Arch::Wasm32
    }
}
