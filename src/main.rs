
use std::net::TcpStream;
use ssh2::Session;
use std::path::Path;
use std::io::prelude::*;

fn main() {
    let tcp = TcpStream::connect("18.185.96.63:22").unwrap();
    let mut sess = Session::new().unwrap();
    sess.handshake(&tcp).unwrap();
    sess.userauth_pubkey_file("ubuntu", None, Path::new("C:\\Users\\Joro\\Downloads\\jorotenev-fr-2019.pem"), None).unwrap();
    let mut channel = sess.channel_session().unwrap();
    channel.exec("ls -a").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    channel.wait_close().unwrap();
    println!("{}", channel.exit_status().unwrap());


}
 