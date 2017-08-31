// Whoami
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/lib.rs

//! Crate for getting the user's username and realname.

extern crate libc;

use std::ptr::{ null_mut };
use std::io::BufReader;
use std::io::Read;
use std::process::Command;
use std::process::Stdio;

pub enum DesktopEnv {
	Gnome,
	Windows,
	Unknown,
}

impl ::std::fmt::Display for DesktopEnv {
	fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
		use self::DesktopEnv::*;

		write!(f, "{}", match *self {
			Gnome => "gnome",
			Windows => "windows",
			Unknown => "unknown",
		})
	}
}

fn getpwuid() -> libc::passwd {
	let mut pwent = libc::passwd {
		pw_name: null_mut(),
		pw_passwd: null_mut(),
		pw_uid: 0,
		pw_gid: 0,
		pw_gecos: null_mut(),
		pw_dir: null_mut(),
		pw_shell: null_mut(),
	};
	let mut pwentp = null_mut();
	let mut buffer = [0i8;16384]; // from the man page

	unsafe {
		libc::getpwuid_r(libc::geteuid(), &mut pwent, &mut buffer[0],
			16384, &mut pwentp);
	}

	pwent
}

fn ptr_to_string(name: *mut i8) -> String {
	let uname = name as *mut _ as *mut u8;

	let s;
	let string;

	unsafe {
		s = ::std::slice::from_raw_parts(uname, libc::strlen(name));
		string = String::from_utf8_lossy(s).to_string();
	}

	string
}

/// Get the user's username.
pub fn username() -> String {
	let pwent = getpwuid();

	ptr_to_string(pwent.pw_name)
}

/// Get the user's full name.  Format: `FIRST_NAME [MIDDLE_NAME] [LAST_NAME]`
pub fn realname() -> String {
	let pwent = getpwuid();

	ptr_to_string(pwent.pw_gecos)
}

/// Get the computer's pretty name.
pub fn computer() -> String {
	let mut computer = String::new();

	let mut program = Command::new("hostnamectl")
		.arg("--pretty")
		.stdout(Stdio::piped())
		.spawn()
		.expect(&format!("Couldn't Find `hostnamectl`"));
	let mut pretty = BufReader::new(program.stdout.as_mut().unwrap());

	pretty.read_to_string(&mut computer).unwrap();

	computer.pop();

	computer
}

/// Get the computer's hostname.
pub fn hostname() -> String {
	let mut string = [0 as libc::c_char; 255];

	unsafe {
		libc::gethostname(&mut string[0], 255);
	}

	ptr_to_string(&mut string[0])
}

/// Get the OS.  Example: "Windows 10" or "Fedora 26 (Workstation Edition)"
pub fn os() -> String {
	if cfg!(target_os = "linux") {
		let mut distro = String::new();

		let mut program = Command::new("cat")
			.arg("/etc/os-release")
			.stdout(Stdio::piped())
			.spawn()
			.expect(&format!("Couldn't Find `cat`"));
		let mut pretty = BufReader::new(program.stdout.as_mut().unwrap());

		pretty.read_to_string(&mut distro).unwrap();

		for i in distro.split('\n') {
			let mut j = i.split('=');

			match j.next().unwrap() {
				"PRETTY_NAME" => return j.next().unwrap()
					.trim_matches('"').to_string(),
				_ => {},
			}
		}

		return "uknown".to_string();
	} else if cfg!(target_os = "windows") {
		"Windows 10".to_string() // TODO: Not all Windows is Windows 10
	} else {
		"unknown".to_string()
	}
}

/// Get the Desktop Environment.  Example: "gnome" or "windows"
#[inline(always)]
pub fn env() -> DesktopEnv {
	if cfg!(target_os = "linux") {
		match ::std::env::var_os("DESKTOP_SESSION") {
			Some(val) => match val.to_str().unwrap() {
				"gnome" => DesktopEnv::Gnome,
				_ => DesktopEnv::Unknown	
			},
			// TODO: Other Linux Desktop Environments
			None => DesktopEnv::Unknown
		}
	} else if cfg!(target_os = "windows") {
		DesktopEnv::Windows
		// TODO: Other Environments
	} else {
		DesktopEnv::Unknown
	}
}
