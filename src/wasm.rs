use std::sync::Once;
use std::mem::MaybeUninit;

use cala_core::os::web::{JsString, JsFn};

use crate::{DesktopEnv, Platform};

static mut USER_AGENT: MaybeUninit<JsFn> = MaybeUninit::uninit();
static INIT: Once = Once::new();

// Get the user agent
fn user_agent() -> String {
    unsafe {
        INIT.call_once(|| {
            USER_AGENT = MaybeUninit::new(
                JsFn::new("return _cala_js_malloc(navigator.userAgent);")
            );
        });
        let user_agent = &*USER_AGENT.as_ptr();
        let string = JsString::from_var(user_agent.call(None, None).unwrap());
        let vec = string.as_var().as_vec();
        String::from_utf16_lossy(&vec)
    }
}

#[inline(always)]
pub fn username() -> String {
    "anonymous".to_string()
}

#[inline(always)]
pub fn realname() -> String {
    "Anonymous".to_string()
}

pub fn computer() -> String {
    let orig_string = user_agent();

    let end = if let Some(e) = orig_string.rfind("/") {
        e
    } else {
        return "Unknown Browser".to_string();
    };
    let start = if let Some(s) = orig_string.rfind(" ") {
        s
    } else {
        return "Unknown Browser".to_string();
    };

    let string = orig_string.get(start + 1..end).unwrap().to_string();

    if string == "Safari" {
        if orig_string.contains("Chrome") {
            "Chrome".to_string()
        } else {
            "Safari".to_string()
        }
    } else {
        string
    }
}

#[inline(always)]
pub fn hostname() -> String {
    "localhost".to_string()
}

pub fn os() -> Option<String> {
    let string = user_agent();

    let begin = if let Some(b) = string.find('(') {
        b
    } else {
        return None;
    };
    let end = if let Some(e) = string.find(')') {
        e
    } else {
        return None;
    };
    let string = &string[begin + 1..end];

    if string.contains("Win32") || string.contains("Win64") {
        let begin = if let Some(b) = string.find("NT") {
            b
        } else {
            return Some("Windows".to_string());
        };
        let end = if let Some(e) = string.find(".") {
            e
        } else {
            return Some("Windows".to_string());
        };
        let string = &string[begin + 3..end];

        Some(format!("Windows {}", string))
    } else if string.contains("Linux") {
        let string = if string.contains("X11") || string.contains("Wayland") {
            let begin = if let Some(b) = string.find(";") {
                b
            } else {
                return Some("Unknown Linux".to_string());
            };
            let string = &string[begin + 2..];

            string
        } else {
            string
        };

        if string.starts_with("Linux") {
            Some("Unknown Linux".to_string())
        } else {
            let end = if let Some(e) = string.find(";") {
                e
            } else {
                return Some("Unknown Linux".to_string());
            };
            Some(string[..end].to_string())
        }
    } else if string.contains("Mac OS X") {
        let begin = string.find("Mac OS X").unwrap();
        Some(if let Some(end) = string[begin..].find(";") {
            string[begin..begin + end].to_string()
        } else {
            string[begin..].to_string().replace("_", ".")
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

pub const fn env() -> DesktopEnv {
    DesktopEnv::Wasm
}

pub fn platform() -> Platform {
    let string = user_agent();

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
