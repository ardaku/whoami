# Release vX.Y.Z

<!--- Check CONTRIBUTING.md for more information-->

## Regression testing plan for all platforms

Set passwords / hostnames to "test" when prompted.

 - [ ] Testing complete on Fedora Silverblue 43.
    <details><summary>Linux / Fedora Silverblue Testing</summary>
    Open a terminal (outside of toolbx), and:

    ```rust
    cargo run --example whoami-demo
    cargo run --example whoami-demo --release
    cargo run --example os-strings
    cargo run --example os-strings --release
    ```

    Expect to see something like:

    ```console
    WhoAmI 2.0.0

    User's Language        whoami::lang_prefs():          Collation=en/US:en,CharClasses=en/US:en,Monetary=en/US:en,Messages=en/US:en,Numeric=en/US:en,Time=en/US:en
    User's Name            whoami::realname():            Jeryn Lau
    User's Username        whoami::username():            jerynlau
    User's Username        whoami::account():             jerynlau
    Device's Pretty Name   whoami::devicename():          Ta'ra
    Device's Hostname      whoami::hostname():            taiara
    Device's Platform      whoami::platform():            Linux
    Device's OS Distro     whoami::distro():              Fedora Linux 43.20251220.0 (Silverblue)
    Device's Desktop Env.  whoami::desktop_env():         Gnome
    Device's CPU Arch      whoami::cpu_arch():            x86_64
    ```

    ```console
    WhoAmI 2.0.0

    User's Language        whoami::lang_prefs():            LanguagePreferences { fallbacks: [Language { lang: [101, 110], country: Some([85, 83]) }, Language { lang: [101, 110], country: None }], collation: None, char_classes: None, monetary: None, messages: None, numeric: None, time: None }
    User's Name            whoami::realname_os():           "Jeryn Lau"
    User's Username        whoami::username_os():           "jerynlau"
    User's Account         whoami::account_os():            "jerynlau"
    Device's Pretty Name   whoami::devicename_os():         "Ta\'ra"
    Device's Hostname      whoami::hostname():              "taiara"
    Device's Platform      whoami::platform():              Linux
    Device's OS Distro     whoami::distro():                Ok("Fedora Linux 43.20251220.0 (Silverblue)")
    Device's Desktop Env.  whoami::desktop_env():           Some(Gnome)
    Device's CPU Arch      whoami::cpu_arch():              X64
    ```

    Now, `toolbx enter`, and do the same.  Expecting something like:

    ```console
    WhoAmI 2.0.0

    User's Language        whoami::lang_prefs():          Collation=en/US:en,CharClasses=en/US:en,Monetary=en/US:en,Messages=en/US:en,Numeric=en/US:en,Time=en/US:en
    User's Name            whoami::realname():            Jeryn Lau
    User's Username        whoami::username():            jerynlau
    User's Username        whoami::account():             jerynlau
    Device's Pretty Name   whoami::devicename():          <unknown>
    Device's Hostname      whoami::hostname():            toolbx
    Device's Platform      whoami::platform():            Linux
    Device's OS Distro     whoami::distro():              Fedora Linux 43 (Toolbx Container Image)
    Device's Desktop Env.  whoami::desktop_env():         Gnome
    Device's CPU Arch      whoami::cpu_arch():            x86_64
    ```

    ```console
    WhoAmI 2.0.0

    User's Language        whoami::lang_prefs():            LanguagePreferences { fallbacks: [Language { lang: [101, 110], country: Some([85, 83]) }, Language { lang: [101, 110], country: None }], collation: None, char_classes: None, monetary: None, messages: None, numeric: None, time: None }
    User's Name            whoami::realname_os():           "Jeryn Lau"
    User's Username        whoami::username_os():           "jerynlau"
    User's Account         whoami::account_os():            "jerynlau"
    Device's Pretty Name   whoami::devicename_os():         "<unknown>"
    Device's Hostname      whoami::hostname():              "toolbx"
    Device's Platform      whoami::platform():              Linux
    Device's OS Distro     whoami::distro():                Ok("Fedora Linux 43 (Toolbx Container Image)")
    Device's Desktop Env.  whoami::desktop_env():           Some(Gnome)
    Device's CPU Arch      whoami::cpu_arch():              X64
    ```
    </details>
 - [ ] Testing complete on Ubuntu Linux 24.04.1 LTS
    <details><summary>Ubuntu Virtualized on Fedora Silverblue Testing</summary>
    <https://ubuntu.com/download/desktop>

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
    WhoAmI 2.0.0

    User's Language        whoami::lang_prefs():          Collation=en/US:en,CharClasses=en/US:en,Monetary=en/US:en,Messages=en/US:en,Numeric=en/US:en,Time=en/US:en
    User's Name            whoami::realname():            Jeryn Lau
    User's Username        whoami::username():            jeryn-lau
    User's Username        whoami::account():             jeryn-lau
    Device's Pretty Name   whoami::devicename():          Taiara
    Device's Hostname      whoami::hostname():            Taiara
    Device's Platform      whoami::platform():            Linux
    Device's OS Distro     whoami::distro():              Ubuntu 25.10
    Device's Desktop Env.  whoami::desktop_env():         Ubuntu
    Device's CPU Arch      whoami::cpu_arch():            x86_64
    ```
    </details>
 - [ ] Testing complete on Windows 11
    <details><summary>Windows Testing</summary>
    Clone whoami, open Git BASH, and run:

    ```rust
    cargo run --example whoami-demo
    cargo run --example whoami-demo --release
    ```

    Expect to see something like:

    ```console
    WhoAmI 2.0.0

    User's Language        whoami::lang_prefs():          Collation=en/US:en,CharClasses=en/US:en,Monetary=en/US:en,Messages=en/US:en,Numeric=en/US:en,Time=en/US:en
    User's Name            whoami::realname():            Jeryn Lau
    User's Username        whoami::username():            aldar
    User's Username        whoami::account():             aldar
    Device's Pretty Name   whoami::devicename():          chiypfu
    Device's Hostname      whoami::hostname():            chiypfu
    Device's Platform      whoami::platform():            Windows
    Device's OS Distro     whoami::distro():              Windows 11 (10.0.26200) (Workstation)
    Device's Desktop Env.  whoami::desktop_env():         Windows
    Device's CPU Arch      whoami::cpu_arch():            x86_64
    ```
    </details>
 - [ ] Testing complete on macOS Catalina
    <details><summary>MacOS Testing</summary>
    Clone whoami, and run:

    ```rust
    cargo run --example whoami-demo
    cargo run --example whoami-demo --release
    ```

    Expect to see something like:

    ```console
    WhoAmI 2.0.0

    User's Language        whoami::lang_prefs():          Collation=en/US:en,CharClasses=en/US:en,Monetary=en/US:en,Messages=en/US:en,Numeric=en/US:en,Time=en/US:en
    User's Name            whoami::realname():            Aldaron Lau
    User's Username        whoami::username():            aldaronlau
    User's Username        whoami::account():             aldaronlau
    Device's Pretty Name   whoami::devicename():          Aldaron’s MacBook Air
    Device's Hostname      whoami::hostname():            Aldarons-MacBook-Air.local
    Device's Platform      whoami::platform():            macOS
    Device's OS Distro     whoami::distro():              Mac OS X 10.15.7
    Device's Desktop Env.  whoami::desktop_env():         Aqua
    Device's CPU Arch      whoami::cpu_arch():            x86_64
    ```
    </details>
 - [ ] Testing complete on FreeBSD
    <details><summary>FreeBSD (virtualized on Fedora Silverblue) Testing</summary>
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
    WhoAmI 2.0.0

    User's Language        whoami::lang_prefs():          Collation=,CharClasses=,Monetary=,Messages=,Numeric=,Time=
    User's Name            whoami::realname():            Jeryn Lau
    User's Username        whoami::username():            jerynlau
    User's Username        whoami::account():             jerynlau
    Device's Pretty Name   whoami::devicename():          <unknown>
    Device's Hostname      whoami::hostname():            testing
    Device's Platform      whoami::platform():            BSD
    Device's OS Distro     whoami::distro():              FreeBSD 14.3-RELEASE
    Device's Desktop Env.  whoami::desktop_env():         <unknown>
    Device's CPU Arch      whoami::cpu_arch():            x86_64
    ```
    </details>
 - [ ] Testing complete on illumos
    <details><summary>Tribblix (virtualized on Fedora Silverblue) Testing</summary>
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
    WhoAmI 2.0.0

    User's Language        whoami::lang_prefs():          Collation=,CharClasses=,Monetary=,Messages=,Numeric=,Time=
    User's Name            whoami::realname():            Tribblix Jack
    User's Username        whoami::username():            jack
    User's Username        whoami::account():             jack
    Device's Pretty Name   whoami::devicename():          tribblix
    Device's Hostname      whoami::hostname():            tribblix
    Device's Platform      whoami::platform():            illumos
    Device's OS Distro     whoami::distro():              Tribblix
    Device's Desktop Env.  whoami::desktop_env():         XFCE
    Device's CPU Arch      whoami::cpu_arch():            x86_64
    ```
    </details>
 - [ ] Testing complete on Redox
    <details><summary>Redox (virtualized on Fedora Silverblue) Testing</summary>
    <https://doc.redox-os.org/book/building-redox.html>

    Run through "Preparing the build"

    ### Create our demo recipe

    Back in the root whoami directory, make sure whome is updated to the whoami
    testing branch.

    ```shell
    mkdir -p build/redox/recipes/demos/whome/
    cp recipe.toml build/redox/recipes/demos/whome/
    cp build/redox/config/desktop.toml build/redox/config/x86_64/ardaku.toml
    ```

    In `build/redox/config/x86_64/ardaku.toml`, under `[packages]`, add:

    ```toml
    whome = {}
    ```

    ### Select the config
    IN `build/redox/mk/config.mk`, set:

    ```make
    CONFIG_NAME?=ardaku
    ```

    ### Build Redox

    Back in `cd build/redox`, this takes a while (need to run in toolbx and outside)

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
    distro:       Redox OS 0.9.0
    desktop_env:  Orbital
    platform:     Redox
    arch:         Unknown: x86_64
    ```
    </details>
 - [ ] Testing complete on Web
    <details><summary>Build web example and start webserver on Fedora Silverblue</summary>
    Check the web console in Firefox:

    ```console
    User's Name            whoami::realname():            Anonymous
    User's Username        whoami::username():            anonymous
    User's Language        whoami::lang_prefs():          Collation=en/US:en,CharClasses=en/US:en,Monetary=en/US:en,Messages=en/US:en,Numeric=en/US:en,Time=en/US:en
    Device's Pretty Name   whoami::devicename():          Browser
    Device's Hostname      whoami::hostname():            localhost
    Device's Platform      whoami::platform():            Linux
    Device's OS Distro     whoami::distro():              Unknown Linux
    Device's Desktop Env.  whoami::desktop_env():         Web Browser (Firefox 146.0)
    Device's CPU Arch      whoami::cpu_arch():            wasm32
    ```

    Check the web console in Opera:

    ```console
    User's Name            whoami::realname():            Anonymous
    User's Username        whoami::username():            anonymous
    User's Language        whoami::lang_prefs():          Collation=en/US:en,CharClasses=en/US:en,Monetary=en/US:en,Messages=en/US:en,Numeric=en/US:en,Time=en/US:en
    Device's Pretty Name   whoami::devicename():          Browser
    Device's Hostname      whoami::hostname():            localhost
    Device's Platform      whoami::platform():            Linux
    Device's OS Distro     whoami::distro():              Unknown Linux
    Device's Desktop Env.  whoami::desktop_env():         Web Browser (Opera 125.0.0.0)
    Device's CPU Arch      whoami::cpu_arch():            wasm32
    ```

    Check the web console in Chrome:

    ```console
    User's Name            whoami::realname():            Anonymous
    User's Username        whoami::username():            anonymous
    User's Language        whoami::lang_prefs():          Collation=en/US:en,CharClasses=en/US:en,Monetary=en/US:en,Messages=en/US:en,Numeric=en/US:en,Time=en/US:en
    Device's Pretty Name   whoami::devicename():          Browser
    Device's Hostname      whoami::hostname():            localhost
    Device's Platform      whoami::platform():            Linux
    Device's OS Distro     whoami::distro():              Unknown Linux
    Device's Desktop Env.  whoami::desktop_env():         Web Browser (Chrome 143.0.0.0)
    Device's CPU Arch      whoami::cpu_arch():            wasm32
    ```

    Check the web console in Ungoogled Chromium:

    ```console
    User's Name            whoami::realname():            Anonymous
    User's Username        whoami::username():            anonymous
    User's Language        whoami::lang_prefs():          Collation=en/US:en,CharClasses=en/US:en,Monetary=en/US:en,Messages=en/US:en,Numeric=en/US:en,Time=en/US:en
    Device's Pretty Name   whoami::devicename():          Browser
    Device's Hostname      whoami::hostname():            localhost
    Device's Platform      whoami::platform():            Linux
    Device's OS Distro     whoami::distro():              Unknown Linux
    Device's Desktop Env.  whoami::desktop_env():         Web Browser (Chrome 143.0.0.0)
    Device's CPU Arch      whoami::cpu_arch():            wasm32
    ```

    Check the web console in GNOME Web (Epiphany):

    ```console
    User's Name            whoami::realname():            Anonymous
    User's Username        whoami::username():            anonymous
    User's Language        whoami::lang_prefs():          Collation=en/US:en,CharClasses=en/US:en,Monetary=en/US:en,Messages=en/US:en,Numeric=en/US:en,Time=en/US:en
    Device's Pretty Name   whoami::devicename():          Browser
    Device's Hostname      whoami::hostname():            localhost
    Device's Platform      whoami::platform():            Linux
    Device's OS Distro     whoami::distro():              Unknown Linux
    Device's Desktop Env.  whoami::desktop_env():         Web Browser (GNOME Web)
    Device's CPU Arch      whoami::cpu_arch():            wasm32
    ```

# Changelog

## Added

## Changed

## Fixed

## Removed
