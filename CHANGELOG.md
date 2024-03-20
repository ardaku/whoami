# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to
[Semantic Versioning].

## [1.5.2] - 2024-03-19

### Changed

 - Update redox\_syscall to 0.5

## [1.5.1] - 2024-03-09

### Fixed

 - Broken link in docs

## [1.5.0] - 2024-03-03

### Added

 - WASI support
 - Redox support
 - Fallible functions
   - `whoami::fallible::devicename()`
   - `whoami::fallible::devicename_os()`
   - `whoami::fallible::distro()`
   - `whoami::fallible::hostname()` - notably doesn't normalize to lowercase
   - `whoami::fallible::realname()`
   - `whoami::fallible::realname_os()`
   - `whoami::fallible::username()`
   - `whoami::fallible::username_os()`
 - `whoami::Language`
 - `whoami::Country`
 - `whoami::langs()`
 - `whoami::fallible::account()`
 - `whoami::fallible::account_os()`
 - `whoami::DesktopEnv::is_gtk()`
 - `whoami::DesktopEnv::is_kde()`

### Removed

 - Generated device names that infer casing based on the hostname when the
   device name is not available - now returns the hostname unchanged
 - Partial (potentially unsound) support for Android, iOS, watchOS, tvOS,
   Fuchsia, Haiku, Solaris, and a few others.  These targets now use the "fake"
   implementation.

### Changed

 - Deprecated `whoami::distro_os()`
 - Deprecated `whoami::hostname()`
 - Deprecated `whoami::hostname_os()`
 - Deprecated `whoami::lang()`
 - illumos and Redox are no longer untested targets
 - Documented that illumos and Redox have a higher MSRV (Rust 1.65) than other
   targets
 - Display implementation on `Platform::Illumos` now displays in lowercase:
   illumos

### Fixed

 - Removed some unnecessary allocations
 - Rare and nearly impossible cases of undefined behavior
 - Better handling of UTF-8 non-conformant strings
 - Multiple instances of undefined behavior on illumos

## [1.4.1] - 2023-06-25

### Fixed

 - License files not being included in package on crates.io

## [1.4.0] - 2023-03-12

### Added

 - Support for illumos

## [1.3.0] - 2022-12-28

### Added

 - `Arch` enum listing CPU architectures
 - `Width` enum listing CPU architecture address widths
 - `arch()` function which returns an `Arch` type representing a CPU
   architecture
 - `Arch::width()` method which returns the address width of a CPU architecture
 - *`web`* feature (enabled by default).  Disabling this feature allows you to
   use wasm32-unknown-unknown with whoami outside of the browser with a fake
   implementation.
 - Officially support compiling to WASI or Daku WebAssembly platforms;
   functionality not supported yet.
 - Convenience `Result` type alias.

### Changed

 - Modernized and cleaned up code style

### Fixed

 - Handling of `lang()` when `$LANG` environment variable on unix set to "C",
   causing duplicated iterator elements `["C", "C"]`; now produces `["en-US"]`.
 - WhoAmI reporting "Safari" when running in Chrome/Chromium browser
 - WhoAmI reporting "Edg" when running in Edge browser (now reports "Edge")
 - WhoAmI reporting "OPR" when running in Opera browser (now reports "Opera")
 - WhoAmI reporting "Mozilla" when running in GNOME web browser (now reports
   "GNOME Web")

## [1.2.3] - 2022-09-12

### Fixed

 - WebAssembly target requiring older versions of dependencies

## [1.2.2] - 2022-09-05

### Added
 - More details to the method documentation

### Changed

 - WhoAmI now has an official MSRV of Rust 1.40.0
 - `whoami::hostname()` is now normalized to lowercase

## [1.2.1] - 2021-11-24

### Fixed
 - The Windows GNU target support being broken

## [1.2.0] - 2021-11-06

### Added
 - Derives for `Clone`, `PartialEq` and `Eq` on `DesktopEnv` and `Platform`

### Changed
 - Lower MSRV for Windows

## [1.1.5] - 2021-09-28

### Fixed

 - Segfault that occurs when passwd entry missing in docker

## [1.1.4] - 2021-09-26

### Changed

 - There are no longer any known panics within the code, all possible panics
   with whoami are now considered bugs.
 - If any of the primary functions return an empty string, whoami should now
   return "Unknown" or"unknown", or for `hostname()`, "localhost".

### Fixed

 - Panicking in situations where certain files don't exist / OS functions fail.

## [1.1.3] - 2021-08-17

### Fixed

 - Circumstance where whoami points to invalid memory on Linux.

## [1.1.2] - 2021-04-03

### Fixed

 - Not parsing the gecos field properly on unix systems (affects the
   `realname()` and `realname_os()` functions; they will no longer return
   extraneous commas on some systems).

## [1.1.1] - 2021-03-13

### Fixed

 - Not compiling on target `x86_64-pc-windows-gnu`.

## [1.1.0] - 2021-01-17

### Added

 - `lang()` function which returns an iterator over user's preferred language(s)

## [1.0.3] - 2020-12-31

### Fixed

 - Link to logo in documentation.

## [1.0.2] - 2020-12-31

### Changed

 - `distro()` on Windows now returns more detailed version.

## [1.0.1] - 2020-12-16

### Added

 - Official support for BSD and variants

