fn main() {
    println!("WhoAmI {}", env!("CARGO_PKG_VERSION"));
    println!();
    println!(
        "User's Language        whoami::lang_prefs():          {}",
        whoami::lang_prefs().unwrap_or_default()
    );
    println!(
        "User's Name            whoami::realname():            {}",
        whoami::realname().unwrap_or_else(|_| "<unknown>".to_string()),
    );
    println!(
        "User's Username        whoami::username():            {}",
        whoami::username().unwrap_or_else(|_| "<unknown>".to_string()),
    );
    println!(
        "User's Username        whoami::account():             {}",
        whoami::account().unwrap_or_else(|_| "<unknown>".to_string()),
    );
    println!(
        "Device's Pretty Name   whoami::devicename():          {}",
        whoami::devicename().unwrap_or_else(|_| "<unknown>".to_string()),
    );
    println!(
        "Device's Hostname      whoami::hostname():            {}",
        whoami::hostname().unwrap_or_else(|_| "<unknown>".to_string()),
    );
    println!(
        "Device's Platform      whoami::platform():            {}",
        whoami::platform(),
    );
    println!(
        "Device's OS Distro     whoami::distro():              {}",
        whoami::distro().unwrap_or_else(|_| "<unknown>".to_string()),
    );
    println!(
        "Device's Desktop Env.  whoami::desktop_env():         {}",
        whoami::desktop_env()
            .map(|e| e.to_string())
            .unwrap_or_else(|| "<unknown>".to_string()),
    );
    println!(
        "Device's CPU Arch      whoami::cpu_arch():            {}",
        whoami::cpu_arch(),
    );
}
