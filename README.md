![WhoAmI Logo]

#### [Changelog] | [Source] | [Getting Started]

[![tests](https://github.com/ardaku/whoami/actions/workflows/ci.yml/badge.svg)](https://github.com/ardaku/whoami/actions/workflows/ci.yml)
[![GitHub commit activity](https://img.shields.io/github/commit-activity/y/ardaku/whoami)](https://github.com/ardaku/whoami/)
[![GitHub contributors](https://img.shields.io/github/contributors/ardaku/whoami)](https://github.com/ardaku/whoami/graphs/contributors)  
[![Crates.io](https://img.shields.io/crates/v/whoami)](https://crates.io/crates/whoami)
[![Crates.io](https://img.shields.io/crates/d/whoami)](https://crates.io/crates/whoami)
[![Crates.io (recent)](https://img.shields.io/crates/dr/whoami)](https://crates.io/crates/whoami)  
[![Crates.io](https://img.shields.io/crates/l/whoami)](https://github.com/search?q=repo%3Aardaku%2Fwhoami+path%3A**%2FLICENSE*&type=code)
[![Docs.rs](https://docs.rs/whoami/badge.svg)](https://docs.rs/whoami/)

Retrieve the current user and environment through simple functions.

Check out the [documentation] for examples.

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
 - illumos variants (SmartOS, OmniOS, others)
 - Redox
 - [Web Assembly]
   - Fake implementation
   - Web Browser - DOM
   - WASI (Wasite, others) **untested, testing planned later**
   - Daku (Ardaku/Quantii, others) **planned later**
 - Android **planned later**
 - iOS / watchOS / tvOS **planned later**
 - Fuchsia **planned later**
 - GNU/Hurd **untested**
 - Others? (make a PR or open an issue)

## MSRV

MSRV is updated according to the [Ardaku MSRV guidelines], so it will only get
updated on minor and major version bumps.  All 2.x releases will be maintained
with bugfixes until the end of 2027 at minimum.

 - WhoAmI 2.2: Rust 1.85 MSRV (bump to get the 2024 edition)
 - WhoAmI 2.1: Rust 1.75 MSRV (bump to pull in nix, etc.)
 - WhoAmI 2.0: Rust 1.65 MSRV (support based on original 2.0 plan)
 - WhoAmI 1.x: Rust 1.40 MSRV (old MSRV policy, maintained until end of 2025 at
   minimum)

## Binary

[whome] - `whoami` command RiR (Re-written in Rust) that depends on this crate.

## Testing

The interactive testing procedure is documented in the release PR template.  The
full manual test suite is run for each change that affects multiple platforms.

## License

Copyright © 2017-2025 The WhoAmI Contributors.

Licensed under any of
 - Apache License, Version 2.0, ([LICENSE_APACHE] or
   <https://www.apache.org/licenses/LICENSE-2.0>)
 - Boost Software License, Version 1.0, ([LICENSE_BOOST] or
   <https://www.boost.org/LICENSE_1_0.txt>)
 - MIT License, ([LICENSE_MIT] or <https://mit-license.org/>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as described above, without any additional terms or conditions.

## Help

If you want help using or contributing to this library, feel free to send me an
email at <aldaronlau@gmail.com>.

[Changelog]: https://github.com/ardaku/whoami/releases
[Source]: https://github.com/ardaku/whoami
[Getting Started]: https://docs.rs/whoami#getting-started
[documentation]: https://docs.rs/whoami
[LICENSE_APACHE]: https://github.com/ardaku/whoami/blob/v2/LICENSE_APACHE
[LICENSE_MIT]: https://github.com/ardaku/whoami/blob/v2/LICENSE_MIT
[LICENSE_BOOST]: https://github.com/ardaku/whoami/blob/v2/LICENSE_BOOST
[Ardaku MSRV guidelines]: https://github.com/ardaku/.github/blob/v1/profile/MSRV.md
[WhoAmI Logo]: https://raw.githubusercontent.com/ardaku/whoami/v2/res/icon.svg
[Web Assembly]: https://github.com/ardaku/whoami/blob/v2/WASM.md
[whome]: https://crates.io/crates/whome
