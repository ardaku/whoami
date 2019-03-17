## 0.5.0
* Added `Platform` enum with corresponding `platform()` function.
* Added `Dive`, `Fuchsia`, and `Redox` to `DesktopEnv` enum.
* Started using `const fn` for some functions.

## 0.4.1
* Fixed README errors.

## 0.4.0
* Works on Mac now
* `env` on Ubuntu now returns DesktopEnv::Ubuntu instead of DesktopEnv::Other("UBUNTU")
* Split off the binary into `whome` (who me?) crate

## 0.3.0
* Fix typo for uknown -> unknown.
* Added more fallbacks.
* Rename realname -> user
* Rename computer -> host

## 0.2.4
* Works now on platforms that use u8 instead of i8 for chars (like ARM).

## 0.2.3
* Fix trailing newline on Windows.

## 0.2.2
* Fix Typo.

## 0.2.1
* Fix undefined behavior on Linux

## 0.2
* Add Windows support.

## 0.1
* Initial release.
