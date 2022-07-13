![WhoAmI Logo](https://raw.githubusercontent.com/ardaku/whoami/stable/res/icon.svg)

#### [Changelog][3] | [Source][4] | [Getting Started][5]

[![tests](https://github.com/ardaku/whoami/workflows/tests/badge.svg)][2]
[![docs](https://docs.rs/whoami/badge.svg)][0]
[![crates.io](https://img.shields.io/crates/v/whoami.svg)][1]

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

### Supported Platforms
WhoAmI targets all platforms that can run Rust, including:
 - Linux
 - Windows
 - Mac OS
 - Web Assembly
 - BSD
 - Android (may partially or fully work, but untested) **planned later**
 - iOS / various game consoles **planned later**
 - Redox **planned later**
 - Fuchsia **planned later**
 - Others? (make a PR)

## MSRV
WhoAmI targets Rust 1.40.0 stable and later

## Binary
[whome](https://crates.io/crates/whome): `whoami` command RiR (Re-written in
Rust) that depends on this crate.

## License
Licensed under any of
 - Apache License, Version 2.0, ([LICENSE_APACHE_2_0.txt][7]
   or [https://www.apache.org/licenses/LICENSE-2.0][8])
 - Boost Software License, Version 1.0, ([LICENSE_BOOST_1_0.txt][11]
   or [https://www.boost.org/LICENSE_1_0.txt][12])
 - MIT License, ([LICENSE_MIT.txt][9] or [https://mit-license.org/][10])

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
[7]: https://github.com/ardaku/whoami/blob/stable/LICENSE_APACHE_2_0.txt
[8]: https://www.apache.org/licenses/LICENSE-2.0
[9]: https://github.com/ardaku/whoami/blob/stable/LICENSE_MIT.txt
[10]: https://mit-license.org/
[11]: https://github.com/ardaku/whoami/blob/stable/LICENSE_BOOST_1_0.txt
[12]: https://www.boost.org/LICENSE_1_0.txt
[13]: mailto:aldaronlau@gmail.com
