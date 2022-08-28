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
use config::{get_default_wftp, get_wftp_server, wftp_xml_string};
use local::{get_file_modified, get_file_size};
use remote::{
    alive, connect, cwd, folder_list, list, mk_dir, mk_file, prev, pwd, quit, remove_dir,
    remove_file, rename_file, size_sort, try_connect, upload,
};
use tauri::generate_context;

fn main() {
    sftp::connect();
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
            size_sort
        ])
        .menu(menu::init())
        .on_menu_event(menu::handler)
        .run(generate_context!())
        .expect("error while running tauri application");
}
