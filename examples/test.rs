use whoami;

fn main() {
    print!(
        "--------------------------------------------------------------------------------\n\
         user's full name (user):              {}\n\
         username (username):                  {}\n\
         --------------------------------------------------------------------------------\n\
         host's fancy name (host):             {}\n\
         hostname (hostname):                  {}\n\
         --------------------------------------------------------------------------------\n\
         operating system (os):                {}\n\
         desktop environment (env):            {}\n\
         --------------------------------------------------------------------------------\n\
         ",
        whoami::user(),
        whoami::username(),
        whoami::host(),
        whoami::hostname(),
        whoami::os(),
        whoami::env(),
    );
}
