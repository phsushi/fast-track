// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::{Manager, webview::cookie::time::format_description::modifier::Padding::Space};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut};
mod commands;
fn main() {
    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![commands::ping])
    .plugin(tauri_plugin_global_shortcut::Builder::new().build())
    .setup(|app|{
        let shortcut = Shortcut::new(Some(Modifiers::ALT), Code::Space);

        app.global_shortcut().on_shortcut(shortcut, |app, _shortcut, _event| {
            let window = app.get_webview_window("main").unwrap();
            if window.is_visible().unwrap(){
                window.hide().unwrap();
            }else{
                window.show().unwrap();
                window.set_focus().unwrap();
            }
        })?;
        Ok(())
    })
    .run(tauri::generate_context!())
    .expect("Não foi possível iniciar o app!")
}
