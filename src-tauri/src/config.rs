use quick_xml::{de::from_str, se::Serializer};
use serde::{Deserialize, Serialize};
use std::{
    fs::OpenOptions,
    io::{BufReader, Read},
};

#[derive(Debug, Deserialize, PartialEq, Default, Serialize)]
#[serde(rename = "wftp", default)]
pub struct Wftp {
    #[serde(rename = "Servers", default)]
    pub servers: Servers,
    pub platform: String,
    pub version: String,
}

#[derive(Debug, Deserialize, PartialEq, Default, Serialize)]
pub struct Servers {
    #[serde(rename = "Server", default)]
    pub server: Vec<WftpServer>,
}

#[derive(Debug, Deserialize, PartialEq, Default, Serialize)]
pub struct Host(String);

#[derive(Debug, Deserialize, PartialEq, Default, Serialize)]
pub struct Port(String);

#[derive(Debug, Deserialize, PartialEq, Default, Serialize)]
pub struct User(String);

#[derive(Debug, Deserialize, PartialEq, Default, Serialize)]
pub struct Pass(String);

#[derive(Debug, Deserialize, PartialEq, Default, Serialize)]
pub struct Name(String);

#[derive(Debug, Deserialize, PartialEq, Default, Serialize)]
pub struct Protocol(String);

#[derive(Debug, Deserialize, PartialEq, Default, Serialize)]
pub struct LogonType(String);

#[derive(Debug, Deserialize, PartialEq, Default, Serialize)]
pub struct WftpServer {
    #[serde(rename = "Host", default)]
    pub host: Host,
    #[serde(rename = "Port", default)]
    pub port: Port,
    #[serde(rename = "User", default)]
    pub user: User,
    #[serde(rename = "Pass", default)]
    pub pass: Pass,
    #[serde(rename = "Name", default)]
    pub name: Name,
    #[serde(rename = "Protocol", default)]
    pub protocol: Protocol,
    #[serde(rename = "LogonType", default)]
    pub logintype: LogonType,
}

const XML_HEADER: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>";

#[tauri::command]
pub fn get_default_wftp() -> Option<String> {
    match OpenOptions::new().read(true).open("wftp.xml") {
        Ok(file) => {
            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();
            let _ = buf_reader.read_to_string(&mut contents);
            Some(contents)
        }
        Err(err) => {
            println!("{}", err);
            None
        }
    }
}

#[tauri::command]
pub fn get_wftp_server(wftp_xml: String) -> Option<Vec<WftpServer>> {
    let wftp: Wftp = from_str(&wftp_xml).unwrap();
    let mut server = Vec::new();
    for elem in wftp.servers.server.into_iter() {
        server.push(elem);
    }
    Some(server)
}

#[tauri::command]
pub fn wftp_xml_string(xml_string: String) -> String {
    let servers: Vec<WftpServer> = serde_json::from_str(&xml_string).unwrap();
    let wftp_server = Wftp {
        servers: Servers { server: servers },
        platform: String::from("windows"),
        version: String::from("1.0"),
    };
    let mut buffer = Vec::new();
    let mut ser = Serializer::new(&mut buffer);
    let _ = wftp_server.serialize(&mut ser).unwrap();
    let wftp_xml = XML_HEADER.to_string() + &(String::from_utf8(buffer).unwrap());
    wftp_xml
}
