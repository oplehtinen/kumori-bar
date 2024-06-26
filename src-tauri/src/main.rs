// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    os::windows::process::CommandExt,
    process::{Command, Output},
    time::Duration,
};
pub mod constants;
pub mod flags;
mod listener;
pub mod winplayer;
use crate::listener::komorebi_init_event_listener;
use constants::KOMOREBI_CLI_EXE;
use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};
use tokio::time::timeout;
use winplayer::{clplayer::ClPlayer, clplayermanager::ClPlayerManager, cltypes::ClStatus};
use winplayer::{player, playermanager::PlayerManager};

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
            set_komorebi_offset,
            get_player_status
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
    execute_komorebi_command("global-work-area-offset", &["0", offset, "0", offset]);
}

#[tauri::command]
async fn get_player_status() -> Result<(), ()> {
    //spawn a new thread to poll player events, so the main thread can continue. it needs to be async.
    println!("haloo");
    start_manager_loop().await;
    //let _ = tauri::async_runtime::spawn(async move { start_player_loop(player_manager) }).await;
    Ok(())
}
async fn start_manager_loop() {
    let mut player_manager: ClPlayerManager =
        ClPlayerManager::new(PlayerManager::new().await.unwrap());
    let mut player: Option<ClPlayer>;
    let mut aumid: String;
    println!("started loop");
    unsafe {
        loop {
            //println!("looping");
            let evt = player_manager.poll_next_event().await;
            match evt.as_str() {
                "ActiveSessionChanged" => {
                    //player = None;
                    player = player_manager.get_active_session();
                    if let Some(player) = player {
                        aumid = player.get_aumid().await;
                        let status = player.get_status().await;
                        println!("{:?}", status);
                        println!("{}", aumid);
                        start_player_loop(player).await;
                    } else {
                        println!("No active session");
                    }
                }
                "SystemSessionChanged" => {
                    player_manager.update_sessions(None);
                    let system_session = player_manager.get_system_session();
                    if let Some(player) = system_session {
                        aumid = player.get_aumid().await;
                    }
                }
                "SessionsChanged" => {
                    player_manager.update_sessions(None);
                    let system_session = player_manager.get_system_session();
                    if let Some(player) = system_session {
                        aumid = player.get_aumid().await;
                    }
                }
                _ => {}
            }

            // println!("{:?}", status);
        }
    }
}

async fn start_player_loop(mut player: ClPlayer) {
    unsafe {
        loop {
            let player_evt: String = player.poll_next_event().await;
            match player_evt.as_str() {
                "PlaybackInfoChanged" => {
                    let status = player.get_status().await;
                    println!("{:?}", status);
                }
                "MediaPropertiesChanged" => {
                    let status = player.get_status().await;
                    println!("{:?}", status);
                }
                "TimelinePropertiesChanged" => {
                    let pos = player.get_position(false).await;
                    println!("{:?}", pos);
                }
                _ => {}
            }
            println!("{}", player_evt);
        }
    }
}

#[tauri::command]
fn get_komorebi_status() -> String {
    let output: Output = execute_komorebi_command("state", &[]);
    String::from_utf8(output.stdout).unwrap()
}

#[tauri::command]
async fn switch_to_workspace(workspace: String, monitor: String) {
    execute_komorebi_command("mouse-follows-focus", &["disable"]);
    execute_komorebi_command(
        "focus-monitor-workspace",
        &[monitor.as_str(), workspace.as_str()],
    );
    execute_komorebi_command("mouse-follows-focus", &["enable"]);
}

fn execute_komorebi_command(command: &str, args: &[&str]) -> Output {
    let mut cmd = Command::new(KOMOREBI_CLI_EXE);
    cmd.arg(command);
    for arg in args {
        println!("{}", arg);
        cmd.arg(arg);
    }
    if !cfg!(debug_assertions) {
        cmd.creation_flags(flags::CREATE_NO_WINDOW);
    } else {
        // for some reason, creation_flags causes weird behavior in debug mode
    }

    let output = cmd.output();
    return output.expect("failed to execute process");
}
