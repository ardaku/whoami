// Whoami
// Copyright (c) 2017 Jeron Lau (Plop Grizzly) <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/linux.rs

use super::libc;
use super::DesktopEnv;

use std::ptr::{ null_mut };
use std::io::BufReader;
use std::io::Read;
use std::mem;
use std::process::Command;
use std::process::Stdio;

fn getpwuid(buffer: &mut [i8;16384]) -> libc::passwd {
	let mut pwent: libc::passwd = unsafe { mem::zeroed() };
	let mut pwentp = null_mut();

	unsafe {
		libc::getpwuid_r(libc::geteuid(), &mut pwent, &mut buffer[0],
			buffer.len(), &mut pwentp);
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

#[cfg(not(target_os = "windows"))]
pub fn username() -> String {
	let mut buffer = [0i8;16384]; // from the man page
	let pwent = getpwuid(&mut buffer);

	ptr_to_string(pwent.pw_name)
}

pub fn realname() -> String {
	let mut buffer = [0i8;16384]; // from the man page
	let pwent = getpwuid(&mut buffer);

	ptr_to_string(pwent.pw_gecos)
}

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

pub fn hostname() -> String {
	let mut string = [0 as libc::c_char; 255];

	unsafe {
		libc::gethostname(&mut string[0], 255);
	}

	ptr_to_string(&mut string[0])
}

pub fn os() -> String {
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
}

#[inline(always)]
pub fn env() -> DesktopEnv {
	match ::std::env::var_os("DESKTOP_SESSION") {
		Some(val) => match val.to_str().unwrap() {
			"gnome" => DesktopEnv::Gnome,
			_ => DesktopEnv::Unknown	
		},
		// TODO: Other Linux Desktop Environments
		None => DesktopEnv::Unknown
	}
}
