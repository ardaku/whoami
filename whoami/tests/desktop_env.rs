#![cfg(all(unix, not(target_vendor = "apple"), feature = "std"))]

use whoami::DesktopEnvironment;

#[test]
fn detects_mate_desktop_environment() {
    for name in ["SSH_CLIENT", "SSH_TTY", "SSH_CONNECTION"] {
        std::env::remove_var(name);
    }
    std::env::set_var("XDG_SESSION_DESKTOP", "MATE");

    assert_eq!(whoami::desktop_env(), Some(DesktopEnvironment::Mate));
}
