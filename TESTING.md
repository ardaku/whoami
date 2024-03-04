# Testing

This file outlines the regression testing plan for all platforms.

## Linux / Fedora Silverblue

Testing is done on Fedora Silverblue 39.

Open a terminal (outside of toolbox), and:

```rust
cargo run --example whoami-demo
cargo run --example whoami-demo --release
cargo run --example os-strings
cargo run --example os-strings --release
```

Expect to see something like:

```console
WhoAmI 1.5.0

User's Language        whoami::langs():               en/US
User's Name            whoami::realname():            Jeron Lau
User's Username        whoami::username():            jeron
User's Username        whoami::fallible::account():   jeron
Device's Pretty Name   whoami::devicename():          Zetêy
Device's Hostname      whoami::fallible::hostname():  zetey
Device's Platform      whoami::platform():            Linux
Device's OS Distro     whoami::distro():              Fedora Linux 39.20240202.0 (Silverblue)
Device's Desktop Env.  whoami::desktop_env():         Gnome
Device's CPU Arch      whoami::arch():                x86_64
```

```console
WhoAmI 1.5.0

User's Language        whoami::langs():                 "en/US"
User's Name            whoami::realname_os():           "Jeron Lau"
User's Username        whoami::username_os():           "jeron"
User's Account         whoami::fallible::account_os():  "jeron"
Device's Pretty Name   whoami::devicename_os():         "Zetêy"
Device's Hostname      whoami::fallible::hostname():    "zetey"
Device's Platform      whoami::platform():              Linux
Device's OS Distro     whoami::distro():                "Fedora Linux 39.20240202.0 (Silverblue)"
Device's Desktop Env.  whoami::desktop_env():           Gnome
Device's CPU Arch      whoami::arch():                  X64
```

Now, `toolbox enter`, and do the same.  Expecting something like:

```console
WhoAmI 1.5.0

User's Language        whoami::langs():               en/US
User's Name            whoami::realname():            Jeron Lau
User's Username        whoami::username():            jeron
User's Username        whoami::fallible::account():   jeron
Device's Pretty Name   whoami::devicename():          toolbox
Device's Hostname      whoami::fallible::hostname():  toolbox
Device's Platform      whoami::platform():            Linux
Device's OS Distro     whoami::distro():              Fedora Linux 39 (Container Image)
Device's Desktop Env.  whoami::desktop_env():         Gnome
Device's CPU Arch      whoami::arch():                x86_64
```

```console
WhoAmI 1.5.0

User's Language        whoami::langs():                 "en/US"
User's Name            whoami::realname_os():           "Jeron Lau"
User's Username        whoami::username_os():           "jeron"
User's Account         whoami::fallible::account_os():  "jeron"
Device's Pretty Name   whoami::devicename_os():         "toolbox"
Device's Hostname      whoami::fallible::hostname():    "toolbox"
Device's Platform      whoami::platform():              Linux
Device's OS Distro     whoami::distro():                "Fedora Linux 39 (Container Image)"
Device's Desktop Env.  whoami::desktop_env():           Gnome
Device's CPU Arch      whoami::arch():                  X64
```

## Linux / Ubuntu

Testing is done on Ubuntu 23.10 (virtualized on Fedora Silverblue):

https://ubuntu.com/download/desktop

Install from file within GNOME boxes (keep all defaults).

In Ubuntu Installer, do default installation.

```shell
sudo apt install git curl gcc tig # y
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh # 1
source "$HOME/.cargo/env"
```

Clone whoami, open a terminal, and run:

```rust
cargo run --example whoami-demo
cargo run --example whoami-demo --release
```

Expect to see something like:

```console
WhoAmI 1.5.0

User's Language        whoami::langs():               en/US
User's Name            whoami::realname():            Jeron Lau
User's Username        whoami::username():            aldaron
User's Username        whoami::fallible::account():   aldaron
Device's Pretty Name   whoami::devicename():          ubuntu-box
Device's Hostname      whoami::fallible::hostname():  ubuntu-box
Device's Platform      whoami::platform():            Linux
Device's OS Distro     whoami::distro():              Ubuntu 23.10
Device's Desktop Env.  whoami::desktop_env():         Ubuntu
Device's CPU Arch      whoami::arch():                x86_64
```

## Windows

Testing is done on Windows 10

Clone whoami, open Git BASH, and run:

```rust
cargo run --example whoami-demo
cargo run --example whoami-demo --release
```

Expect to see something like:

