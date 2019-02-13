mod modules;
use self::modules::ssh;


fn main() {
    let command = "ls -a";
    let ip = "18.185.96.63:22";
    ssh::invoke_command(command, ip)
}
