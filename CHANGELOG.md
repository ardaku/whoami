# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://code.plopgrizzly.com/semver/).

## [Unreleased]

## [0.5.3] - 2019-07-17
### Changed
- Now uses a more modern Rust coding style (replace `::std::` with `std::`).
- Now uses a more modern Rust coding style with `mem::MaybeUninit`.
- `impl Display` for desktop environment now uses proper capitalization.
- Don't depend on `libc` anymore.

## [0.5.2] - 2019-05-12
### Fixed
- Fixed more broken links!

## [0.5.1] - 2019-05-12
### Fixed
- Clippy lint warning: change `expect(&format!("…"))` to `expect("…")` for optimization in 2 cases.
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
- `env` on Ubuntu now returns DesktopEnv::Ubuntu instead of DesktopEnv::Other("UBUNTU")
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
