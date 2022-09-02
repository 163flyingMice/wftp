#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate lazy_static;
pub mod config;
pub mod local;
pub mod menu;
pub mod remote;
pub mod result;
pub mod sftp;
pub mod util;
use std::{collections::HashMap, sync::Mutex, thread};

use config::{get_default_wftp, get_wftp_server, wftp_xml_string};
use local::{get_file_modified, get_file_size};
use remote::{
    alive, connect, cwd, folder_list, list, mk_dir, mk_file, prev, pwd, quit, remove_dir,
    remove_file, rename_file, size_sort, try_connect, upload,
};
use tauri::generate_context;
use util::queue::{MyQueue, Queue};

use crate::sftp::{
    readdir, sftp_connect, sftp_create, sftp_cwd, sftp_dir_download, sftp_download,
    sftp_folder_list, sftp_mkdir, sftp_prev, sftp_pwd, sftp_rename, sftp_rmdir, sftp_unlink,
    sftp_upload,
};

lazy_static! {
    static ref QUEUE: Mutex<HashMap<String, MyQueue<String>>> = {
        let map: HashMap<String, MyQueue<String>> = HashMap::new();
        Mutex::new(map)
    };
}

fn main() {
    thread::spawn(move || loop {
        match QUEUE.lock() {
            Ok(mut map) => {
                for (_, v) in map.iter_mut() {
                    while let Some(v) = v.dequeue() {
                        println!("{v}");
                    }
                }
            }
            Err(err) => {
                println!("{}", err);
            }
        }
    });
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            try_connect,
            list,
            cwd,
            connect,
            folder_list,
            prev,
            rename_file,
            remove_file,
            remove_dir,
            mk_dir,
            pwd,
            mk_file,
            upload,
            alive,
            get_wftp_server,
            get_default_wftp,
            wftp_xml_string,
            get_file_size,
            get_file_modified,
            quit,
            size_sort,
            readdir,
            sftp_connect,
            sftp_folder_list,
            sftp_prev,
            sftp_cwd,
            sftp_pwd,
            sftp_rename,
            sftp_mkdir,
            sftp_create,
            sftp_rmdir,
            sftp_unlink,
            sftp_upload,
            sftp_download,
            sftp_dir_download,
        ])
        .menu(menu::init())
        .on_menu_event(menu::handler)
        .run(generate_context!())
        .expect("error while running tauri application");
}
