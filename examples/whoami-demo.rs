fn main() {
    println!("WhoAmI {}", env!("CARGO_PKG_VERSION"));
    println!();
    println!(
        "User→Name      whoami::realname():    {}",
        whoami::realname()
    );
    println!(
        "User→Username  whoami::username():    {}",
        whoami::username()
    );
    println!(
        "Host→Name      whoami::devicename():  {}",
        whoami::devicename()
    );
    println!(
        "Host→Hostname  whoami::hostname():    {}",
        whoami::hostname()
    );
    println!(
        "Platform       whoami::platform():    {}",
        whoami::platform()
    );
    println!("OS Distro      whoami::distro():      {}", whoami::distro());
    println!(
        "Desktop Env.   whoami::desktop_env(): {}",
        whoami::desktop_env()
    );
}
