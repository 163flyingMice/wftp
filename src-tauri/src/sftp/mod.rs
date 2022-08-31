use ssh2::{FileStat, Session, Sftp};
use std::collections::HashMap;
use std::net::TcpStream;
use std::path::Path;
use std::sync::Mutex;

use crate::remote::{FileList, FolderLeaf, FolderTree};
use crate::result::{Error, Success, CONNECTED_SUCCESS_CODE, DISCONNECTED_ERROR_CODE};
use crate::util::{get_format_perm, get_format_time, get_snow_id};

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
pub fn sftp_connect(addr: String, user: String, pass: String) -> String {
    match TcpStream::connect(addr) {
        Ok(tcp) => match Session::new() {
            Ok(mut sess) => {
                sess.set_tcp_stream(tcp);
                match sess.handshake() {
                    Ok(_) => match sess.userauth_password(&user, &pass) {
                        Ok(_) => match sess.sftp() {
                            Ok(sftp) => {
                                let sftp_struct = SftpStruct {
                                    sftp: Some(sftp),
                                    current_path: String::from("/"),
                                };
                                let snow_id = get_snow_id();
                                match SFTP_VEC.lock() {
                                    Ok(mut t) => {
                                        t.insert(snow_id.clone(), Some(sftp_struct));
                                        serde_json::to_string(&Success::new(
                                            CONNECTED_SUCCESS_CODE,
                                            String::from("连接成功！"),
                                            snow_id.clone(),
                                        ))
                                        .unwrap()
                                    }
                                    Err(err) => serde_json::to_string(&Error::new(
                                        DISCONNECTED_ERROR_CODE,
                                        err.to_string(),
                                    ))
                                    .unwrap(),
                                }
                            }
                            Err(err) => serde_json::to_string(&Error::new(
                                DISCONNECTED_ERROR_CODE,
                                err.to_string(),
                            ))
                            .unwrap(),
                        },
                        Err(err) => serde_json::to_string(&Error::new(
                            DISCONNECTED_ERROR_CODE,
                            err.to_string(),
                        ))
                        .unwrap(),
                    },
                    Err(err) => {
                        serde_json::to_string(&Error::new(DISCONNECTED_ERROR_CODE, err.to_string()))
                            .unwrap()
                    }
                }
            }
            Err(err) => serde_json::to_string(&Error::new(400, err.to_string())).unwrap(),
        },
        Err(err) => serde_json::to_string(&Error::new(400, err.to_string())).unwrap(),
    }
}

#[tauri::command]
pub fn readdir(name: String) -> String {
    match SFTP_VEC.lock() {
        Ok(mut t) => {
            if let Some(f) = t.get_mut(&name) {
                if let Some(s) = f {
                    if let Some(sftp) = s.sftp.as_mut() {
                        match sftp.readdir(Path::new(s.current_path.as_str())) {
                            Ok(result) => {
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
                                    let mut size: usize = 0;
                                    if let Some(temp) = file_stat.size {
                                        size = temp as usize;
                                    }
                                    let is_directory: String;
                                    let mut perm: String = String::from("");
                                    if file_stat.is_dir() {
                                        size = 0;
                                        perm += "d";
                                        is_directory = String::from("文件夹");
                                        name.insert(String::from("kind"), String::from("folder"));
                                    } else {
                                        is_directory = String::from("文件");
                                        let mut temp: String = String::from("不知命文件");
                                        if let Some(ext) = elem.0.clone().extension() {
                                            if let Some(extention) = ext.to_str() {
                                                temp = extention.to_string() + " 文件";
                                            }
                                        } else {
                                            temp = title.clone() + "文件";
                                        }
                                        name.insert(String::from("kind"), temp);
                                    }
                                    let mut path = String::from("/");
                                    if let Some(temp) = elem.0.to_str() {
                                        path = temp.to_string();
                                        if title == "" {
                                            title = String::from("未知文件命");
                                            let temp_name = temp
                                                .split("\\")
                                                .map(|s| s.to_string())
                                                .collect::<Vec<String>>();
                                            let temp_len = temp_name.len();
                                            if let Some(temp) = temp_name.iter().nth(temp_len - 1) {
                                                title = temp.to_string();
                                            }
                                        }
                                    }
                                    let mut owner = String::from("0");
                                    let mut group = String::from("0");
                                    if let Some(temp) = file_stat.uid {
                                        owner = temp.to_string();
                                    }
                                    if let Some(temp) = file_stat.gid {
                                        group = temp.to_string();
                                    }
                                    let mut mtime = 0;
                                    if let Some(temp) = file_stat.mtime {
                                        mtime = temp as i64;
                                    }
                                    let mut perm_extra = 000;
                                    if let Some(temp) = file_stat.perm {
                                        perm_extra = temp;
                                    }

                                    name.insert(String::from("name"), title);
                                    name.insert(String::from("path"), path);
                                    file_list.push(FileList {
                                        permissions: perm + &get_format_perm(perm_extra),
                                        owner: owner,
                                        group: group,
                                        size: size,
                                        update_at: get_format_time(mtime + 28800),
                                        is_directory: is_directory,
                                        name: name,
                                    })
                                }
                                return serde_json::to_string(&Success::new(
                                    200,
                                    String::from("获取成功！"),
                                    file_list,
                                ))
                                .unwrap();
                            }
                            Err(err) => {
                                return serde_json::to_string(&Error::new(400, err.to_string()))
                                    .unwrap();
                            }
                        }
                    }
                }
            }
        }
        Err(err) => {
            return serde_json::to_string(&Error::new(400, err.to_string())).unwrap();
        }
    }
    return serde_json::to_string(&Error::new(400, String::from("获取失败！"))).unwrap();
}

