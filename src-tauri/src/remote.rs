extern crate ftp;
use base64_stream::base64::decode;
use ftp::FtpStream;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Mutex};

use crate::result::CustomError;
use crate::result::{Error, Success, CONNECTED_SUCCESS_CODE, DISCONNECTED_ERROR_CODE};

#[derive(Debug, Serialize, Deserialize)]
pub struct FolderTree {
    pub title: String,
    pub key: String,
    pub children: Option<Vec<FolderLeaf>>,
}

impl FolderTree {
    pub fn new() -> Self {
        FolderTree {
            title: String::from("/"),
            key: String::from("0"),
            children: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FolderLeaf {
    pub title: Option<String>,
    pub key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileList {
    pub permissions: String,
    pub owner: String,
    pub group: String,
    pub size: usize,
    pub update_at: String,
    pub is_directory: String,
    pub name: HashMap<String, String>,
}

lazy_static! {
    static ref OWNER_FTP_STREAM: Mutex<HashMap<String, Option<FtpStream>>> = {
        let map: HashMap<String, Option<FtpStream>> = HashMap::new();
        Mutex::new(map)
    };
}

impl FileList {
    pub fn new() -> Self {
        let mut name = HashMap::new();
        name.insert(String::from("kind"), String::from("folder"));
        name.insert(String::from("name"), String::from(".."));
        FileList {
            permissions: String::from(""),
            owner: String::from(""),
            group: String::from(""),
            size: 0,
            update_at: String::from(""),
            is_directory: String::from(""),
            name: name,
        }
    }
}

#[tauri::command]
pub fn alive(name: String) -> String {
    if let Some(temp) = OWNER_FTP_STREAM.lock().unwrap().get_mut(&name) {
        if let Some(_) = temp {
            return serde_json::to_string(&Success::new(
                CONNECTED_SUCCESS_CODE,
                String::from("连接中"),
                (),
            ))
            .unwrap();
        }
    }
    serde_json::to_string(&Error::new(
        DISCONNECTED_ERROR_CODE,
        CustomError::GetFtpstreamError.to_string(),
    ))
    .unwrap()
}

#[tauri::command]
pub fn connect(addr: String, user: String, pass: String, name: String) -> String {
    let mut ftp_stream;
    if let Ok(t) = FtpStream::connect(&addr) {
        ftp_stream = t;
        let _ = ftp_stream.login(&user, &pass);
        OWNER_FTP_STREAM
            .lock()
            .unwrap()
            .insert(name, Some(ftp_stream));
        serde_json::to_string(&Success::new(
            CONNECTED_SUCCESS_CODE,
            String::from("连接成功！"),
            (),
        ))
        .unwrap()
    } else {
        serde_json::to_string(&Error::new(
            DISCONNECTED_ERROR_CODE,
            CustomError::GetFtpstreamError.to_string(),
        ))
        .unwrap()
    }
}

#[tauri::command]
pub fn try_connect(addr: String, username: String, password: String) -> String {
    let mut ftp_stream = FtpStream::connect(&addr).unwrap();
    let result = ftp_stream.login(&username, &password);
    match result {
        Ok(()) => "连接成功！".to_string(),
        Err(err) => err.to_string(),
    }
}

#[tauri::command]
pub fn pwd(name: String) -> Option<String> {
    if let Some(temp) = OWNER_FTP_STREAM.lock().unwrap().get_mut(&name) {
        if let Some(ftp_stream) = temp {
            let root = ftp_stream.pwd().unwrap();
            return Some(root);
        }
    }
    None
}

#[tauri::command]
pub fn prev(name: String) -> String {
    if let Some(temp) = OWNER_FTP_STREAM.lock().unwrap().get_mut(&name) {
        if let Some(ftp_stream) = temp {
            match ftp_stream.cdup() {
                Ok(_) => {
                    return "更改文件夹成功！".to_string();
                }
                Err(err) => {
                    return err.to_string();
                }
            }
        }
    }
    String::from("更改文件夹失败！")
}

#[tauri::command]
pub fn cwd(name: String, path: String) -> String {
    if let Some(temp) = OWNER_FTP_STREAM.lock().unwrap().get_mut(&name) {
        if let Some(ftp_stream) = temp {
            match ftp_stream.cwd(&path) {
                Ok(_) => {
                    return "更改文件夹成功！".to_string();
                }
                Err(err) => {
                    return err.to_string();
                }
            }
        }
    }
    String::from("更改文件夹失败！")
}

#[tauri::command]
pub fn list(name: String) -> Option<Vec<FileList>> {
    if let Some(temp) = OWNER_FTP_STREAM.lock().unwrap().get_mut(&name) {
        if let Some(ftp_stream) = temp {
            let list = ftp_stream.list(None).expect("获取列表失败！");
            let mut file_list = vec![FileList::new()];
            for param in &list {
                let temp = param
                    .trim()
                    .split(" ")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();
                let temp_len = temp.len();
                let is_directory: String;
                let first_at = temp.iter().nth(0).unwrap().chars().next().unwrap();
                let mut name = HashMap::new();
                let temp_name = temp.iter().nth(temp_len - 1).unwrap().to_string();
                if first_at == 'd' {
                    is_directory = "文件夹".to_string();
                    name.insert(String::from("kind"), String::from("folder"));
                } else {
                    let extens: Vec<&str> = temp_name.split(".").collect();
                    is_directory = extens[(extens.len() - 1)].to_string().to_uppercase() + " 文件";
                    name.insert(String::from("kind"), String::from("file"));
                }
                name.insert(String::from("name"), temp_name);
                let size: usize;
                let temp_size = temp.iter().nth(temp_len - 6).unwrap();
                if temp_size != "0" && temp_size != "" {
                    size = temp_size.parse::<usize>().unwrap();
                } else {
                    size = 0;
                }
                file_list.push(FileList {
                    permissions: temp.iter().nth(0).unwrap().to_string(),
                    owner: temp.iter().nth(2).unwrap().to_string(),
                    group: temp.iter().nth(3).unwrap().to_string(),
                    size: size,
                    is_directory: is_directory,
                    update_at: temp.iter().nth(temp_len - 4).unwrap().to_string()
                        + " "
                        + temp.iter().nth(temp_len - 3).unwrap()
                        + " "
                        + temp.iter().nth(temp_len - 2).unwrap(),
                    name: name,
                });
            }
            return Some(file_list);
        }
    }
    None
}

#[tauri::command]
pub fn folder_list(name: String) -> Option<FolderTree> {
    if let Some(temp) = OWNER_FTP_STREAM.lock().unwrap().get_mut(&name) {
        if let Some(ftp_stream) = temp {
            let list = ftp_stream.list(None).expect("获取列表失败！");
            let mut folder_tree = FolderTree::new();
            let mut folder_leaf = Vec::new();
            let mut num = 0;
            for param in &list {
                let temp = param
                    .trim()
                    .split(" ")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();
                let temp_len = temp.len();
                folder_leaf.push(FolderLeaf {
                    title: Some(temp.iter().nth(temp_len - 1).unwrap().to_string()),
                    key: String::from(String::from("0-") + &(num.clone().to_string())),
                });
                num += 1;
            }
            folder_tree.children = Some(folder_leaf);
            Some(folder_tree)
        } else {
            None
        }
    } else {
        None
    }
}

#[tauri::command]
pub fn rename_file(name: String, from_name: String, to_name: String) -> String {
    if let Some(temp) = OWNER_FTP_STREAM.lock().unwrap().get_mut(&name) {
        if let Some(ftp_stream) = temp {
            match ftp_stream.rename(&from_name, &to_name) {
                Ok(_) => {
                    return "更改文件夹名称成功！".to_string();
                }
                Err(err) => {
                    return err.to_string();
                }
            }
        }
    }
    String::from("更改文件夹名称失败！")
}

#[tauri::command]
pub fn remove_file(name: String, filename: String) -> String {
    if let Some(temp) = OWNER_FTP_STREAM.lock().unwrap().get_mut(&name) {
        if let Some(ftp_stream) = temp {
            match ftp_stream.rm(&filename) {
                Ok(_) => {
                    return ("删除文件".to_string() + &filename + "成功！").to_string();
                }
                Err(err) => {
                    return err.to_string();
                }
            }
        }
    }
    String::from("删除文件失败！")
}

#[tauri::command]
pub fn remove_dir(name: String, path: String) -> String {
    if let Some(temp) = OWNER_FTP_STREAM.lock().unwrap().get_mut(&name) {
        if let Some(ftp_stream) = temp {
            match ftp_stream.rmdir(&path) {
                Ok(_) => {
                    return ("删除文件夹".to_string() + &path + "成功！").to_string();
                }
                Err(err) => {
                    return err.to_string();
                }
            }
        }
    }
    String::from("删除文件夹失败！")
}

#[tauri::command]
pub fn mk_dir(name: String, path: String) -> String {
    if let Some(temp) = OWNER_FTP_STREAM.lock().unwrap().get_mut(&name) {
        if let Some(ftp_stream) = temp {
            match ftp_stream.mkdir(&path) {
                Ok(_) => {
                    return ("创建文件夹".to_string() + &path + "成功！").to_string();
                }
                Err(err) => {
                    return err.to_string();
                }
            }
        }
    }
    String::from("创建文件夹失败！")
}

#[tauri::command]
pub fn mk_file(name: String, filename: String) -> String {
    if let Some(temp) = OWNER_FTP_STREAM.lock().unwrap().get_mut(&name) {
        if let Some(ftp_stream) = temp {
            let mut b = "".as_bytes();
            match ftp_stream.put(&filename, &mut b) {
                Ok(_) => {
                    return ("创建文件".to_string() + &filename + "成功！").to_string();
                }
                Err(err) => {
                    return err.to_string();
                }
            }
        }
    }
    String::from("创建文件失败！")
}

#[tauri::command]
pub fn upload(name: String, filename: String, content: String) -> String {
    if let Some(temp) = OWNER_FTP_STREAM.lock().unwrap().get_mut(&name) {
        if let Some(ftp_stream) = temp {
            let temp = decode(content).unwrap();
            let mut b = temp.as_slice();
            match ftp_stream.put(&filename, &mut b) {
                Ok(_) => {
                    return ("上传文件".to_string() + &filename + "成功！").to_string();
                }
                Err(err) => {
                    return err.to_string();
                }
            }
        }
    }
    String::from("上传文件失败！")
}

#[tauri::command]
pub fn quit(name: String) -> String {
    if let Some(temp) = OWNER_FTP_STREAM.lock().unwrap().get_mut(&name) {
        if let Some(ftp_stream) = temp {
            let _ = ftp_stream.quit();
            return String::from("已断开连接!");
        }
    }
    String::from("出现异常!")
}

#[tauri::command]
pub fn size_sort(mut file_list: Option<Vec<FileList>>, sort_way: bool) -> Option<Vec<FileList>> {
    if let Some(list) = file_list.as_mut() {
        let mut len = list.len();
        while len > 1 {
            let mut pos_max = 0;
            for i in 1..len {
                if sort_way {
                    if list[i].size > list[pos_max].size {
                        pos_max = i;
                    }
                } else {
                    if list[i].size < list[pos_max].size {
                        pos_max = i;
                    }
                }
            }
            len -= 1;
            list.swap(pos_max, len);
        }
        return Some(list.to_vec());
    }
    None
}
