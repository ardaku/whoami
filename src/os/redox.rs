#![forbid(unsafe_code)]

use std::{io::Error, ffi::OsString, fs, borrow::Cow};

use redox_syscall::{call, error};

use crate::{
    os::{Os, Target},
    Arch, DesktopEnv, Language, Platform, Result
};

/// Row in the Redox /etc/passwd file
struct Passwd<'a>(Cow<'a, str>);

impl Passwd<'_> {
    fn column(&self, number: usize) -> Option<&str> {
        self.split(';').skip(number).next()
    }

    fn username(&self) -> Option<String> {
        self.column(0).map(ToString::to_string)
    }

    fn uid(&self) -> Option<usize> {
        self.column(1)?.parse().ok()
    }

    fn gid(&self) -> Option<usize> {
        self.column(2)?.parse().ok()
    }

    fn fullname(&self) -> Option<String> {
        self.column(3).map(ToString::to_string)
    }
}

struct Uname<'a>(Cow<'a, str>);

impl Uname {
    fn row(&self, number: usize) -> Option<&str> {
        self.lines().skip(number).next()
    }

    fn kernel_name(&self) -> Option<String> {
        self.row(0).map(ToString::to_string)
    }

    fn kernel_release(&self) -> Option<String> {
        self.row(2).map(ToString::to_string)
    }

    fn machine_arch(&self) -> Option<Arch> {
        // FIXME: Don't hardcode unknown arch
        Some(Arc::Unknown(self.row(4)?))
    }
}

fn to_io_error(error: error::Error) -> Error {
    Error::from_raw_os_error(error.errno)
}

fn euid() -> Result<usize> {
    call::geteuid().map_err(to_io_error)
}

fn egid() -> Result<usize> {
    call::getegid().map_err(to_io_error)
}

fn passwd() -> Result<Passwd<'static>> {
    let (euid, egid) = (euid()?, egid()?);
    let passwd_file = fs::read_to_string("/etc/passwd")?;

    for user in passwd_file.lines() {
        let passwd = Passwd(user.into());

        if passwd.uid() == Some(euid) && passwd.gid() == Some(egid) {
            return Ok(Passwd(passwd.0.to_owned()));
        }
    }

    Err(Error::new(ErrorKind::NotFound, "Missing record"))
}

fn uname() -> Result<Uname<'static>> {
    let uname_file = fs::read_to_string("sys:uname")?;

    Ok(Uname(uname_file.into()))
}

fn redox_version() -> Result<String> {
    let release_file = fs::read_to_string("/etc/redox-release")?;

    Ok(release_file.lines().next().unwrap_or_default().to_string())
}

fn hostname() -> Result<String> {
    let hostname_file = fs::read_to_string("/etc/hostname")?;

    Ok(hostname_file.lines().next().unwrap_or_default().to_string())
}

#[inline(always)]
pub(crate) fn lang() -> impl Iterator<Item = String> {
    std::iter::once("en-US".to_string())
}

impl Target for Os {
    fn langs(self) -> Vec<Language> {
        todo!()
    }

    #[inline(always)]
    fn realname(self) -> Result<OsString> {
        Ok(passwd()?.fullname().unwrap_or_default().into())
    }

    #[inline(always)]
    fn username(self) -> Result<OsString> {
        Ok(passwd()?.username().unwrap_or_default().into())
    }

    #[inline(always)]
    fn devicename(self) -> Result<OsString> {
        hostname().map(OsString::from)
    }

    #[inline(always)]
    fn hostname(self) -> Result<String> {
        hostname()
    }

    #[inline(always)]
    fn distro(self) -> Result<String> {
        let version = redox_version();
        let mut distro_name = "Redox ".to_string();

        distro_name.push_str(&version);
        distro_name
    }

    #[inline(always)]
    fn desktop_env(self) -> DesktopEnv {
        DesktopEnv::Orbital
    }

    #[inline(always)]
    fn platform(self) -> Platform {
        Platform::Redox
    }

    #[inline(always)]
    fn arch(self) -> Result<Arch> {
        uname().arch()
    }
}