#[tauri::command]
pub fn sftp_rmdir(name: String, mut path: String) -> String {
    match SFTP_VEC.lock() {
        Ok(mut s) => {
            if let Some(temp) = s.get_mut(&name) {
                if let Some(sftp_temp) = temp {
                    if let Some(sftp) = sftp_temp.sftp.as_mut() {
                        if sftp_temp.current_path != "/" {
                            path = sftp_temp.current_path.as_mut().to_string() + "/" + &path;
                        } else {
                            path = sftp_temp.current_path.as_mut().to_string() + &path;
                        }
                        match sftp.rmdir(Path::new(&path)) {
                            Ok(_) => {
                                return serde_json::to_string(&Success::new(
                                    200,
                                    String::from("删除文件夹成功！"),
                                    (),
                                ))
                                .unwrap();
                            }
                            Err(err) => {
                                return serde_json::to_string(&Error::new(400, err.to_string()))
                                    .unwrap();
                            }
                        }
                    }
                }
            }
        }
        Err(err) => {
            return serde_json::to_string(&Error::new(400, err.to_string())).unwrap();
        }
    }
    return serde_json::to_string(&Error::new(502, String::from("删除文件夹失败！"))).unwrap();
}

#[tauri::command]
pub fn sftp_unlink(name: String, mut filename: String) -> String {
    match SFTP_VEC.lock() {
        Ok(mut s) => {
            if let Some(temp) = s.get_mut(&name) {
                if let Some(sftp_temp) = temp {
                    if let Some(sftp) = sftp_temp.sftp.as_mut() {
                        if sftp_temp.current_path != "/" {
                            filename =
                                sftp_temp.current_path.as_mut().to_string() + "/" + &filename;
                        } else {
                            filename = sftp_temp.current_path.as_mut().to_string() + &filename;
                        }
                        match sftp.unlink(Path::new(&filename)) {
                            Ok(_) => {
                                return serde_json::to_string(&Success::new(
                                    200,
                                    String::from("删除文件成功！"),
                                    (),
                                ))
                                .unwrap();
                            }
                            Err(err) => {
                                return serde_json::to_string(&Error::new(400, err.to_string()))
                                    .unwrap();
                            }
                        }
                    }
                }
            }
        }
        Err(err) => {
            return serde_json::to_string(&Error::new(400, err.to_string())).unwrap();
        }
    }
    return serde_json::to_string(&Error::new(502, String::from("删除文件失败！"))).unwrap();
}

#[tauri::command]
pub fn sftp_create(name: String, filename: String) -> String {
    match SFTP_VEC.lock() {
        Ok(mut s) => {
            if let Some(temp) = s.get_mut(&name) {
                if let Some(sftp_temp) = temp {
                    if let Some(sftp) = sftp_temp.sftp.as_mut() {
                        match sftp.create(Path::new(&filename)) {
                            Ok(_) => {
                                return serde_json::to_string(&Success::new(
                                    200,
                                    String::from("创建文件成功！"),
                                    (),
                                ))
                                .unwrap();
                            }
                            Err(err) => {
                                return serde_json::to_string(&Success::new(
                                    400,
                                    err.to_string(),
                                    (),
                                ))
                                .unwrap();
                            }
                        }
                    }
                }
            }
        }
        Err(err) => {
            return serde_json::to_string(&Error::new(400, err.to_string())).unwrap();
        }
    }
    return serde_json::to_string(&Error::new(502, String::from("创建文件失败！"))).unwrap();
}

