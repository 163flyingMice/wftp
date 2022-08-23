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
            let wftp: Wftp = from_str(&contents).unwrap();
            let mut server = Vec::new();
            for elem in wftp.servers.server.into_iter() {
                server.push(elem);
            }
            Some(server)
        }
        Err(err) => {
            println!("{}", err);
            None
        }
    }
}

pub fn insert_wftp_server() {
    let xml = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
    <wftp version=\"1.0\" platform=\"windows\">
        <Servers>
            <Server>
                <Host>
                    120.76.76.154
                </Host>
                <Port>
                    65521
                </Port>
                <User>
                    zhangwenhao
                </User>
                <Pass>
                    am01MjAxMzE0
                </Pass>
                <Name>
                centos
                </Name>
                <Protocol>1</Protocol>
                <LogonType>1</LogonType>
            </Server>
            <Server>
                <Host>
                    120.76.76.154
                </Host>
                <Port>
                    65521
                </Port>
                <User>
                    zhangwenhao
                </User>
                <Pass>
                    am01MjAxMzE0
                </Pass>
                <Name>
                centos1
                </Name>
                <Protocol>1</Protocol>
                <LogonType>1</LogonType>
            </Server>
        </Servers>
    </wftp>
    ";
    let mut wftp: Wftp = from_str(&xml).unwrap();
    for mut elem in wftp.servers.server.iter_mut() {
        elem.name = Name(String::from("111"));
        println!("{:?}", elem);
    }
    let mut buffer = Vec::new();

    {
        let mut ser = Serializer::new(&mut buffer);
        wftp.serialize(&mut ser).unwrap();
    }
    let got = String::from_utf8(buffer).unwrap();
    println!("{}", got);
}
