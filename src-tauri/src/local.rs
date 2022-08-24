use chrono::prelude::*;
use std::{fs, time::UNIX_EPOCH};

#[tauri::command]
pub fn get_file_size(path: String) -> String {
    match fs::metadata(&path) {
        Ok(result) => result.len().to_string(),
        Err(err) => err.to_string(),
    }
}

#[tauri::command]
pub fn get_file_modified(path: String) -> String {
    match fs::metadata(&path) {
        Ok(result) => {
            let since_the_epoch = result
                .modified()
                .unwrap()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards");
            let ms = since_the_epoch.as_secs() as i64 * 1i64
                + 28800i64
                + (since_the_epoch.subsec_millis() as f64 / 1_000.0) as i64;
            let dt = Utc.timestamp(ms, 0);
            format!("{}", dt.format("%Y/%m/%d %H:%M:%S"))
        }
        Err(err) => err.to_string(),
    }
}