#[tauri::command]
pub fn sftp_mkdir(name: String, path: String) -> String {
    match SFTP_VEC.lock() {
        Ok(mut s) => {
            if let Some(temp) = s.get_mut(&name) {
                if let Some(sftp_temp) = temp {
                    if let Some(sftp) = sftp_temp.sftp.as_mut() {
                        match sftp.mkdir(Path::new(&path), 0600) {
                            Ok(_) => {
                                return serde_json::to_string(&Success::new(
                                    200,
                                    String::from("创建文件夹成功！"),
                                    (),
                                ))
                                .unwrap();
                            }
                            Err(err) => {
                                return serde_json::to_string(&Error::new(400, err.to_string()))
                                    .unwrap();
                            }
                        }
                    }
                }
            }
        }
        Err(err) => {
            return serde_json::to_string(&Error::new(400, err.to_string())).unwrap();
        }
    }
    return serde_json::to_string(&Error::new(502, String::from("创建文件夹失败！"))).unwrap();
}

#[tauri::command]
pub fn sftp_rename(name: String, mut from_name: String, mut to_name: String) -> String {
    match SFTP_VEC.lock() {
        Ok(mut s) => {
            if let Some(temp) = s.get_mut(&name) {
                if let Some(sftp_temp) = temp {
                    if let Some(sftp) = sftp_temp.sftp.as_mut() {
                        if sftp_temp.current_path != "/" {
                            from_name =
                                sftp_temp.current_path.as_mut().to_string() + "/" + &from_name;
                            to_name = sftp_temp.current_path.as_mut().to_string() + "/" + &to_name;
                        } else {
                            from_name = sftp_temp.current_path.as_mut().to_string() + &from_name;
                            to_name = sftp_temp.current_path.as_mut().to_string() + &to_name;
                        }
                        match sftp.rename(Path::new(&from_name), Path::new(&to_name), None) {
                            Ok(_) => {
                                return serde_json::to_string(&Success::new(
                                    200,
                                    String::from("重命名成功！"),
                                    (),
                                ))
                                .unwrap();
                            }
                            Err(err) => {
                                return serde_json::to_string(&Error::new(400, err.to_string()))
                                    .unwrap();
                            }
                        }
                    }
                }
            }
        }
        Err(err) => {
            return serde_json::to_string(&Error::new(400, err.to_string())).unwrap();
        }
    }
    return serde_json::to_string(&Error::new(502, String::from("重命名失败！"))).unwrap();
}

#[tauri::command]
pub fn sftp_set_stat(name: String, filename: String) -> String {
    match SFTP_VEC.lock() {
        Ok(mut s) => {
            if let Some(temp) = s.get_mut(&name) {
                if let Some(sftp_temp) = temp {
                    if let Some(sftp) = sftp_temp.sftp.as_mut() {
                        match sftp.setstat(
                            Path::new(&filename),
                            FileStat {
                                size: None,
                                uid: None,
                                gid: None,
                                perm: None,
                                atime: None,
                                mtime: None,
                            },
                        ) {
                            Ok(_) => {
                                return serde_json::to_string(&Success::new(
                                    200,
                                    String::from("修改文件属性成功！"),
                                    (),
                                ))
                                .unwrap();
                            }
                            Err(err) => {
                                return serde_json::to_string(&Success::new(
                                    400,
                                    err.to_string(),
                                    (),
                                ))
                                .unwrap();
                            }
                        }
                    }
                }
            }
        }
        Err(err) => {
            return serde_json::to_string(&Success::new(400, err.to_string(), ())).unwrap();
        }
    }
    return serde_json::to_string(&Error::new(502, String::from("修改文件属性失败！"))).unwrap();
}

