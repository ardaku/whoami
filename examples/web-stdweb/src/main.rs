use devout::*;

const INFO: &str = "Info";

fn main() {
    out!(INFO, "-------------------------------------------------------------");
    out!(INFO, "user's full name (user):              {}", whoami::user());
    out!(INFO, "username (username):                  {}", whoami::username());
    out!(INFO, "-------------------------------------------------------------");
    out!(INFO, "host's fancy name (host):             {}", whoami::host());
    out!(INFO, "hostname (hostname):                  {}", whoami::hostname());
    out!(INFO, "-------------------------------------------------------------");
    out!(INFO, "platform (platform):                  {}", whoami::platform());
    out!(INFO, "operating system (os):                {}", whoami::os());
    out!(INFO, "desktop environment (env):            {}", whoami::env());
    out!(INFO, "-------------------------------------------------------------");
}
