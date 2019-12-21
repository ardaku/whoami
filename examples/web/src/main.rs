extern {
    fn console_warn(data: *const u16, size: usize);
    fn console_info(data: *const u16, size: usize);
    fn console_debug(data: *const u16, size: usize);
}

fn _out(string: &str) {
    let string: Vec<u16> = string.encode_utf16().collect();
    unsafe { console_info(string.as_ptr(), string.len()); }
}

fn _dev(string: &str) {
    let string: Vec<u16> = string.encode_utf16().collect();
    unsafe { console_debug(string.as_ptr(), string.len()); }
}

fn _fix(string: &str) {
    let string: Vec<u16> = string.encode_utf16().collect();
    unsafe { console_warn(string.as_ptr(), string.len()); }
}

/// Print to stdout.  Use for messages that will print in production.
macro_rules! out {
    () => {{
        _out("");
    }};
    ($($arg:tt)*) => {{
        _out(&format!($($arg)*));
    }};
}

/// Print to stderr.  Use for messages that will print in development, but not
/// in production.
macro_rules! dev {
    () => {{
        _dev("");
    }};
    ($($arg:tt)*) => {{
        _dev(&format!($($arg)*));
    }};
}

/// Print to stderr.  Use for runtime cases where something should be fixed.  It
/// does not have to be detremental.
macro_rules! fix {
    () => {{
        _fix("");
    }};
    ($($arg:tt)*) => {{
        _fix(&format!($($arg)*));
    }};
}

fn main() {
    fix!("FIXME");
    dev!("Test");
    out!(
        "--------------------------------------------------------------------\n\
         user's full name (user):              {}\n\
         username (username):                  {}\n\
         --------------------------------------------------------------------\n\
         host's fancy name (host):             {}\n\
         hostname (hostname):                  {}\n\
         --------------------------------------------------------------------\n\
         platform (platform):                  {}\n\
         operating system (os):                {}\n\
         desktop environment (env):            {}\n\
         --------------------------------------------------------------------\n",
        whoami::user(),
        whoami::username(),
        whoami::host(),
        whoami::hostname(),
        whoami::platform(),
        whoami::os(),
        whoami::env(),
    );
}