### Fixed

 - `platform()` will now return `Platform::Bsd` when running BSD.
 - Misaligned address error on FreeBSD when calling `username()`.

## [1.0.0] - 2020-11-23

### Removed
 - `Platform::Dive` and `DesktopEnv::Dive`, as that was an OS idea not a real OS
 - Explicit support for `stdweb`, now built on `web-sys`/`wasm-bindgen`.

## [0.9.0] - 2020-06-24

### Added

 - `stdweb` and `wasm-bindgen` support
 - Versions of `-> String` functions that return `OsString`s:
   - `devicename_os()`
   - `distro_os()`
   - `hostname_os()`
   - `realname_os()`
   - `username_os()`

### Changed

 - Renamed `DesktopEnv::Mac` to `DesktopEnv::Aqua`
 - Renamed `DesktopEnv::Wasm` to `DesktopEnv::WebBrowser`
 - Renamed `DesktopEnv::Redox` to `DesktopEnv::Orbital`
 - Renamed `DesktopEnv::Fuchsia` to `DesktopEnv::Ermine`
 - Renamed `Platform::FreeBsd` to `Platform::Bsd`
 - Renamed `env()` to `desktop_env()`
 - Renamed `host()` to `devicename()`
 - Renamed `os()` to `distro()`
 - Renamed `user()` to `realname()`

### Fixed

 - Inconsistencies on Windows
 - MacOS running commands instead of using native APIs (this results in speed
   improvements on MacOS)
 - One of the Linux functions also using commands instead of native APIs (faster)

### Contributors

Thanks to everyone who contributed to make this version of whoami possible!

- [AldaronLau](https://github.com/AldaronLau)
- [Vlad-Shcherbina](https://github.com/Vlad-Shcherbina)

## [0.8.2] - 2020-06-11

### Changed

 - Windows `host()` and `hostname()` now behave like they do on Linux and MacOS

### Fixed

 - Windows FFI Undefined Behavior because of not checking for errors
 - Cross-compiling from Linux to Windows not working

## [0.8.1] - 2020-02-22

### Fixed

 - Remove unnecessary use of `to_mut()` on `Cow`s returned from
   `String::from_utf8_lossy()`.

## [0.8.0] - 2020-02-21

### Added

 - Detection for KDE desktop environment.

### Changed

 - Unknown desktop environment may now contain lowercase characters.

### Fixed

 - No longer unwraps in any place where bad data from the OS could cause
   a panic.

## [0.7.0] - 2019-12-21

### Removed

 - `stdweb` dependency when targetting web assembly.

### Changed

 - All public enums now have the attribute `#[non_exhaustive]` and derive
   `Debug`.

### Fixed

 - Some out-of-date documentation

## [0.6.0] - 2019-10-25

### Added

 - Web Assembly support.

### Removed

 - `Platform::Web` variant of enum, use `env()` if you need to.

### Changed

 - `platform()` is no longer a const fn (needed for wasm platform
   detection).

## [0.5.3] - 2019-07-18

### Changed

 - Now uses a more modern Rust coding style (replace `::std::` with `std::`).
 - Now uses a more modern Rust coding style with `mem::MaybeUninit`.
 - `impl Display` for desktop environment now uses proper capitalization.
 - Don't depend on `libc` anymore.

### Fixed

 - Fancy Names not working on Windows
   - `user()` now uses Windows Display Name on Windows rather than the username.
   - `host()` now uses Windows Name DNS Fully Qualified rather than the
     hostname.

## [0.5.2] - 2019-05-12

### Fixed

 - Fixed more broken links!

## [0.5.1] - 2019-05-12

### Fixed

 - Clippy lint warning: change `expect(&format!("…"))` to `expect("…")` for
   optimization in 2 cases.
 - Fixed broken links

## [0.5.0] - 2019-03-17

### Added

 - `Platform` enum with corresponding `platform()` function.
 - `Dive`, `Fuchsia`, and `Redox` to `DesktopEnv` enum.

### Changed

 - Started using `const fn` for some functions.

## [0.4.1] - 2019-01-12

### Fixed

 - Fixed README errors.

## [0.4.0] - 2019-01-12

### Added

 - MacOS support.

### Changed

 - `env` on Ubuntu now returns `DesktopEnv::Ubuntu` instead of
   `DesktopEnv::Other("UBUNTU")`
 - Split off the binary into `whome` (who me?) crate

## [0.3.0] - 2019-01-04

### Added

 - Added more fallbacks.

### Changed

 - Rename realname -> user
 - Rename computer -> host

### Fixed

 - Fix typo for uknown -> unknown.

## [0.2.4] - 2018-12-04

### Fixed

 - Works now on platforms that use u8 instead of i8 for chars (like ARM).

## [0.2.3] - 2018-11-26

### Fixed

 - Trailing newline on Windows.

## [0.2.2] - 2018-06-02

### Fixed

 - Typo in markdown.

## [0.2.1] - 2018-06-02

### Fixed

 - Undefined behavior on Linux.

## [0.2.0] - 2017-12-28

### Added

 - Windows support.

## [0.1.1] - 2017-08-04

### Fixed

 - Something in the markdown.

## [0.1.0] - 2017-08-04

### Added

 - Published to crates.io.

[Keep a Changelog]: https://keepachangelog.com/en/1.0.0/
[Semantic Versioning]: https://github.com/AldaronLau/semver/
