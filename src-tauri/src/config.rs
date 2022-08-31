use quick_xml::{de::from_str, se::Serializer};
use serde::{Deserialize, Serialize};

use crate::result::{Error, Success};

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
pub struct Protocol(usize);

#[derive(Debug, Deserialize, PartialEq, Default, Serialize)]
pub struct LogonType(usize);

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
pub fn get_default_wftp() -> String {
    serde_json::to_string(&Success::new(
        200,
        String::from("获取成功！"),
        String::from(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
        <wftp platform=\"windows\" version=\"1.0\">
            <Servers>
                <Server>
                    <Host>127.0.0.1</Host>
                    <Port>65521</Port>
                    <User>zhangwenhao</User>
                    <Pass>am01MjAxMzE0</Pass>
                    <Name>6666</Name>
                    <Protocol>1</Protocol>
                    <LogonType>1</LogonType>
                </Server>
                <Server>
                    <Host>127.0.0.1</Host>
                    <Port>65521</Port>
                    <User>zhangwenhao</User>
                    <Pass>am01MjAxMzE0</Pass>
                    <Name>6666</Name>
                    <Protocol>1</Protocol>
                    <LogonType>1</LogonType>
                </Server>
            </Servers>
        </wftp>",
        ),
    ))
    .unwrap()
}

#[tauri::command]
pub fn get_wftp_server(wftp_xml: String) -> String {
    match from_str(&wftp_xml) {
        Ok(wftp) => {
            let wftp: Wftp = wftp;
            let mut server = Vec::new();
            for elem in wftp.servers.server.into_iter() {
                server.push(elem);
            }
            serde_json::to_string(&Success::new(200, String::from("获取成功！"), server)).unwrap()
        }
        Err(err) => serde_json::to_string(&Error::new(400, err.to_string())).unwrap(),
    }
}

#[tauri::command]
pub fn wftp_xml_string(xml_string: String) -> String {
    match serde_json::from_str(&xml_string) {
        Ok(servers) => {
            let wftp_server = Wftp {
                servers: Servers { server: servers },
                platform: String::from("windows"),
                version: String::from("1.0"),
            };
            let mut buffer = Vec::new();
            let mut ser = Serializer::new(&mut buffer);
            match wftp_server.serialize(&mut ser) {
                Ok(_) => match String::from_utf8(buffer) {
                    Ok(t) => {
                        let wftp_xml = XML_HEADER.to_string() + &t;
                        serde_json::to_string(&Success::new(
                            200,
                            String::from("获取成功！"),
                            wftp_xml,
                        ))
                        .unwrap()
                    }
                    Err(err) => serde_json::to_string(&Error::new(400, err.to_string())).unwrap(),
                },
                Err(err) => serde_json::to_string(&Error::new(400, err.to_string())).unwrap(),
            }
        }
        Err(err) => serde_json::to_string(&Error::new(400, err.to_string())).unwrap(),
    }
}
