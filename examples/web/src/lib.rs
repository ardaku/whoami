use wasm_bindgen::prelude::*;

fn log(text: String) {
    web_sys::console::log_1(&text.into())
}

#[wasm_bindgen]
pub fn main() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    // Print out code from regular example.
    log(format!(
        "User's Name            whoami::realname():    {}",
        whoami::realname(),
    ));
    log(format!(
        "User's Username        whoami::username():    {}",
        whoami::username(),
    ));
    log(format!(
        "User's Languages       whoami::langs():        {:?}",
        whoami::langs()
            .map(|l| l.map(|l| l.to_string()).unwrap_or("??".to_string()))
            .collect::<Vec<String>>(),
    ));
    log(format!(
        "Device's Pretty Name   whoami::devicename():  {}",
        whoami::devicename(),
    ));
    log(format!(
        "Device's Hostname      whoami::fallible::hostname():    {}",
        whoami::fallible::hostname()
            .unwrap_or_else(|_| "localhost".to_string()),
    ));
    log(format!(
        "Device's Platform      whoami::platform():    {}",
        whoami::platform(),
    ));
    log(format!(
        "Device's OS Distro     whoami::distro():      {}",
        whoami::distro(),
    ));
    log(format!(
        "Device's Desktop Env.  whoami::desktop_env(): {}",
        whoami::desktop_env(),
    ));
    log(format!(
        "Device's CPU Arch      whoami::arch():        {}",
        whoami::arch(),
    ));
}