```console
WhoAmI 1.5.0

User's Language        whoami::langs():               en/US
User's Name            whoami::realname():            Aldaron Lau
User's Username        whoami::username():            Aldaron Lau
User's Username        whoami::fallible::account():   Aldaron Lau
Device's Pretty Name   whoami::devicename():          Helpy-Witch
Device's Hostname      whoami::fallible::hostname():  HELPY-WITCH
Device's Platform      whoami::platform():            Windows
Device's OS Distro     whoami::distro():              Windows 10.0.19044 (Workstation)
Device's Desktop Env.  whoami::desktop_env():         Windows
Device's CPU Arch      whoami::arch():                x86_64
```

## MacOS

Testing is done on macOS Catalina.

Clone whoami, and run:

```rust
cargo run --example whoami-demo
cargo run --example whoami-demo --release
```

Expect to see something like:

```console
WhoAmI 1.5.0

User's Language        whoami::langs():               en/US
User's Name            whoami::realname():            Aldaron Lau
User's Username        whoami::username():            aldaronlau
User's Username        whoami::fallible::account():   aldaronlau
Device's Pretty Name   whoami::devicename():          Aldaron’s MacBook Air
Device's Hostname      whoami::fallible::hostname():  Aldarons-MacBook-Air.local
Device's Platform      whoami::platform():            Mac OS
Device's OS Distro     whoami::distro():              Mac OS X 10.15.7
Device's Desktop Env.  whoami::desktop_env():         Aqua
Device's CPU Arch      whoami::arch():                x86_64
```

## BSD

Testing is done on FreeBSD (virtualized on Fedora Silverblue):

Download from within GNOME Boxes.

Set 4 GiB memory, and 20 GiB Storage limit

Go through the installation process, keeping all defaults.

### Install packages

Log in as root

```shell
pkg install git
```

Log in as you

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh # 1
. "$HOME/.cargo/env"
git clone https://github.com/ardaku/whoami.git
cd whoami
# run both debug and release
cargo run --example whoami-demo
cargo run --example whoami-demo --release
```

Expect to see something like:

```console
WhoAmI 1.5.0

User's Language        whoami::langs():               
User's Name            whoami::realname():            Aldaron Lau
User's Username        whoami::username():            aldaron
User's Username        whoami::fallible::account():   aldaron
Device's Pretty Name   whoami::devicename():          bsdtime
Device's Hostname      whoami::fallible::hostname():  bsdtime
Device's Platform      whoami::platform():            BSD
Device's OS Distro     whoami::distro():              FreeBSD 14.0-RELEASE
Device's Desktop Env.  whoami::desktop_env():         Unknown: Unknown
Device's CPU Arch      whoami::arch():                x86_64
```

## illumos

Testing is done on Tribblix (virtualized on Fedora Silverblue):

<http://www.tribblix.org/download.html>

Download the 64-bit x86/x64 standard image.

Install it in GNOME Boxes (select operating system OpenIndiana Hipster).

Set 4 GiB memory, and 16 GiB Storage limit

Login as `jack` (password `jack`)

```shell
su - root # password `tribblix`
format # 0, quit
./live_install -G c1t0d0 develop # replace c1t0d0 with disk
reboot -p
```

Login as `jack` (password `jack`)

Now, install Rust (use bash instead of sh, sh doesn't work)

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash # 1
source "$HOME/.cargo/env"
```

### Testing

```shell
git clone https://github.com/ardaku/whoami.git
cd whoami
# run both debug and release
cargo run --example whoami-demo
cargo run --example whoami-demo --release
```

Expected output is

```console
WhoAmI 1.5.0

User's Language        whoami::langs():               ??
User's Name            whoami::realname():            Tribblix Jack
User's Username        whoami::username():            jack
Device's Pretty Name   whoami::devicename():          tribblix
Device's Hostname      whoami::fallible::hostname():  tribblix
Device's Platform      whoami::platform():            illumos
Device's OS Distro     whoami::distro():              Tribblix
Device's Desktop Env.  whoami::desktop_env():         Unknown: Unknown
Device's CPU Arch      whoami::arch():                Unknown: i86pc
```

## Redox

<https://doc.redox-os.org/book/ch08-01-advanced-build.html#understanding-cross-compilation-for-redox>

Tested on Fedora Silverblue 39

### Update Rust Nightly and Stable

```shell
rustup update nightly stable
rustup target add --toolchain stable x86_64-unknown-redox
```

### Install pre-requisites

