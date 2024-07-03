// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{AppHandle, EventTarget, Manager, Window};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn dispatch_event(handle: AppHandle) {
    println!("Dispatching test event to window with label 'main-1'");
    handle
        .emit_to(EventTarget::window("main-1"), "test-event", ())
        .unwrap();
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![dispatch_event])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
