fn main() {
    print!(
        "whoami {}\n\n\
         User's Full Name      whoami::user()       {}\n\
         Username              whoami::username()   {}\n\
         Host's Fancy Name     whoami::host()       {}\n\
         Hostname              whoami::hostname()   {}\n\
         Platform              whoami::platform()   {}\n\
         Operating System      whoami::os()         {}\n\
         Desktop Environment   whoami::env()        {}\n\
         ",
        env!("CARGO_PKG_VERSION"),
        whoami::user(),
        whoami::username(),
        whoami::host(),
        whoami::hostname(),
        whoami::platform(),
        whoami::os(),
        whoami::env(),
    );
}