```shell
sudo dnf install git file autoconf vim bison flex genisoimage gperf glibc-devel.i686 expat expat-devel fuse-devel fuse3-devel gmp-devel perl-HTML-Parser libpng-devel libtool libjpeg-turbo-devel libvorbis-devel SDL2_ttf-devel mesa-libOSMesa-devel m4 nasm po4a syslinux texinfo sdl12-compat-devel ninja-build meson python3-mako make gcc gcc-c++ openssl patch automake perl-Pod-Html perl-FindBin gperf curl gettext-devel perl-Pod-Xhtml pkgconf-pkg-config cmake cbindgen just qemu doxygen 'perl(ExtUtils::MakeMaker)'

cargo install --locked --force --version 0.1.1 cargo-config
```

### Get redox source

```shell
mkdir -p build/
cd build/
git clone https://gitlab.redox-os.org/redox-os/redox.git --origin upstream --recursive
```

### Create our demo recipe

Make sure whome is updated to the whoami testing branch.

```shell
mkdir -p build/redox/cookbook/recipes/demos/whome/
cp recipe.toml build/redox/cookbook/recipes/demos/whome/
cp build/redox/config/desktop.toml config/x86_64/ardaku.toml
```

In `config/x86_64/ardaku.toml`, under `[packages]`:

```toml
whome = {}
```

### Build Redox

This takes a while

```shell
make all
```

or 

```shell
make rebuild
```

### Run Redox

```shell
make qemu
```

### Test it

Verify you are on the new version

```shell
whome --version
```

Default settings should output:

```console
realname:     user
username:     user
devicename:   redox
hostname:     redox
distro:       Redox OS 0.8.0
desktop_env:  Orbital
platform:     Redox
arch:         Unknown: x86_64
```

## Web

Build web example and start webserver on Fedora Silverblue.

Check the web console in Firefox:

```console
User's Name            whoami::realname():            Anonymous
User's Username        whoami::username():            anonymous
User's Language        whoami::langs():               en/US, en
Device's Pretty Name   whoami::devicename():          Firefox 122.0
Device's Hostname      whoami::fallible::hostname():  localhost
Device's Platform      whoami::platform():            Linux
Device's OS Distro     whoami::distro():              Unknown Linux
Device's Desktop Env.  whoami::desktop_env():         Web Browser
Device's CPU Arch      whoami::arch():                wasm32
```

Check the web console in Opera:

```console
User's Name            whoami::realname():            Anonymous
User's Username        whoami::username():            anonymous
User's Language        whoami::langs():               en/US, en
Device's Pretty Name   whoami::devicename():          Opera 107.0.0.0
Device's Hostname      whoami::fallible::hostname():  localhost
Device's Platform      whoami::platform():            Linux
Device's OS Distro     whoami::distro():              Unknown Linux
Device's Desktop Env.  whoami::desktop_env():         Web Browser
Device's CPU Arch      whoami::arch():                wasm32
```

Check the web console in Chrome:

```console
User's Name            whoami::realname():            Anonymous
User's Username        whoami::username():            anonymous
User's Language        whoami::langs():               en/US, en
Device's Pretty Name   whoami::devicename():          Chrome 122.0.0.0
Device's Hostname      whoami::fallible::hostname():  localhost
Device's Platform      whoami::platform():            Linux
Device's OS Distro     whoami::distro():              Unknown Linux
Device's Desktop Env.  whoami::desktop_env():         Web Browser
Device's CPU Arch      whoami::arch():                wasm32
```

Check the web console in Ungoogled Chromium:

```console
User's Name            whoami::realname():            Anonymous
User's Username        whoami::username():            anonymous
User's Language        whoami::langs():               en/US, en
Device's Pretty Name   whoami::devicename():          Chrome 122.0.0.0
Device's Hostname      whoami::fallible::hostname():  localhost
Device's Platform      whoami::platform():            Linux
Device's OS Distro     whoami::distro():              Unknown Linux
Device's Desktop Env.  whoami::desktop_env():         Web Browser
Device's CPU Arch      whoami::arch():                wasm32
```

Check the web console in GNOME Web (Epiphany):

```console
User's Name            whoami::realname():            Anonymous
User's Username        whoami::username():            anonymous
User's Language        whoami::langs():               en/US
Device's Pretty Name   whoami::devicename():          GNOME Web
Device's Hostname      whoami::fallible::hostname():  localhost
Device's Platform      whoami::platform():            Linux
Device's OS Distro     whoami::distro():              Unknown Linux
Device's Desktop Env.  whoami::desktop_env():         Web Browser
Device's CPU Arch      whoami::arch():                wasm32
```
