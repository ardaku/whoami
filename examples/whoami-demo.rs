fn main() {
    print!(
        "whoami {}\n\n\
         User's Full Name      whoami::realname()       {}\n\
         Username              whoami::username()       {}\n\
         Host's Fancy Name     whoami::devicename()     {}\n\
         Hostname              whoami::hostname()       {}\n\
         Platform              whoami::platform()       {}\n\
         Operating System      whoami::distro()         {}\n\
         Desktop Environment   whoami::desktop_env()    {}\n\
         ",
        env!("CARGO_PKG_VERSION"),
        whoami::realname(),
        whoami::username(),
        whoami::devicename(),
        whoami::hostname(),
        whoami::platform(),
        whoami::distro(),
        whoami::desktop_env(),
    );
}
