// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;
use tauri::command;
mod listener;
use crate::listener::komorebi_init_event_listener;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_komorebi_status,
            switch_to_workspace,
            komorebi_init_event_listener
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_komorebi_status() -> String {
    let output = Command::new("komorebic")
        .arg("state")
        .output()
        .expect("failed to execute process");

    String::from_utf8(output.stdout).unwrap()
}

#[tauri::command]
fn switch_to_workspace(workspace: &str) {
    Command::new("komorebic")
        .arg("focus-named-workspace")
        .arg(workspace)
        .output()
        .expect("failed to execute process");
}
