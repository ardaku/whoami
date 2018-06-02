// "whoami" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>

#[cfg(feature = "term")]
extern crate term;
extern crate whoami;

#[cfg(feature = "term")]
fn version() {
	let mut t = term::stdout().unwrap();
	t.fg(term::color::BRIGHT_BLUE).unwrap();
	write!(t, "whoami").unwrap();
	t.reset().unwrap();
	write!(t, " (Plop Grizzly) ").unwrap();
	t.fg(term::color::BRIGHT_GREEN).unwrap();
	writeln!(t, env!("CARGO_PKG_VERSION")).unwrap();
	t.reset().unwrap();
	writeln!(t, "Copyright (c) 2017-2018 Jeron A. Lau").unwrap();
	write!(t, "License: ").unwrap();
	writeln!(t, "MIT").unwrap();
	t.reset().unwrap();
}

#[cfg(feature = "term")]
fn help() {
	let mut t = term::stdout().unwrap();
//	t.attr(term::Attr::Bold).unwrap();
	write!(t, "Usage: ").unwrap();
//	t.reset().unwrap();
	writeln!(t, "whoami [OPTION]").unwrap();
	writeln!(t, "Print the name of the user who is logged in.").unwrap();
	writeln!(t).unwrap();
	writeln!(t, "    help        print this help and exit.").unwrap();
	writeln!(t, "    version     print version and exit").unwrap();
	writeln!(t, "    realname    print the user's first, middle, and last
                name, if they have been provided.").unwrap();
	writeln!(t, "    username    print the user's username.  This is the
                same as running with no arguments.").unwrap();
	writeln!(t, "    hostname    print the computer's hostname.").unwrap();
	writeln!(t, "    computer    print the computer's name.").unwrap();
	writeln!(t, "    allnames    print the user's realname and username.")
		.unwrap();

	writeln!(t, "    env         print the desktop environment.")
		.unwrap();
	writeln!(t, "    os          print the operating system name and \
		version.").unwrap();
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
				"realname" | "--realname" =>
					println!("{}", whoami::realname()),
				"username" | "--username" =>
					println!("{}", whoami::username()),
				// TODO: Set Hostname on Linux & Aldaron's OS
				"hostname" | "--hostname" =>
					println!("{}", whoami::hostname()),
				"computer" | "--computer" =>
					println!("{}", whoami::computer()),
				"allnames" | "--allnames" => {
					println!("username: {}\nrealname: {}\n\
						hostame: {}",
						whoami::username(),
						whoami::realname(),
						whoami::hostname());
				}
				"env" | "--env" =>
					println!("{}", whoami::env()),
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
