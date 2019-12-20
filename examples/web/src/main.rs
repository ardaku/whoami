extern {
    fn jscala_print(data: *const u16, size: usize);
}

macro_rules! print {
    ($($what:expr),* $(,)?) => {{
            let mut temp = String::new();
            $(temp.push_str(&format!("{}", $what));)*
            let temp: Vec<u16> = temp.encode_utf16().collect();
            unsafe { jscala_print(temp.as_ptr(), temp.len()); }
    }}
}

fn main() {
    print!(
        "--------------------------------------------------------------------\n\
         user's full name (user):              ", whoami::user(), "\n\
         username (username):                  ", whoami::username(), "\n\
         --------------------------------------------------------------------\n\
         host's fancy name (host):             ", whoami::host(), "\n\
         hostname (hostname):                  ", whoami::hostname(), "\n\
         --------------------------------------------------------------------\n\
         platform (platform):                  ", whoami::platform(), "\n\
         operating system (os):                ", whoami::os(), "\n\
         desktop environment (env):            ", whoami::env(), "\n\
         --------------------------------------------------------------------\n"
    );
}
