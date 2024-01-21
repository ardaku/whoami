fn main() {
    println!("WhoAmI {}", env!("CARGO_PKG_VERSION"));
    println!();
    println!(
        "User's Name            whoami::realname_os():     {:?}",
        whoami::realname_os()
    );
    println!(
        "User's Username        whoami::username_os():     {:?}",
        whoami::username()
    );
    println!(
        "User's Language        whoami::lang():            {:?}",
        whoami::langs()
            .map(|l| l.map(|l| l.to_string()).unwrap_or("??".to_string()))
            .collect::<Vec<String>>(),
    );
    println!(
        "Device's Pretty Name   whoami::devicename_os():   {:?}",
        whoami::devicename()
    );
    println!(
        "Device's Hostname      whoami::hostname_os():     {:?}",
        whoami::hostname()
    );
    println!(
        "Device's Platform      whoami::platform_os():     {:?}",
        whoami::platform()
    );
    println!(
        "Device's OS Distro     whoami::distro_os():       {:?}",
        whoami::distro()
    );
    println!(
        "Device's Desktop Env.  whoami::desktop_env():     {:?}",
        whoami::desktop_env()
    );
    println!(
        "Device's CPU Arch      whoami::arch():            {:?}",
        whoami::arch()
    );
}
