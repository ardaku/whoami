# Testing

This file outlines the regression testing plan for all platforms.

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

### Build Redox

```shell
make all
```

### Run Redox

```shell
make qemu
```
