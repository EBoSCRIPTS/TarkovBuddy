// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::fs::OpenOptions;


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![storePrice])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn storePrice(path: String, contents: String){
    println!("{}", path);
    let mut history_file = OpenOptions::new()
        .append(true)
        .open(path + "history.txt")
        .expect("File not found!");

    history_file.write(contents.as_bytes()).expect("Unable to write to file");
}