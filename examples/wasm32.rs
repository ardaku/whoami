#[cfg(not(target_arch = "wasm32"))]
fn main() {}

#[cfg(target_arch = "wasm32")]
fn main() {
    use stdweb;
    use whoami;

    stdweb::initialize();

    let a = format!(
        "----------------------------------------------------------------\n\
         user's full name (user): {}\n\
         username (username): {}\n\
         ----------------------------------------------------------------\n\
         host's fancy name (host): {}\n\
         hostname (hostname): {}\n\
         ----------------------------------------------------------------\n\
         platform (platform): {}\n\
         operating system (os): {}\n\
         desktop environment (env): {}\n\
         ----------------------------------------------------------------\n\
         ",
        whoami::user(),
        whoami::username(),
        whoami::host(),
        whoami::hostname(),
        whoami::platform(),
        whoami::os(),
        whoami::env(),
    );

    stdweb::web::alert(&a);
}
