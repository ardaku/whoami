# Testing

This file outlines the regression testing plan for all platforms.

## Linux / Fedora Silverblue

## Linux / Ubuntu

## Windows

## MacOS

## FreeBSD

## Illumos

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
WhoAmI 1.5.0-pre.0

User's Language        whoami::langs():               
User's Name            whoami::realname():            Unknown
User's Username        whoami::username():            unknown
Device's Pretty Name   whoami::devicename():          tribblix
Device's Hostname      whoami::fallible::hostname():  tribblix
Device's Platform      whoami::platform():            Illumos
Device's OS Distro     whoami::distro():              Tribblix
Device's Desktop Env.  whoami::desktop_env():         Unknown: Unknown
Device's CPU Arch      whoami::arch():                Unknown: 
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
