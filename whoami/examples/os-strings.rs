fn main() {
    println!("WhoAmI {}", env!("CARGO_PKG_VERSION"));
    println!();
    println!(
        "User's Language        whoami::lang_prefs():            {:?}",
        whoami::lang_prefs().unwrap_or_default()
    );
    println!(
        "User's Name            whoami::realname_os():           {:?}",
        whoami::realname_os()
            .unwrap_or_else(|_| "<unknown>".to_string().into()),
    );
    println!(
        "User's Username        whoami::username_os():           {:?}",
        whoami::username_os()
            .unwrap_or_else(|_| "<unknown>".to_string().into()),
    );
    println!(
        "User's Account         whoami::account_os():            {:?}",
        whoami::account_os().unwrap_or_else(|_| "<unknown>".to_string().into()),
    );
    println!(
        "Device's Pretty Name   whoami::devicename_os():         {:?}",
        whoami::devicename_os()
            .unwrap_or_else(|_| "<unknown>".to_string().into()),
    );
    println!(
        "Device's Hostname      whoami::hostname():              {:?}",
        whoami::hostname().unwrap_or_else(|_| "<unknown>".to_string()),
    );
    println!(
        "Device's Platform      whoami::platform():              {:?}",
        whoami::platform(),
    );
    println!(
        "Device's OS Distro     whoami::distro():                {:?}",
        whoami::distro(),
    );
    println!(
        "Device's Desktop Env.  whoami::desktop_env():           {:?}",
        whoami::desktop_env(),
    );
    println!(
        "Device's CPU Arch      whoami::cpu_arch():              {:?}",
        whoami::cpu_arch(),
    );
}
