![whoami](https://libcala.github.io/whoami/icon.svg)

[![Build Status](https://travis-ci.com/libcala/whoami.svg?branch=master)](https://travis-ci.com/libcala/whoami)

# WhoAmI
Retrieve the current user and environment.

## Getting Started
Using the whoami crate is super easy!  All of the exported items are simple functions with no parameters that return either a `String` or enum.  The following example shows how to use all of the functions:

```rust
fn main() {
    print!(
        "--------------------------------------------------------------------------------\n\
         user's full name (user):              {}\n\
         username (username):                  {}\n\
         --------------------------------------------------------------------------------\n\
         host's fancy name (host):             {}\n\
         hostname (hostname):                  {}\n\
         --------------------------------------------------------------------------------\n\
         platform (platform):                  {}\n\
         operating system (os):                {}\n\
         desktop environment (env):            {}\n\
         --------------------------------------------------------------------------------\n\
         ",
        whoami::user(),
        whoami::username(),
        whoami::host(),
        whoami::hostname(),
        whoami::platform(),
        whoami::os(),
        whoami::env(),
    );
}
```

## Features
* Get the user's full name
* Get the user's username
* Get the computer's hostname
* Get the computer's fancy name
* Get the computer's desktop environment
* Get the computer's OS name and version
* Get the computer's platform name
* Works on Linux, Windows and Mac OS

## Binary
[whome](https://crates.io/crates/whome): replacement of the `whoami` command that depends on this crate.

## TODO
* Support iOS / Android / Nintendo Switch (and other consoles) / Wasm (Web Assembly) / other OS's.

## Links
* [Website](https://libcala.github.io/whoami)
* [Cargo](https://crates.io/crates/whoami)
* [Documentation](https://docs.rs/whoami)
* [Change Log](https://libcala.github.io/whoami/CHANGELOG)
* [Contributors](https://libcala.github.io/whoami/CONTRIBUTORS)
* [Code of Conduct](https://libcala.github.io/whoami/CODEOFCONDUCT)
