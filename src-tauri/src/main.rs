// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use native_dialog::FileDialog;
use tauri::Manager;
use std::sync::Mutex;

#[macro_use]
extern crate lazy_static;

// define file and folder path variable, don't know if it's the right way of doing it
lazy_static! {
    static ref FILE_PATH: Mutex<String> = Mutex::new("".to_string());
    static ref FOLDER_PATH: Mutex<String> = Mutex::new("".to_string());
}

#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn select_file_button(app: tauri::AppHandle) {
    FILE_PATH.lock().unwrap().replace_range(.., &choose_file());
    println!("{}",FILE_PATH.lock().unwrap());
    let _ = app.emit_all("file_path_changed", Payload { message: FILE_PATH.lock().unwrap().to_string() });
}

#[tauri::command]
async fn select_folder_button(app: tauri::AppHandle) {
    FOLDER_PATH.lock().unwrap().replace_range(.., &choose_folder());
    println!("{}",FOLDER_PATH.lock().unwrap());
    let _ = app.emit_all("folder_path_changed", Payload { message: FOLDER_PATH.lock().unwrap().to_string() });
}

#[tauri::command]
fn debug_call(message: &str){
    println!("[DBG] {}", message);
}

// prompt user file chooser using native_dialogue crate
fn choose_file() -> String{
    println!("Let's choose a file !");
    let path = FileDialog::new()
        .show_open_single_file()
        .unwrap();
    format!("{:?}", path).replace("Some(\"", "").replace("\")", "") // turn the FileDialog into a string and remove Some("")
}

fn choose_folder() -> String{
    println!("Let's choose a folder !");
    let path = FileDialog::new()
        .show_open_single_dir()
        .unwrap();
    format!("{:?}", path).replace("Some(\"", "").replace("\")", "") // turn the FileDialog into a string
}

fn main() {
    // generate the tauri app
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![select_file_button, select_folder_button, debug_call])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
