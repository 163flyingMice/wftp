#![cfg_attr(
    all(not(debug_assertions), target_os = "window&s"),
    windows_subsystem = "windows"
)]
pub mod operate;
use operate::{connect, cwd, folder_list, list, prev, rename_file, try_connect};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            try_connect,
            list,
            cwd,
            connect,
            folder_list,
            prev,
            rename_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
