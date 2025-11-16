use colored::*;
use whoami;

fn main() {
    let username = whoami::username().unwrap_or("unknown".into());
    let realname = whoami::realname().unwrap_or("unknown".into());
    let platform = whoami::platform().to_string();
    let hostname = whoami::hostname().unwrap_or("unknown".into());
    let device = whoami::devicename().unwrap_or("unknown".into());

    println!("{}", "┌───────────────────────────────────┐".bright_magenta());
    println!("{} {}", "│ Hello,".bright_magenta(), username.bright_green());
    println!("{} {}", "│ Real Name:".bright_magenta(), realname.cyan());
    println!("{} {}", "│ Platform:".bright_magenta(), platform.yellow());
    println!("{} {}", "│ Hostname:".bright_magenta(), hostname.blue());
    println!("{} {}", "│ Device:".bright_magenta(), device.bright_yellow());
    println!("{}", "└───────────────────────────────────┘".bright_magenta());
}
