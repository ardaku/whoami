![WhoAmI Logo](https://raw.githubusercontent.com/ardaku/whoami/stable/res/icon.svg)

#### [Changelog][3] | [Source][4] | [Getting Started][5]

[![tests](https://github.com/ardaku/whoami/actions/workflows/ci.yml/badge.svg)](https://github.com/ardaku/whoami/actions/workflows/ci.yml)
[![GitHub commit activity](https://img.shields.io/github/commit-activity/y/ardaku/whoami)](https://github.com/ardaku/whoami/)
[![GitHub contributors](https://img.shields.io/github/contributors/ardaku/whoami)](https://github.com/ardaku/whoami/graphs/contributors)  
[![Crates.io](https://img.shields.io/crates/v/whoami)](https://crates.io/crates/whoami)
[![Crates.io](https://img.shields.io/crates/d/whoami)](https://crates.io/crates/whoami)
[![Crates.io (recent)](https://img.shields.io/crates/dr/whoami)](https://crates.io/crates/whoami)  
[![Crates.io](https://img.shields.io/crates/l/whoami)](https://github.com/ardaku/whoami/search?l=Text&q=license)
[![Docs.rs](https://docs.rs/whoami/badge.svg)](https://docs.rs/whoami/)

Retrieve the current user and environment through simple functions.

Check out the [documentation][0] for examples.

### Features
 - Get the user's full name
 - Get the user's username
 - Get the user's preferred language(s)
 - Get the devices's hostname
 - Get the devices's "pretty" or "fancy" name
 - Get the devices's desktop environment
 - Get the devices's OS name and version
 - Get the devices's platform name
 - Get the devices's CPU architecture and its width

### Supported Platforms
WhoAmI targets all platforms that can run Rust, including:
 - Linux
 - Windows
 - Mac OS
 - BSD variants (FreeBSD, others)
 - [Web Assembly](https://github.com/ardaku/whoami/blob/stable/WASM.md)
   - Mock implementation
   - Web Browser - DOM
   - WASI (Wasite/Quantii, others) **mock implementation, full implementation planned later**
   - Daku (Ardaku/Quantii, others) **mock implementation, full implementation planned later**
 - Illumos variants (SmartOS, OmniOS, others) **may partially or fully work - but untested**
 - Android **may partially or fully work - but untested, planned later**
 - iOS **planned later**
 - Redox **planned later**
 - Fuchsia **planned later**
 - Various game consoles **planned later**
 - Others? (make a PR or open an issue)

## MSRV
WhoAmI 1.x.y targets Rust 1.40.0 stable and later, and the 1.x.y track will
be maintained at least until the release of the Rust 2024 edition.

The MSRV will only be updated on major version bumps, and version 2.0.0 will
target Rust 1.65.0 and later to make use of the `let else` syntax.

## Binary
[whome](https://crates.io/crates/whome): `whoami` command RiR (Re-written in
Rust) that depends on this crate.

## License
Copyright © 2017-2023 The WhoAmI Contributors.

Licensed under any of
 - Apache License, Version 2.0, ([LICENSE_APACHE][7]
   or [https://www.apache.org/licenses/LICENSE-2.0][8])
 - Boost Software License, Version 1.0, ([LICENSE_BOOST][11]
   or [https://www.boost.org/LICENSE_1_0.txt][12])
 - MIT License, ([LICENSE_MIT][9] or [https://mit-license.org/][10])

at your option.

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as described above, without any additional terms or conditions.

## Help
If you want help using or contributing to this library, feel free to send me an
email at [aldaronlau@gmail.com][13].

[0]: https://docs.rs/whoami
[1]: https://crates.io/crates/whoami
[2]: https://github.com/ardaku/whoami/actions?query=workflow%3Atests
[3]: https://github.com/ardaku/whoami/blob/stable/CHANGELOG.md
[4]: https://github.com/ardaku/whoami/
[5]: https://docs.rs/whoami#getting-started
[6]: https://aldaronlau.com/
[7]: https://github.com/ardaku/whoami/blob/stable/LICENSE_APACHE
[8]: https://www.apache.org/licenses/LICENSE-2.0
[9]: https://github.com/ardaku/whoami/blob/stable/LICENSE_MIT
[10]: https://mit-license.org/
[11]: https://github.com/ardaku/whoami/blob/stable/LICENSE_BOOST
[12]: https://www.boost.org/LICENSE_1_0.txt
[13]: mailto:aldaronlau@gmail.com