#[tauri::command]
pub fn sftp_folder_list(name: String) -> String {
    match SFTP_VEC.lock() {
        Ok(mut s) => {
            if let Some(temp) = s.get_mut(&name) {
                if let Some(sftp_temp) = temp {
                    if let Some(sftp) = sftp_temp.sftp.as_mut() {
                        match sftp.readdir(Path::new(&sftp_temp.current_path)) {
                            Ok(list) => {
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
                                    if title == None {
                                        title = Some(String::from("不知名资源"));
                                        if let Some(temp) = elem.0.to_str() {
                                            let temp_name = temp
                                                .split("\\")
                                                .map(|s| s.to_string())
                                                .collect::<Vec<String>>();
                                            let temp_len = temp_name.len();
                                            if let Some(temp) = temp_name.iter().nth(temp_len - 1) {
                                                title = Some(temp.to_string());
                                            }
                                        }
                                    }
                                    folder_leaf.push(FolderLeaf {
                                        title: title,
                                        key: String::from(
                                            String::from("0-") + &(num.clone().to_string()),
                                        ),
                                    });
                                    num += 1;
                                }
                                folder_tree.children = Some(folder_leaf);
                                return serde_json::to_string(&Success::new(
                                    200,
                                    String::from("获取成功！"),
                                    folder_tree,
                                ))
                                .unwrap();
                            }
                            Err(err) => {
                                return serde_json::to_string(&Success::new(
                                    400,
                                    err.to_string(),
                                    (),
                                ))
                                .unwrap();
                            }
                        }
                    }
                }
            }
        }
        Err(err) => {
            return serde_json::to_string(&Success::new(400, err.to_string(), ())).unwrap();
        }
    }
    return serde_json::to_string(&Error::new(400, String::from("获取失败！"))).unwrap();
}

#[tauri::command]
pub fn sftp_cwd(name: String, path: String) -> String {
    match SFTP_VEC.lock() {
        Ok(mut s) => {
            if let Some(temp) = s.get_mut(&name) {
                if let Some(sftp_temp) = temp {
                    if let Some(_) = sftp_temp.sftp.as_mut() {
                        if sftp_temp.current_path != "/" {
                            sftp_temp.current_path =
                                sftp_temp.current_path.as_mut().to_string() + "/" + &path;
                        } else {
                            sftp_temp.current_path =
                                sftp_temp.current_path.as_mut().to_string() + &path;
                        }
                        return serde_json::to_string(&Success::new(
                            200,
                            String::from("更改文件夹成功！"),
                            String::from(""),
                        ))
                        .unwrap();
                    }
                }
            }
        }
        Err(err) => {
            return serde_json::to_string(&Success::new(400, err.to_string(), ())).unwrap();
        }
    }
    return serde_json::to_string(&Error::new(400, String::from("更改文件夹失败！"))).unwrap();
}

#[tauri::command]
pub fn sftp_prev(name: String) -> String {
    match SFTP_VEC.lock() {
        Ok(mut s) => {
            if let Some(temp) = s.get_mut(&name) {
                if let Some(sftp_temp) = temp {
                    if let Some(_) = sftp_temp.sftp.as_mut() {
                        if sftp_temp.current_path != "/" {
                            let mut extens: Vec<&str> = sftp_temp.current_path.split("/").collect();
                            extens.pop();
                            let mut path = extens.join("/").to_string();
                            if path == "" {
                                path = String::from("/");
                            }
                            sftp_temp.current_path = path;
                        }
                        return serde_json::to_string(&Success::new(
                            200,
                            String::from("更改文件夹成功！"),
                            String::from(""),
                        ))
                        .unwrap();
                    }
                }
            }
        }
        Err(err) => {
            return serde_json::to_string(&Success::new(400, err.to_string(), ())).unwrap();
        }
    }
    return serde_json::to_string(&Error::new(400, String::from("更改文件夹失败！"))).unwrap();
}

#[tauri::command]
pub fn sftp_pwd(name: String) -> String {
    match SFTP_VEC.lock() {
        Ok(mut s) => {
            if let Some(temp) = s.get_mut(&name) {
                if let Some(sftp_temp) = temp {
                    if let Some(_) = sftp_temp.sftp.as_mut() {
                        return serde_json::to_string(&Success::new(
                            200,
                            String::from("获取成功！"),
                            sftp_temp.current_path.clone(),
                        ))
                        .unwrap();
                    }
                }
            }
        }
        Err(err) => {
            return serde_json::to_string(&Success::new(400, err.to_string(), ())).unwrap();
        }
    }
    return serde_json::to_string(&Error::new(400, String::from("获取失败！"))).unwrap();
}
