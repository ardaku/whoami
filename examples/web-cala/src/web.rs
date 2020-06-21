use cala_core;
use devout::*;

const INFO: &str = "Info";

cala_core::exec!(whoami_web);

async fn whoami_web() {
    out!(INFO, "-------------------------------------------------------------");
    out!(INFO, "user's real name (realname):          {}", whoami::realname());
    out!(INFO, "username (username):                  {}", whoami::username());
    out!(INFO, "-------------------------------------------------------------");
    out!(INFO, "host's fancy name (devicename):       {}", whoami::devicename());
    out!(INFO, "hostname (hostname):                  {}", whoami::hostname());
    out!(INFO, "-------------------------------------------------------------");
    out!(INFO, "platform (platform):                  {}", whoami::platform());
    out!(INFO, "operating system (distro):            {}", whoami::distro());
    out!(INFO, "desktop environment (desktop_env):    {}", whoami::desktop_env());
    out!(INFO, "-------------------------------------------------------------");
}
