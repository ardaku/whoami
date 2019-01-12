// Copyright © Jeron Lau 2017 - 2019.
// Dual-licensed under either the MIT License or the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at https://www.boost.org/LICENSE_1_0.txt)

use super::libc;
use super::DesktopEnv;

use std::io::BufReader;
use std::io::Read;
use std::mem;
use std::process::Command;
use std::process::Stdio;
use std::ptr::null_mut;

fn getpwuid(buffer: &mut [libc::c_char; 16384]) -> libc::passwd {
    let mut pwent: libc::passwd = unsafe { mem::zeroed() };
    let mut pwentp = null_mut();

    unsafe {
        libc::getpwuid_r(
            libc::geteuid(),
            &mut pwent,
            &mut buffer[0],
            buffer.len(),
            &mut pwentp,
        );
    }

    pwent
}

fn ptr_to_string(name: *mut libc::c_char) -> String {
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
    let mut buffer = [0 as libc::c_char; 16384]; // from the man page
    let pwent = getpwuid(&mut buffer);

    ptr_to_string(pwent.pw_name)
}

fn fancy_fallback(computer: &mut String, fallback_fn: fn() -> String) {
    let mut cap = true;

    if computer.is_empty() {
        let fallback = fallback_fn();

        for c in fallback.chars() {
            match c {
                '.' | '-' | '_' => {
                    computer.push(' ');
                    cap = true;
                }
                a => {
                    if cap {
                        cap = false;
                        for i in a.to_uppercase() {
                            computer.push(i);
                        }
                    } else {
                        computer.push(a);
                    }
                }
            }
        }
    }
}

pub fn realname() -> String {
    let mut buffer = [0 as libc::c_char; 16384]; // from the man page
    let pwent = getpwuid(&mut buffer);
    let mut realname = ptr_to_string(pwent.pw_gecos);

    fancy_fallback(&mut realname, username);

    realname
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

    fancy_fallback(&mut computer, hostname);

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

    let mut fallback = None;

    for i in distro.split('\n') {
        let mut j = i.split('=');

        match j.next().unwrap() {
            "PRETTY_NAME" => return j.next().unwrap().trim_matches('"').to_string(),
            "NAME" => fallback = Some(j.next().unwrap().trim_matches('"').to_string()),
            _ => {}
        }
    }

    if let Some(x) = fallback {
        return x;
    } else {
        return "unknown".to_string();
    }
}

#[inline(always)]
pub fn env() -> DesktopEnv {
    match ::std::env::var_os("DESKTOP_SESSION") {
        Some(env) => {
            let env = env.to_str().unwrap().to_uppercase();

            if env.contains("GNOME") {
                DesktopEnv::Gnome
            } else if env.contains("LXDE") {
                DesktopEnv::Lxde
            } else if env.contains("OPENBOX") {
                DesktopEnv::Openbox
            } else if env.contains("I3") {
                DesktopEnv::I3
            } else if env.contains("UBUNTU") {
                DesktopEnv::Ubuntu
            } else {
                DesktopEnv::Unknown(env)
            }
        }
        // TODO: Other Linux Desktop Environments
        None => DesktopEnv::Unknown("Unknown".to_string()),
    }
}
