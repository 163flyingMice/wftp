use std::fs;

#[tauri::command]
pub fn get_file_size(path: String) -> String {
    match fs::metadata(&path) {
        Ok(result) => result.len().to_string(),
        Err(err) => err.to_string(),
    }
}
