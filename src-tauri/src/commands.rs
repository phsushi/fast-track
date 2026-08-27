#[tauri::command]

pub fn ping() -> String {
    "pong do Rust".to_string()
}
