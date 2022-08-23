use quick_xml::events::Event;
use quick_xml::Reader;
use serde::{Deserialize, Serialize};
use std::{
    fs::OpenOptions,
    io::{BufReader, Read},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct WftpServer {
    pub host: String,
    pub port: String,
    pub user: String,
    pub pass: String,
    pub name: String,
    pub protocol: String,
    pub logintype: String,
}

impl WftpServer {
    fn new() -> Self {
        WftpServer {
            host: String::from(""),
            port: String::from(""),
            user: String::from(""),
            pass: String::from(""),
            name: String::from(""),
            protocol: String::from(""),
            logintype: String::from(""),
        }
    }
}

#[tauri::command]
pub fn get_wftp_server() -> Option<Vec<WftpServer>> {
    match OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open("wftp.xml")
    {
        Ok(file) => {
            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();
            let _ = buf_reader.read_to_string(&mut contents);
            let mut reader = Reader::from_str(&contents);
            let mut buf = Vec::new();
            let mut txt = Vec::new();
            loop {
                match reader.read_event(&mut buf) {
                    Ok(Event::Text(e)) => txt.push(e.unescape_and_decode(&reader).unwrap()),
                    Ok(Event::Eof) => break,
                    Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                    _ => (),
                }
                reader.trim_text(true);
            }
            buf.clear();
            let mut wftp_server_vec = Vec::<WftpServer>::new();
            let mut iter = txt.into_iter();
            iter.next();
            loop {
                let mut wftp_server = WftpServer::new();
                if let Some(t) = iter.next() {
                    wftp_server.host = t;
                } else {
                    break;
                }
                if let Some(t) = iter.next() {
                    wftp_server.port = t;
                } else {
                    break;
                }
                if let Some(t) = iter.next() {
                    wftp_server.user = t;
                } else {
                    break;
                }
                if let Some(t) = iter.next() {
                    wftp_server.pass = t;
                } else {
                    break;
                }
                if let Some(t) = iter.next() {
                    wftp_server.name = t;
                } else {
                    break;
                }
                if let Some(t) = iter.next() {
                    wftp_server.protocol = t;
                } else {
                    break;
                }
                if let Some(t) = iter.next() {
                    wftp_server.logintype = t;
                } else {
                    break;
                }
                wftp_server_vec.push(wftp_server);
            }
            Some(wftp_server_vec)
        }
        Err(err) => {
            println!("{}", err);
            None
        }
    }
}
