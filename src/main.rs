// Copyright © Jeron Lau 2017 - 2019.
// Dual-licensed under either the MIT License or the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at https://www.boost.org/LICENSE_1_0.txt)

#[cfg(feature = "term")]
extern crate term;
extern crate whoami;

#[cfg(feature = "term")]
fn version() {
    let mut t = term::stdout().unwrap();
    t.fg(term::color::BRIGHT_RED).unwrap();
    write!(t, "jeronaldaron.").unwrap();
    t.fg(term::color::BRIGHT_BLUE).unwrap();
    t.attr(term::Attr::Bold).unwrap();
    write!(t, env!("CARGO_PKG_NAME")).unwrap();
    t.reset().unwrap();
    write!(t, " ").unwrap();
    t.fg(term::color::BRIGHT_GREEN).unwrap();
    writeln!(t, env!("CARGO_PKG_VERSION")).unwrap();
    t.reset().unwrap();
    t.attr(term::Attr::Bold).unwrap();
    write!(t, "Copyright ©").unwrap();
    t.reset().unwrap();
    write!(t, " ").unwrap();
    t.fg(term::color::MAGENTA).unwrap();
    writeln!(t, "Jeron Lau 2017 - 2019.").unwrap();
    t.reset().unwrap();
    t.attr(term::Attr::Bold).unwrap();
    write!(t, "License ").unwrap();
    t.reset().unwrap();
    t.fg(term::color::MAGENTA).unwrap();
    writeln!(t, "MIT / BSL-1.0").unwrap();
    t.reset().unwrap();
}

#[cfg(feature = "term")]
fn help() {
    let mut t = term::stdout().unwrap();
    t.attr(term::Attr::Bold).unwrap();
    write!(t, "Usage ").unwrap();
    t.reset().unwrap();
    t.fg(term::color::BRIGHT_GREEN).unwrap();
    write!(t, "whoami ").unwrap();
    t.fg(term::color::BRIGHT_CYAN).unwrap();
    writeln!(t, "[OPTION]").unwrap();
    t.reset().unwrap();
    writeln!(t, "Print the name of the user who is logged in.").unwrap();
    writeln!(t).unwrap();
    t.fg(term::color::BRIGHT_CYAN).unwrap();
    write!(t, "        help            ").unwrap();
    t.reset().unwrap();
    writeln!(t, "Print this help and exit.").unwrap();
    t.fg(term::color::BRIGHT_CYAN).unwrap();
    write!(t, "        version         ").unwrap();
    t.reset().unwrap();
    writeln!(t, "Print version and exit").unwrap();
    t.fg(term::color::BRIGHT_CYAN).unwrap();
    write!(t, "        user            ").unwrap();
    t.reset().unwrap();
    writeln!(t, "Print the user's full name.").unwrap();
    t.fg(term::color::BRIGHT_CYAN).unwrap();
    write!(t, "        username        ").unwrap();
    t.reset().unwrap();
    writeln!(t, "Print the user's username.  This is the same as running
                        with no arguments.")
        .unwrap();

    t.fg(term::color::BRIGHT_CYAN).unwrap();
    write!(t, "        host            ").unwrap();
    t.reset().unwrap();
    writeln!(t, "Print the host device's (pretty) name.").unwrap();

    t.fg(term::color::BRIGHT_CYAN).unwrap();
    write!(t, "        hostname        ").unwrap();
    t.reset().unwrap();
    writeln!(t, "Print the host device's hostname.").unwrap();

    t.fg(term::color::BRIGHT_CYAN).unwrap();
    write!(t, "        env             ").unwrap();
    t.reset().unwrap();
    writeln!(t, "Print the desktop environment.").unwrap();

    t.fg(term::color::BRIGHT_CYAN).unwrap();
    write!(t, "        os              ").unwrap();
    t.reset().unwrap();
    writeln!(t, "Print the operating system name and version.").unwrap();

    t.fg(term::color::BRIGHT_CYAN).unwrap();
    write!(t, "        print           ").unwrap();
    t.reset().unwrap();
    writeln!(t, "Print everything known by whoami.").unwrap();

    writeln!(t).unwrap();
}

#[cfg(feature = "term")]
fn main() {
    let args = &mut ::std::env::args();

    if let Some(a) = args.nth(1) {
        if args.count() > 2 {
            println!("too many arguments, try `whoami help`");
        } else {
            match a.as_str() {
                "help" | "--help" => help(),
                "version" | "--version" => version(),
                "user" | "--user" => println!("{}", whoami::user()),
                "username" | "--username" => println!("{}", whoami::username()),
                // TODO: Set Hostname.
                "hostname" | "--hostname" => println!("{}", whoami::hostname()),
                "host" | "--host" => println!("{}", whoami::host()),
                "print" | "--print" => {
                    print!(
                        "user = {}\nusername = {}\n\
                         host = {}\nhostname = {}\n\
                         env = {}\nos = {}\n",
                        whoami::user(),
                        whoami::username(),
                        whoami::host(),
                        whoami::hostname(),
                        whoami::env(),
                        whoami::os(),
                    );
                }
                "env" | "--env" => println!("{}", whoami::env()),
                "os" | "--os" => println!("{}", whoami::os()),
                a => {
                    print!("Unknown Argument: {}\n\n", a);
                    help();
                }
            }
        }
    } else {
        println!("{}", whoami::username()); // no args
    }
}
