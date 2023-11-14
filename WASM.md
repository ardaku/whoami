# WhoAmI WebAssembly Documentation

## Web (Browser)
By default, when WhoAmI is compiled for Wasm32 unknown (neither -wasi or -daku),
WhoAmI links to web-sys and defaults values to browser information:

 - `realname()`: "Anonymous"
 - `username()`: "anonymous"
 - `lang()`: Browser preferred language list
 - `devicename()`: Browser name (Example: "Firefox 110.0")
 - `hostname()`: "localhost"
 - `platform()`: Host operating system by view of browser (Example: "Linux")
 - `distro()`: Host distro by view of browser (Example "Unknown Linux")
 - `desktop_env()`: "Web Browser"
 - `arch()`: "wasm32"

## Mock
If you compile WhoAmI with `default-features = false`, WhoAmI will not bind to
web-sys, and will instead return these mock values:

 - `realname()`: "Anonymous"
 - `username()`: "anonymous"
 - `lang()`: "en-US"
 - `devicename()`: "Unknown"
 - `hostname()`: "localhost"
 - `platform()`: "Unknown"
 - `distro()`: "Emulated"
 - `desktop_env()`: "Unknown WebAssembly"
 - `arch()`: "wasm32"

## Wasi (Wasite)
Building WhoAmI targeting Wasi will assume the
[wasite](https://ardaku.org/wasite/env_vars.html) environment variables are set,
as Wasi alone does not currently support the functionality WhoAmI requires.

 - `realname()`: `$USER`
 - `username()`: `$USER`
 - `lang()`: `$LANGS`
 - `devicename()`: `$NAME`
 - `hostname()`: `$HOSTNAME`
 - `platform()`: "Wasite"
 - `distro()`: "Unknown wasi"
 - `desktop_env()`: "Unknown wasite"
 - `arch()`: "wasm32"

## Daku (Quantii, other Ardaku environments)
WhoAmi will depend on currently unstable portals in the
[Daku](https://ardaku.org/daku/) specification.
