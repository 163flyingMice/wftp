use chrono::{TimeZone, Utc};
use ssh2::{FileStat, Session, Sftp};
use std::collections::HashMap;
use std::net::TcpStream;
use std::path::Path;
use std::sync::Mutex;

use crate::remote::{FileList, FolderLeaf, FolderTree};
use crate::result::{CustomError, Error, Success, CONNECTED_SUCCESS_CODE, DISCONNECTED_ERROR_CODE};

pub struct SftpStruct {
    sftp: Option<Sftp>,
    current_path: String,
}

lazy_static! {
    static ref SFTP_VEC: Mutex<HashMap<String, Option<SftpStruct>>> = {
        let map: HashMap<String, Option<SftpStruct>> = HashMap::new();
        Mutex::new(map)
    };
}

#[tauri::command]
pub fn sftp_connect(name: String, addr: String, user: String, pass: String) -> String {
    let tcp = TcpStream::connect(addr).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    if let Ok(_) = sess.userauth_password(&user, &pass) {
        let sftp = sess.sftp().unwrap();
        let sftp_struct = SftpStruct {
            sftp: Some(sftp),
            current_path: String::from("/"),
        };
        SFTP_VEC
            .lock()
            .unwrap()
            .insert(name.to_string(), Some(sftp_struct));
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
pub fn readdir(name: String) -> Option<Vec<FileList>> {
    if let Some(sftp) = SFTP_VEC.lock().unwrap().get_mut(&name).unwrap() {
        let result = sftp
            .sftp
            .as_mut()
            .unwrap()
            .readdir(Path::new(sftp.current_path.as_str()))
            .unwrap();
        let mut file_list = vec![FileList::new()];
        for elem in result {
            let file_stat = elem.1.clone();
            let mut name = HashMap::new();
            let mut title: String = String::from("");
            if let Some(filename_temp) = elem.0.clone().file_name() {
                if let Some(filename) = filename_temp.to_str() {
                    title = filename.to_string();
                }
            }
            let mut size = file_stat.size.unwrap() as usize;
            let is_directory: String;
            if file_stat.is_dir() {
                size = 0;
                is_directory = String::from("文件夹");
                name.insert(String::from("kind"), String::from("folder"));
            } else {
                is_directory = String::from("文件");
                let temp: String;
                if let Some(ext) = elem.0.clone().extension() {
                    temp = ext.to_str().unwrap().to_string() + " 文件";
                } else {
                    temp = title.clone() + "文件";
                }
                name.insert(String::from("kind"), temp);
            }
            if title == "" {
                let temp_name = elem
                    .0
                    .to_str()
                    .unwrap()
                    .split("\\")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();
                let temp_len = temp_name.len();
                title = temp_name.iter().nth(temp_len - 1).unwrap().to_string();
            }
            name.insert(String::from("name"), title);
            name.insert(
                String::from("path"),
                elem.0.clone().to_str().unwrap().to_string(),
            );
            file_list.push(FileList {
                permissions: file_stat.perm.unwrap().to_string(),
                owner: file_stat.uid.unwrap().to_string(),
                group: file_stat.gid.unwrap().to_string(),
                size: size,
                update_at: get_format_time(file_stat.mtime.unwrap() as i64 + 28800),
                is_directory: is_directory,
                name: name,
            })
        }
        Some(file_list)
    } else {
        None
    }
}

fn get_format_time(ms: i64) -> String {
    let dt = Utc.timestamp(ms, 0);
    format!("{}", dt.format("%Y/%m/%d %H:%M:%S"))
}

#[tauri::command]
pub fn rmdir(name: String, path: String) -> String {
    if let Some(sftp) = SFTP_VEC.lock().unwrap().get_mut(&name).unwrap() {
        let _ = sftp.sftp.as_mut().unwrap().rmdir(Path::new(&path)).unwrap();
        return serde_json::to_string(&Success::new(200, String::from("删除文件夹成功！"), ()))
            .unwrap();
    } else {
        return serde_json::to_string(&Error::new(502, String::from("删除文件夹失败！"))).unwrap();
    }
}

#[tauri::command]
pub fn unlink(name: String, path: String) -> String {
    if let Some(sftp) = SFTP_VEC.lock().unwrap().get_mut(&name).unwrap() {
        let _ = sftp
            .sftp
            .as_mut()
            .unwrap()
            .unlink(Path::new(&path))
            .unwrap();
        return serde_json::to_string(&Success::new(200, String::from("删除文件成功！"), ()))
            .unwrap();
    } else {
        return serde_json::to_string(&Error::new(502, String::from("删除文件失败！"))).unwrap();
    }
}

#[tauri::command]
pub fn create(name: String, filename: String) -> String {
    if let Some(sftp) = SFTP_VEC.lock().unwrap().get_mut(&name).unwrap() {
        let _ = sftp.sftp.as_mut().unwrap().create(Path::new(&filename));
        return serde_json::to_string(&Success::new(200, String::from("创建文件成功！"), ()))
            .unwrap();
    } else {
        return serde_json::to_string(&Error::new(502, String::from("创建文件失败！"))).unwrap();
    }
}

#[tauri::command]
pub fn mkdir(name: String, filename: String) -> String {
    if let Some(sftp) = SFTP_VEC.lock().unwrap().get_mut(&name).unwrap() {
        let _ = sftp
            .sftp
            .as_mut()
            .unwrap()
            .mkdir(Path::new(&filename), 0600);
        return serde_json::to_string(&Success::new(200, String::from("创建文件夹成功！"), ()))
            .unwrap();
    } else {
        return serde_json::to_string(&Error::new(502, String::from("创建文件夹失败！"))).unwrap();
    }
}

#[tauri::command]
pub fn sftp_rename(name: String, from_name: String, to_name: String) -> String {
    if let Some(sftp) = SFTP_VEC.lock().unwrap().get_mut(&name).unwrap() {
        let _ =
            sftp.sftp
                .as_mut()
                .unwrap()
                .rename(Path::new(&from_name), Path::new(&to_name), None);
        return serde_json::to_string(&Success::new(200, String::from("重命名成功！"), ()))
            .unwrap();
    } else {
        return serde_json::to_string(&Error::new(502, String::from("重命名失败！"))).unwrap();
    }
}

#[tauri::command]
pub fn sftp_set_stat(name: String, filename: String) -> String {
    if let Some(sftp) = SFTP_VEC.lock().unwrap().get_mut(&name).unwrap() {
        let _ = sftp.sftp.as_mut().unwrap().setstat(
            Path::new(&filename),
            FileStat {
                size: None,
                uid: None,
                gid: None,
                perm: None,
                atime: None,
                mtime: None,
            },
        );
        return serde_json::to_string(&Success::new(200, String::from("修改文件属性成功！"), ()))
            .unwrap();
    } else {
        return serde_json::to_string(&Error::new(502, String::from("修改文件属性失败！")))
            .unwrap();
    }
}

#[tauri::command]
pub fn sftp_folder_list(name: String) -> Option<FolderTree> {
    if let Some(temp) = SFTP_VEC.lock().unwrap().get_mut(&name) {
        if let Some(sftp) = temp {
            let list = sftp
                .sftp
                .as_mut()
                .unwrap()
                .readdir(Path::new(&sftp.current_path))
                .expect("获取列表失败！");
            let mut folder_tree = FolderTree::new();
            let mut folder_leaf = Vec::new();
            let mut num = 0;
            for elem in &list {
                let mut title: Option<String> = None;
                if let Some(filename_temp) = elem.0.file_name() {
                    if let Some(filename) = filename_temp.to_str() {
                        title = Some(filename.to_string());
                    }
                }
                folder_leaf.push(FolderLeaf {
                    title: title,
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
pub fn sftp_cwd(name: String, path: String) -> String {
    if let Some(temp) = SFTP_VEC.lock().unwrap().get_mut(&name) {
        if let Some(sftp) = temp {
            sftp.current_path = sftp.current_path.as_mut().to_string() + "/" + &path;
            return String::from("更改文件夹成功！");
        }
    }
    String::from("更改文件夹失败！")
}

#[tauri::command]
pub fn sftp_prev(name: String) -> String {
    if let Some(temp) = SFTP_VEC.lock().unwrap().get_mut(&name) {
        if let Some(sftp) = temp {
            let mut extens: Vec<&str> = sftp.current_path.split("/").collect();
            extens.pop();
            let path = extens.join("/").to_string();
            sftp.current_path = path;
            return String::from("更改文件夹成功！");
        }
    }
    String::from("更改文件夹失败！")
}
