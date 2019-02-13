use ssh2::Session;
use std::io::prelude::*;
use std::net::TcpStream;
use std::path::Path;
pub fn invoke_command(command: &str, ip: &str) {
    println!("Invoking {} @ {}", command, ip);
    let tcp = TcpStream::connect(ip).unwrap();
    let mut sess = Session::new().unwrap();
    sess.handshake(&tcp).unwrap();
    sess.userauth_pubkey_file(
        "ubuntu",
        None,
        Path::new("C:\\Users\\Joro\\Downloads\\jorotenev-fr-2019.pem"),
        None,
    )
    .unwrap();
    let mut channel = sess.channel_session().unwrap();
    channel.exec(command).unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    channel.wait_close().unwrap();
    println!("{}", channel.exit_status().unwrap());
}
