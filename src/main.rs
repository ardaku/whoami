fn main() {
    print!(
        "whoami {} ({}, {})\n\n\
         user's full name      whoami::user()       {}\n\
         username              whoami::username()   {}\n\
         host's fancy name     whoami::host()       {}\n\
         hostname              whoami::hostname()   {}\n\
         platform              whoami::platform()   {}\n\
         operating system      whoami::os()         {}\n\
         desktop environment   whoami::env()        {}\n\
         ",
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
        env!("CARGO_PKG_HOMEPAGE"),
        whoami::user(),
        whoami::username(),
        whoami::host(),
        whoami::hostname(),
        whoami::platform(),
        whoami::os(),
        whoami::env(),
    );
}
