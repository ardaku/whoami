// Whoami
// Copyright (c) 2017 Jeron Lau (Plop Grizzly) <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/main.rs

#[cfg(feature = "term")]
extern crate term;
extern crate whoami;

#[cfg(feature = "term")]
fn version() {
	let mut t = term::stdout().unwrap();
	t.fg(term::color::BRIGHT_BLUE).unwrap();
	write!(t, "whoami").unwrap();
	t.reset().unwrap();
	write!(t, ".Plop Grizzly ").unwrap();
	t.fg(term::color::BRIGHT_GREEN).unwrap();
	writeln!(t, env!("CARGO_PKG_VERSION")).unwrap();
	t.reset().unwrap();
	write!(t, concat!("\nCopyright: ")).unwrap();
	writeln!(t, "(C) 2017 Jeron Aldaron Lau (Plop Grizzly) \
		<jeron.lau@plopgrizzly.com>");
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
	let mut args = &mut ::std::env::args();

	if let Some(a) = args.nth(1) {
		if args.count() > 2 {
			println!("too many arguments, try `whoami help`");
		} else {
			match a.as_str() {
				"help" => help(),
				"version" => version(),
				"realname" => println!("{}", whoami::realname()),
				"username" => println!("{}", whoami::username()),
				// TODO: Set Hostname on Linux & Aldaron's OS
				"hostname" => println!("{}", whoami::hostname()),
				"computer" => println!("{}", whoami::computer()),
				"allnames" => println!(
					"username: {}\nrealname: {}\nhostame: {\
					}",
					whoami::username(), whoami::realname(),
					whoami::hostname()),
				"env" => println!("{}", whoami::env()),
				"os" => println!("{}", whoami::os()),
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