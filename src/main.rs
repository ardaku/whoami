// Whoami
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/main.rs

#[cfg(feature = "term")]
extern crate term;

#[allow(dead_code)] // Because it's a library
mod lib;

#[cfg(feature = "term")]
fn version() {
	let mut t = term::stdout().unwrap();
	t.attr(term::Attr::Bold).unwrap();
	t.attr(term::Attr::Underline(true)).unwrap();
	t.fg(term::color::BRIGHT_BLUE).unwrap();
	write!(t, "whoami").unwrap();
	t.reset().unwrap();
	write!(t, ".Aldaron's Tech ").unwrap();
	t.attr(term::Attr::Italic(true)).unwrap();
	writeln!(t, env!("CARGO_PKG_VERSION")).unwrap();
	t.reset().unwrap();
	write!(t, concat!("\nCopyright: ")).unwrap();
	t.attr(term::Attr::Italic(true)).unwrap();
	writeln!(t, concat!("(C) 2017 Aldaron's Tech, ",
		env!("CARGO_PKG_AUTHORS"))).unwrap();
	t.attr(term::Attr::Italic(false)).unwrap();
	write!(t, "License: ").unwrap();
	t.attr(term::Attr::Italic(true)).unwrap();
	writeln!(t, "MIT").unwrap();
	t.reset().unwrap();
}

#[cfg(feature = "term")]
fn help() {
	let mut t = term::stdout().unwrap();
	t.attr(term::Attr::Bold).unwrap();
	write!(t, "Usage: ").unwrap();
	t.reset().unwrap();
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
			println!("too many arguments, try `whoami --help`");
		} else {
			match a.as_str() {
				"help" => help(),
				"version" => version(),
				"realname" => println!("{}", lib::realname()),
				"username" => println!("{}", lib::username()),
				// TODO: Set Hostname on Linux & Aldaron's OS
				"hostname" => println!("{}", lib::hostname()),
				"computer" => println!("{}", lib::computer()),
				"allnames" => println!(
					"username: {}\nrealname: {}\nhostame: {\
					}",
					lib::username(), lib::realname(),
					lib::hostname()),
				"env" => println!("{}", lib::env()),
				"os" => println!("{}", lib::os()),
				a => {
					print!("Unknown Argument: {}\n\n", a);
					help();
				}
			}
		}
	} else {
		println!("{}", lib::username()); // no args
	}
}
