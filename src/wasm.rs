use crate::{DesktopEnv, Platform};
use stdweb::{self, js};

fn user_agent() -> String {
    let value: stdweb::Value = js! {
        var string = navigator.userAgent;
        return string;
    };

    use stdweb::Value::*;

    match value {
        String(string) => string, // js! { console.log(@{string}) },
        _ => unreachable!(),
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

    js! {
        console.log(@{&string});
    };

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

pub fn os() -> String {
    let string = user_agent();

    let begin = if let Some(b) = string.find('(') {
        b
    } else {
        return "Unknown".to_string();
    };
    let end = if let Some(e) = string.find(')') {
        e
    } else {
        return "Unknown".to_string();
    };
    let string = &string[begin + 1..end];

    if string.contains("Win32") || string.contains("Win64") {
        let begin = if let Some(b) = string.find("NT") {
            b
        } else {
            return "Windows".to_string();
        };
        let end = if let Some(e) = string.find(".") {
            e
        } else {
            return "Windows".to_string();
        };
        let string = &string[begin + 3..end];

        format!("Windows {}", string)
    } else if string.contains("Linux") {
        let string = if string.contains("X11") || string.contains("Wayland") {
            let begin = if let Some(b) = string.find(";") {
                b
            } else {
                return "Unknown Linux".to_string();
            };
            let string = &string[begin + 2..];

            string
        } else {
            string
        };

        if string.starts_with("Linux") {
            "Unknown Linux".to_string()
        } else {
            let end = if let Some(e) = string.find(";") {
                e
            } else {
                return "Unknown Linux".to_string();
            };
            string[..end].to_string()
        }
    } else if string.contains("OSX") {
        "Mac OS".to_string()
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
        string.to_string()
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
    } else if string.contains("OSX") {
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