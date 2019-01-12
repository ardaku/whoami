# Whoami
A library and program that retrieves the current user and environment.

## Getting Started
Using the whoami crate is super easy!  All of the exported items are simple functions with no parameters that return `String`s (with the exception of `env`, which returns an enum).  The following example shows how to use all of the functions:

```rust
use whoami;

fn main() {
    print!(
        "--------------------------------------\n\
         user's full name (user):              {}\n\
         username (username):                  {}\n\
         --------------------------------------\n\
         host's fancy name (host):             {}\n\
         hostname (hostname):                  {}\n\
         --------------------------------------\n\
         operating system (os):                {}\n\
         desktop environment (env):            {}\n\
         --------------------------------------\n\
         ",
        whoami::user(),
        whoami::username(),
        whoami::host(),
        whoami::hostname(),
        whoami::os(),
        whoami::env(),
    );
}
```

## Installing
If you clone the repo, install with `cargo install --path . --features="term" --force`.
Otherwise, install with `cargo install whoami --features="term" --force`.

## Features
* Get the user's full name
* Get the user's username
* Get the computer's hostname
* Get the computer's fancy name
* Get the computer's desktop environment
* Get the computer's OS name and version
* Works on Linux / Windows / Mac OS (for some functions)

## TODO
* Work on Mac OS / iOS / Android / Nintendo Switch / Wasm (Web Assembly) / other OS's.

## Links
* [Website](https://free.plopgrizzly.com/whoami)
* [Cargo](https://crates.io/crates/whoami)
* [Documentation](https://docs.rs/whoami)
* [Change Log](https://free.plopgrizzly.com/whoami/changelog)
* [Contributing](https://plopgrizzly.com/contributing)
* [Code of Conduct](https://free.plopgrizzly.com/whoami/codeofconduct)

---

[![Plop Grizzly](https://plopgrizzly.com/images/logo-bar.png)](https://plopgrizzly.com)
