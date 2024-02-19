// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::App;
use tauri::Manager;
use tauri::AppHandle;

#[derive(Clone, serde::Serialize)]
struct LogMessage {
  message: String,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn ready(app: AppHandle) {
    println!("Test");
    std::thread::spawn(move || {
        app.emit_all("log", LogMessage { message: "Hi".into() }).unwrap();
    });
}

#[tauri::command]
fn loadFile(app: AppHandle, file: String) {
    println!("{}", file);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![ready])
        .invoke_handler(tauri::generate_handler![loadFile])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
