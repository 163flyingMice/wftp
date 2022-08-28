use ssh2::Session;
use std::io::prelude::*;
use std::net::TcpStream;
use std::path::Path;

pub fn connect() {
    let tcp = TcpStream::connect("118.24.26.116:22").unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_password("root", "Aa33344565").unwrap();
    sess.keepalive_send().unwrap();
    let sftp = sess.sftp().unwrap();
    let result = sftp.readdir(Path::new(".cache")).unwrap();
    println!("{:?}", result);

    let mut channel = sess.channel_session().unwrap();
    let _ = channel.exec("ls -la").unwrap();
    let mut s = String::new();
    let len = channel.read_to_string(&mut s).unwrap();
    println!("{:?}", len);
    println!("{:?}", s);
    // let _ = channel.wait_close();
    // println!("{}", channel.exit_status().unwrap());
}
