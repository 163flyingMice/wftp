extern crate ftp;
use std::collections::HashMap;

use ftp::FtpStream;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FolderTree {
    pub title: String,
    pub key: String,
    pub children: Option<Vec<FolderLeaf>>,
}

impl FolderTree {
    fn new() -> Self {
        FolderTree {
            title: String::from("/"),
            key: String::from("0"),
            children: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FolderLeaf {
    pub title: String,
    pub key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileList {
    pub permissions: String,
    pub owner: String,
    pub group: String,
    pub size: String,
    pub update_at: String,
    pub is_directory: String,
    pub name: HashMap<String, String>,
}

static mut OWNER_FTP_STREAM: Option<FtpStream> = None;

impl FileList {
    fn new() -> Self {
        let mut name = HashMap::new();
        name.insert(String::from("kind"), String::from("folder"));
        name.insert(String::from("name"), String::from(".."));
        FileList {
            permissions: String::from(""),
            owner: String::from(""),
            group: String::from(""),
            size: String::from(""),
            update_at: String::from(""),
            is_directory: String::from(""),
            name: name,
        }
    }
}
#[tauri::command]
pub fn connect(addr: String, username: String, password: String) -> String {
    let mut ftp_stream = FtpStream::connect(&addr).unwrap();
    let _ = ftp_stream.login(&username, &password);
    unsafe {
        OWNER_FTP_STREAM = Some(ftp_stream);
    }
    String::from("连接成功！")
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
pub fn pwd() -> String {
    let mut root = "".to_string();
    unsafe {
        root = OWNER_FTP_STREAM.as_mut().unwrap().pwd().unwrap();
    }
    root
}

#[tauri::command]
pub fn prev() -> String {
    // let pwd = pwd();
    // let extens: Vec<&str> = pwd.split("/").collect();
    // let path = String::from("/") + &extens[1..(extens.len() - 1)].join("/");
    unsafe {
        match OWNER_FTP_STREAM.as_mut().unwrap().cdup() {
            Ok(_) => "更改文件夹成功！".to_string(),
            Err(err) => err.to_string(),
        }
    }
}

#[tauri::command]
pub fn cwd(path: String) -> String {
    unsafe {
        match OWNER_FTP_STREAM.as_mut().unwrap().cwd(&path) {
            Ok(_) => "更改文件夹成功！".to_string(),
            Err(err) => err.to_string(),
        }
    }
}

#[tauri::command]
pub fn list(path: String) -> Vec<FileList> {
    let mut list: Vec<String> = Vec::new();
    unsafe {
        list = OWNER_FTP_STREAM
            .as_mut()
            .unwrap()
            .list(None)
            .expect("获取列表失败！");
    }
    let mut file_list = vec![FileList::new()];
    for param in &list {
        let temp = param
            .trim()
            .split(" ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let temp_len = temp.len();
        let mut is_directory: String = "".to_string();
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
        file_list.push(FileList {
            permissions: temp.iter().nth(0).unwrap().to_string(),
            owner: temp.iter().nth(2).unwrap().to_string(),
            group: temp.iter().nth(3).unwrap().to_string(),
            size: temp.iter().nth(temp_len - 6).unwrap().to_string(),
            is_directory: is_directory,
            update_at: temp.iter().nth(temp_len - 4).unwrap().to_string()
                + " "
                + temp.iter().nth(temp_len - 3).unwrap()
                + " "
                + temp.iter().nth(temp_len - 2).unwrap(),
            name: name,
        });
    }
    file_list
}

#[tauri::command]
pub fn folder_list() -> FolderTree {
    let mut list: Vec<String> = Vec::new();
    unsafe {
        list = OWNER_FTP_STREAM
            .as_mut()
            .unwrap()
            .list(None)
            .expect("获取列表失败！");
    }
    let mut folder_tree = FolderTree::new();
    let mut folder_leaf = Vec::new();
    for param in &list {
        let temp = param
            .trim()
            .split(" ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let temp_len = temp.len();
        folder_leaf.push(FolderLeaf {
            title: temp.iter().nth(temp_len - 1).unwrap().to_string(),
            key: String::from("0-1"),
        });
    }
    folder_tree.children = Some(folder_leaf);
    folder_tree
}

#[tauri::command]
pub fn rename_file(from_name: String, to_name: String) -> String {
    unsafe {
        match OWNER_FTP_STREAM
            .as_mut()
            .unwrap()
            .rename(&from_name, &to_name)
        {
            Ok(_) => "更改文件夹名称成功！".to_string(),
            Err(err) => err.to_string(),
        }
    }
}

#[tauri::command]
pub fn remove_file(filename: String) -> String {
    unsafe {
        match OWNER_FTP_STREAM.as_mut().unwrap().rm(&filename) {
            Ok(_) => ("删除文件".to_string() + &filename + "成功！").to_string(),
            Err(err) => err.to_string(),
        }
    }
}

#[tauri::command]
pub fn remove_dir(path: String) -> String {
    unsafe {
        match OWNER_FTP_STREAM.as_mut().unwrap().rmdir(&path) {
            Ok(_) => ("删除文件夹".to_string() + &path + "成功！").to_string(),
            Err(err) => err.to_string(),
        }
    }
}

#[tauri::command]
pub fn mk_dir(path: String) -> String {
    unsafe {
        match OWNER_FTP_STREAM.as_mut().unwrap().mkdir(&path) {
            Ok(_) => ("创建文件夹".to_string() + &path + "成功！").to_string(),
            Err(err) => err.to_string(),
        }
    }
}

#[tauri::command]
pub fn mk_file(filename: String) -> String {
    unsafe {
        let mut b = "".as_bytes();
        match OWNER_FTP_STREAM.as_mut().unwrap().put(&filename, &mut b) {
            Ok(_) => ("创建文件夹".to_string() + &filename + "成功！").to_string(),
            Err(err) => err.to_string(),
        }
    }
}