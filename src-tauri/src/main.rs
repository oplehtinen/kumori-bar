// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;
mod listener;
use crate::listener::komorebi_init_event_listener;
use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide);
    let system_tray = SystemTray::new().with_menu(tray_menu);
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_komorebi_status,
            switch_to_workspace,
            komorebi_init_event_listener,
            set_komorebi_offset
        ])
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "hide" => {
                    let window = app.get_window("main").unwrap();
                    window.hide().unwrap();
                }
                _ => {}
            },
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn set_komorebi_offset(offset: &str) {
    Command::new("komorebic")
        .arg("global-work-area-offset")
        .arg("0")
        .arg(offset)
        .arg("0")
        .arg(offset)
        .output()
        .expect("failed to execute process");
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
fn switch_to_workspace(workspace: &str, monitor: &str) {
    Command::new("komorebic")
        .arg("mouse-follows-focus")
        .arg("disable")
        .output()
        .expect("failed to execute process");
    // wait for the command to finish
    //std::thread::sleep(std::time::Duration::from_millis(50));
    Command::new("komorebic")
        .arg("focus-monitor-workspace")
        .arg(monitor)
        .arg(workspace)
        .output()
        .expect("failed to execute process");
    // std::thread::sleep(std::time::Duration::from_millis(50));
    Command::new("komorebic")
        .arg("mouse-follows-focus")
        .arg("enable")
        .output()
        .expect("failed to execute process");
}
