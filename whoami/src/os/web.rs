#[cfg(not(any(target_pointer_width = "32", target_pointer_width = "64")))]
compile_error!("Unexpected pointer width for target platform");

use wasm_bindgen::prelude::*;
use web_sys::window;






#[wasm_bindgen]
pub fn browser() -> String {
    window()
        .and_then(|w| w.navigator().user_agent().ok())
        .unwrap_or_else(|| "Unknown Browser".to_string())
}

#[wasm_bindgen]
pub fn is_mobile() -> bool {
    window()
        .and_then(|w| Some(w.navigator().user_agent().ok()?))
        .map(|ua| {
            ua.to_lowercase().contains("mobile")
                || ua.to_lowercase().contains("android")
                || ua.to_lowercase().contains("iphone")
                || ua.to_lowercase().contains("ipad")
        })
        .unwrap_or(false)
}

#[wasm_bindgen]
pub fn timezone() -> String {
    let dtf = js_sys::Intl::DateTimeFormat::new(
        &js_sys::Array::new(),
        &js_sys::Object::new(),
    );

    let opts = dtf.resolved_options();

    // SAFELY READ opts.timeZone USING REFLECT
    let tz = js_sys::Reflect::get(&opts, &JsValue::from_str("timeZone"));

    match tz {
        Ok(val) => val.as_string().unwrap_or("Unknown".into()),
        Err(_) => "Unknown".into(),
    }
}




use std::{
    ffi::OsString,
    io::{Error, ErrorKind},
};



use crate::{
    os::{Os, Target},
    Arch, DesktopEnv, Language, LanguagePrefs, Platform, Result,
};



// Get the user agent
fn user_agent() -> Option<String> {
    window()?.navigator().user_agent().ok()
}

// Get the document domain
fn document_domain() -> Option<String> {
    window()?.document()?.location()?.hostname().ok()
}

impl Target for Os {
    fn lang_prefs(self) -> Result<LanguagePrefs> {
        if let Some(window) = window() {
            let langs = window
                .navigator()
                .languages()
                .to_vec()
                .into_iter()
                .filter_map(|l| l.as_string().map(Language::from))
                .collect::<Vec<_>>();
            Ok(LanguagePrefs {
                fallbacks: langs,
                ..Default::default()
            })
        } else {
            Err(Error::new(
                ErrorKind::NotFound,
                "Failed to retrieve languages: Window object is missing",
            ))
        }
    }

    fn realname(self) -> Result<OsString> {
        Ok("Anonymous".to_string().into())
    }

    fn username(self) -> Result<OsString> {
        Ok("anonymous".to_string().into())
    }

    fn devicename(self) -> Result<OsString> {
        let orig_string = user_agent().unwrap_or_default();
        let start = if let Some(s) = orig_string.rfind(' ') {
            s
        } else {
            return Ok("Unknown Browser".to_string().into());
        };
        let string = orig_string
            .get(start + 1..)
            .unwrap_or("Unknown Browser")
            .replace('/', " ");
        let string = if let Some(s) = string.rfind("Safari") {
            if let Some(s) = orig_string.rfind("Chrome") {
                if let Some(e) =
                    orig_string.get(s..).unwrap_or_default().find(' ')
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
        };

        Ok(string.into())
    }

    fn hostname(self) -> Result<String> {
        document_domain()
            .filter(|x| !x.is_empty())
            .ok_or_else(|| {
                Error::new(
                    ErrorKind::NotFound,
                    "Domain missing, failed to retrieve document domain from window"
                )
            })
    }

    fn distro(self) -> Result<String> {
        let string = user_agent()
            .ok_or_else(|| Error::from(ErrorKind::PermissionDenied))?;
        let err = || Error::new(ErrorKind::InvalidData, "Parsing failed");
        let begin = string.find('(').ok_or_else(err)?;
        let end = string.find(')').ok_or_else(err)?;
        let string = &string[begin + 1..end];

        Ok(if string.contains("Win32") || string.contains("Win64") {
            let begin = if let Some(b) = string.find("NT") {
                b
            } else {
                return Ok("Windows".to_string());
            };
            let end = if let Some(e) = string.find('.') {
                e
            } else {
                return Ok("Windows".to_string());
            };
            let string = &string[begin + 3..end];

            format!("Windows {string}")
        } else if string.contains("Linux") {
            let string = if string.contains("X11") || string.contains("Wayland")
            {
                let begin = if let Some(b) = string.find(';') {
                    b
                } else {
                    return Ok("Unknown Linux".to_string());
                };
                &string[begin + 2..]
            } else {
                string
            };

            if string.starts_with("Linux") {
                "Unknown Linux".to_string()
            } else {
                let end = if let Some(e) = string.find(';') {
                    e
                } else {
                    return Ok("Unknown Linux".to_string());
                };
                string[..end].to_string()
            }
        } else if let Some(begin) = string.find("Mac OS X") {
            if let Some(end) = string[begin..].find(';') {
                string[begin..begin + end].to_string()
            } else {
                string[begin..].to_string().replace('_', ".")
            }
        } else {
            string.to_string()
        })
    }

    #[inline(always)]
    fn desktop_env(self) -> Option<DesktopEnv> {
        Some(DesktopEnv::WebBrowser)
    }

    fn platform(self) -> Platform {
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
            Platform::Mac
        } else {
            Platform::Unknown(string.to_string())
        }
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
