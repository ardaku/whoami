use devout::*;
use wasm_bindgen::prelude::*;

const INFO: &str = "Info";

#[wasm_bindgen]
pub fn wasm_main() {
    out!(INFO, "User→Name      whoami::realname():    {}", whoami::realname());
    out!(INFO, "User→Username  whoami::username():    {}", whoami::username());
    out!(INFO, "Host→Name      whoami::devicename():  {}", whoami::devicename());
    out!(INFO, "Host→Hostname  whoami::hostname():    {}", whoami::hostname());
    out!(INFO, "Platform       whoami::platform():    {}", whoami::platform());
    out!(INFO, "OS Distro      whoami::distro():      {}", whoami::distro());
    out!(INFO, "Desktop Env.   whoami::desktop_env(): {}", whoami::desktop_env());
}
