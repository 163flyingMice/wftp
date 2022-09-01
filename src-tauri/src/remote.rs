extern crate ftp;
use base64_stream::base64::decode;
use ftp::FtpStream;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Mutex};

use crate::result::CustomError;
use crate::result::{Error, Success, CONNECTED_SUCCESS_CODE, DISCONNECTED_ERROR_CODE};
use crate::util::get_snow_id;

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
            return Success::new(CONNECTED_SUCCESS_CODE, "连接中", ()).out();
        }
    }
    Error::new(DISCONNECTED_ERROR_CODE, CustomError::GetFtpstreamError).out()
}

#[tauri::command]
pub fn connect(addr: String, user: String, pass: String) -> String {
    match FtpStream::connect(&addr) {
        Ok(mut ftp_stream) => match ftp_stream.login(&user, &pass) {
            Ok(_) => {
                let snow_id = get_snow_id();
                OWNER_FTP_STREAM
                    .lock()
                    .unwrap()
                    .insert(snow_id.clone(), Some(ftp_stream));
                return Success::new(CONNECTED_SUCCESS_CODE, "连接成功！", snow_id.clone()).out();
            }
            Err(err) => return Error::new(DISCONNECTED_ERROR_CODE, err).out(),
        },
        Err(err) => return Error::new(DISCONNECTED_ERROR_CODE, err).out(),
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
pub fn pwd(name: String) -> String {
    match OWNER_FTP_STREAM.lock() {
        Ok(mut s) => {
            if let Some(temp) = s.get_mut(&name) {
                if let Some(ftp_stream) = temp {
                    match ftp_stream.pwd() {
                        Ok(root) => {
                            return Success::new(200, "获取成功！", root).out();
                        }
                        Err(err) => return Error::new(502, err).out(),
                    }
                }
            }
        }
        Err(err) => return Error::new(502, err).out(),
    }
    return Error::new(502, "获取失败！").out();
}

#[tauri::command]
pub fn prev(name: String) -> String {
    match OWNER_FTP_STREAM.lock() {
        Ok(mut s) => {
            if let Some(temp) = s.get_mut(&name) {
                if let Some(ftp_stream) = temp {
                    match ftp_stream.cdup() {
                        Ok(_) => {
                            return Success::new(200, "更改文件夹成功！", ()).out();
                        }
                        Err(err) => return Error::new(502, err).out(),
                    }
                }
            }
        }
        Err(err) => return Error::new(502, err).out(),
    }
    return Error::new(502, "更改文件夹失败！").out();
}

#[tauri::command]
pub fn cwd(name: String, path: String) -> String {
    match OWNER_FTP_STREAM.lock() {
        Ok(mut s) => {
            if let Some(temp) = s.get_mut(&name) {
                if let Some(ftp_stream) = temp {
                    match ftp_stream.cwd(&path) {
                        Ok(_) => {
                            return Success::new(200, "更改文件夹成功！", ()).out();
                        }
                        Err(err) => return Error::new(502, err).out(),
                    }
                }
            }
        }
        Err(err) => return Error::new(502, err).out(),
    }
    return Error::new(502, "更改文件夹失败！").out();
}

#[tauri::command]
pub fn list(name: String) -> String {
    match OWNER_FTP_STREAM.lock() {
        Ok(mut s) => {
            if let Some(temp) = s.get_mut(&name) {
                if let Some(ftp_stream) = temp {
                    match ftp_stream.list(None) {
                        Ok(list) => {
                            let mut file_list = vec![FileList::new()];
                            for param in &list {
                                let temp = param
                                    .trim()
                                    .split(" ")
                                    .map(|s| s.to_string())
                                    .collect::<Vec<String>>();
                                let temp_len = temp.len();
                                let is_directory: String;
                                let mut first_at: char = 'd';
                                let mut perm: String = String::from("---------");
                                if let Some(temp_f) = temp.iter().nth(0) {
                                    perm = temp_f.to_string();
                                    if let Some(temp_s) = temp_f.chars().next() {
                                        first_at = temp_s;
                                    }
                                }
                                let mut name = HashMap::new();
                                let mut temp_name = String::from("不知名资源文件");
                                if let Some(temp_f) = temp.iter().nth(temp_len - 1) {
                                    temp_name = temp_f.to_string();
                                }
                                if first_at == 'd' {
                                    is_directory = "文件夹".to_string();
                                    name.insert(String::from("kind"), String::from("folder"));
                                } else {
                                    let extens: Vec<&str> = temp_name.split(".").collect();
                                    is_directory =
                                        extens[(extens.len() - 1)].to_string().to_uppercase()
                                            + " 文件";
                                    name.insert(String::from("kind"), String::from("file"));
                                }
                                name.insert(String::from("name"), temp_name);
                                let mut size: usize = 0;
                                let mut temp_size = "";
                                if let Some(temp_f) = temp.iter().nth(temp_len - 6) {
                                    temp_size = temp_f;
                                }
                                if temp_size != "0" && temp_size != "" {
                                    if let Ok(temp_f) = temp_size.parse::<usize>() {
                                        size = temp_f;
                                    }
                                }
                                let mut owner: String = String::from("");
                                if let Some(temp_f) = temp.iter().nth(2) {
                                    owner = temp_f.to_string();
                                }
                                let mut group: String = String::from("");
                                if let Some(temp_f) = temp.iter().nth(3) {
                                    group = temp_f.to_string();
                                }
                                let mut modified = String::from("");
                                if let Some(temp_f) = temp.iter().nth(temp_len - 4) {
                                    modified = modified + " " + temp_f;
                                }
                                if let Some(temp_f) = temp.iter().nth(temp_len - 3) {
                                    modified = modified + " " + temp_f;
                                }
                                if let Some(temp_f) = temp.iter().nth(temp_len - 2) {
                                    modified = modified + " " + temp_f;
                                }
                                file_list.push(FileList {
                                    permissions: perm,
                                    owner: owner,
                                    group: group,
                                    size: size,
                                    is_directory: is_directory,
                                    update_at: modified,
                                    name: name,
                                });
                            }
                            return Success::new(200, "获取成功！", file_list).out();
                        }
                        Err(err) => {
                            return Error::new(502, err).out();
                        }
                    }
                }
            }
        }
        Err(err) => {
            return Error::new(502, err).out();
        }
    }
    return Error::new(502, "获取失败！").out();
}

#[tauri::command]
pub fn folder_list(name: String) -> String {
    match OWNER_FTP_STREAM.lock() {
        Ok(mut s) => {
            if let Some(temp) = s.get_mut(&name) {
                if let Some(ftp_stream) = temp {
                    match ftp_stream.list(None) {
                        Ok(list) => {
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
                                let mut title = String::from("");
                                if let Some(temp_f) = temp.iter().nth(temp_len - 1) {
                                    title = temp_f.to_string();
                                }
                                folder_leaf.push(FolderLeaf {
                                    title: Some(title),
                                    key: String::from(
                                        String::from("0-") + &(num.clone().to_string()),
                                    ),
                                });
                                num += 1;
                            }
                            folder_tree.children = Some(folder_leaf);
                            return Success::new(200, "获取成功！", folder_tree).out();
                        }
                        Err(err) => {
                            return Error::new(502, err).out();
                        }
                    }
                }
            }
        }
        Err(err) => {
            return Error::new(502, err).out();
        }
    }
    return Error::new(502, "获取失败！").out();
}

#[tauri::command]
pub fn rename_file(name: String, from_name: String, to_name: String) -> String {
    match OWNER_FTP_STREAM.lock() {
        Ok(mut s) => {
            if let Some(temp) = s.get_mut(&name) {
                if let Some(ftp_stream) = temp {
                    match ftp_stream.rename(&from_name, &to_name) {
                        Ok(_) => {
                            return Success::new(200, "更改文件夹名称成功！", "").out();
                        }
                        Err(err) => {
                            return Error::new(502, err).out();
                        }
                    }
                }
            }
        }
        Err(err) => {
            return Error::new(502, err).out();
        }
    }
    return Error::new(502, "更改文件夹名称失败！").out();
}

#[tauri::command]
pub fn remove_file(name: String, filename: String) -> String {
    match OWNER_FTP_STREAM.lock() {
        Ok(mut s) => {
            if let Some(temp) = s.get_mut(&name) {
                if let Some(ftp_stream) = temp {
                    match ftp_stream.rm(&filename) {
                        Ok(_) => {
                            return Success::new(200, "删除文件成功！", ()).out();
                        }
                        Err(err) => {
                            return Error::new(502, err).out();
                        }
                    }
                }
            }
        }
        Err(err) => {
            return Error::new(502, err).out();
        }
    }
    return Error::new(502, "删除文件失败！").out();
}

#[tauri::command]
pub fn remove_dir(name: String, path: String) -> String {
    match OWNER_FTP_STREAM.lock() {
        Ok(mut s) => {
            if let Some(temp) = s.get_mut(&name) {
                if let Some(ftp_stream) = temp {
                    match ftp_stream.rmdir(&path) {
                        Ok(_) => {
                            return Success::new(
                                200,
                                String::from("删除文件夹") + &path + "成功！",
                                (),
                            )
                            .out();
                        }
                        Err(err) => {
                            return Error::new(502, err).out();
                        }
                    }
                }
            }
        }
        Err(err) => {
            return Error::new(502, err).out();
        }
    }
    return Error::new(502, "删除文件夹失败！").out();
}

#[tauri::command]
pub fn mk_dir(name: String, path: String) -> String {
    match OWNER_FTP_STREAM.lock() {
        Ok(mut s) => {
            if let Some(temp) = s.get_mut(&name) {
                if let Some(ftp_stream) = temp {
                    match ftp_stream.mkdir(&path) {
                        Ok(_) => {
                            return Success::new(
                                200,
                                String::from("创建文件夹") + &path + "成功！",
                                (),
                            )
                            .out();
                        }
                        Err(err) => {
                            return Error::new(502, err).out();
                        }
                    }
                }
            }
        }
        Err(err) => {
            return Error::new(502, err).out();
        }
    }
    return Error::new(502, "创建文件夹失败！").out();
}

#[tauri::command]
pub fn mk_file(name: String, filename: String) -> String {
    match OWNER_FTP_STREAM.lock() {
        Ok(mut s) => {
            if let Some(temp) = s.get_mut(&name) {
                if let Some(ftp_stream) = temp {
                    let mut b = "".as_bytes();
                    match ftp_stream.put(&filename, &mut b) {
                        Ok(_) => {
                            return Success::new(
                                200,
                                String::from("创建文件") + &filename + "成功！",
                                (),
                            )
                            .out();
                        }
                        Err(err) => {
                            return Error::new(502, err).out();
                        }
                    }
                }
            }
        }
        Err(err) => {
            return Error::new(502, err).out();
        }
    }
    return Error::new(502, "创建文件失败！").out();
}

#[tauri::command]
pub fn upload(name: String, filename: String, content: String) -> String {
    match OWNER_FTP_STREAM.lock() {
        Ok(mut s) => {
            if let Some(temp) = s.get_mut(&name) {
                if let Some(ftp_stream) = temp {
                    println!("{:?}", content);
                    match decode(content) {
                        Ok(temp) => {
                            let mut b = temp.as_slice();
                            match ftp_stream.put(&filename, &mut b) {
                                Ok(_) => {
                                    return Success::new(
                                        200,
                                        String::from("上传文件") + &filename + "成功！",
                                        (),
                                    )
                                    .out();
                                }
                                Err(err) => {
                                    return Error::new(502, err).out();
                                }
                            }
                        }
                        Err(err) => {
                            return Error::new(502, err).out();
                        }
                    }
                }
            }
        }
        Err(err) => {
            return Error::new(502, err).out();
        }
    }
    return Error::new(502, "上传文件失败！").out();
}

#[tauri::command]
pub fn quit(name: String) -> String {
    match OWNER_FTP_STREAM.lock() {
        Ok(mut s) => {
            if let Some(temp) = s.get_mut(&name) {
                if let Some(ftp_stream) = temp {
                    let _ = ftp_stream.quit();
                    return Success::new(200, "已断开连接", ()).out();
                }
            }
        }
        Err(err) => {
            return Error::new(502, err).out();
        }
    }
    return Error::new(502, "出现异常").out();
}

#[tauri::command]
pub fn size_sort(mut file_list: Option<Vec<FileList>>, sort_way: bool) -> String {
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
        return Success::new(200, "排序成功！", list.to_vec()).out();
    }
    return Success::new(200, "排序成功！", vec![""]).out();
}

