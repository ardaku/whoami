// "whoami" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>

use super::DesktopEnv;

pub fn username() -> String {
	extern "system" {
		fn GetUserNameW(a: *mut u16, b: *mut usize) -> i32;
	}

	let mut name = [0; 256];
	let mut size = 256;

	unsafe {
		GetUserNameW(&mut name[0], &mut size);
	}
	
	if size == 0 { String::new() }
	else { String::from_utf16_lossy(&name[..size-1]) }
}

pub fn realname() -> String {
	username()
}

pub fn computer() -> String {
	hostname()
}

pub fn hostname() -> String {
	extern "system" {
		fn GetComputerNameW(a: *mut u16, b: *mut usize) -> i32;
	}

	let mut name = [0; 256];
	let mut size = 256;

	unsafe {
		GetComputerNameW(&mut name[0], &mut size);
	}
	
	String::from_utf16_lossy(&name[..size])
}

pub fn os() -> String {
	extern "system" {
		fn GetVersion() -> usize;
	}
	
	let bits = unsafe {
		GetVersion()
	} as u32;
	
	let mut out = "Windows ".to_string();
	
	let major: u8 = ((bits & 0b00000000_00000000_00000000_11111111) >> 0) as u8;
	let minor: u8 = ((bits & 0b00000000_00000000_11111111_00000000) >> 8) as u8;
	let build: u16 = ((bits & 0b11111111_11111111_00000000_00000000) >> 16) as u16;

	match major {
		5 => out.push_str("XP"),
		6 => match minor {
			0 => out.push_str("Vista"),
			1 => out.push_str("7"),
			2 => match build {
				9200 => out.push_str("10"),
				_ => out.push_str("8")
			},
			_ => out.push_str("8"),
		}
		_ => out.push_str("Unknown")
	}
	
	out
}

#[inline(always)]
pub fn env() -> DesktopEnv {
	DesktopEnv::Windows
}
