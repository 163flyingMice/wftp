#![cfg_attr(
    all(not(debug_assertions), target_os = "window&s"),
    windows_subsystem = "windows"
)]

pub mod config;
pub mod operate;
use config::{get_wftp_server, insert_wftp_server};
use operate::{
    alive, connect, cwd, folder_list, list, mk_dir, mk_file, prev, pwd, remove_dir, remove_file,
    rename_file, try_connect, upload,
};

fn main() {
    insert_wftp_server();
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
