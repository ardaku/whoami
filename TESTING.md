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

Install it in GNOME Boxes.



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
