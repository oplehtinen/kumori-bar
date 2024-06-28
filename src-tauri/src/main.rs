// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    os::windows::process::CommandExt,
    process::{Command, Output},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
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
use tokio::sync::{Mutex, Notify};
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
    tokio::spawn(async move {
        let player_manager: Arc<Mutex<PlayerManager>> =
            Arc::new(Mutex::new(PlayerManager::new().await.unwrap()));
        let cl_player_manager: ClPlayerManager = ClPlayerManager::new(player_manager);
        start_manager_loop(cl_player_manager).await
    });
    //let _ = tauri::async_runtime::spawn(async move { start_player_loop(player_manager) }).await;
    Ok(())
}
async fn start_manager_loop(mut player_manager: ClPlayerManager) {
    let mut player: Option<ClPlayer>;
    let mut aumid: String;
    let stop_flag = Arc::new(AtomicBool::new(false));
    let notify = Arc::new(Notify::new());
    println!("started loop");
    loop {
        println!("looping manager");
        let evt = player_manager.poll_next_event().await;

        match evt.as_str() {
            "ActiveSessionChanged" => {
                //player = None;
                println!("Active session changed");
                player = player_manager.get_active_session().await;
                if let Some(player) = player {
                    aumid = player.get_aumid().await;
                    let status = player.get_status().await;
                    //println!("{:?}", status);

                    stop_flag.store(false, Ordering::Relaxed);
                    let stop_flag_clone = stop_flag.clone();
                    let notify_clone = notify.clone();
                    tauri::async_runtime::spawn(async move {
                        start_player_loop(player, stop_flag_clone, notify_clone).await;
                    });
                    println!("Started player loop");
                } else {
                    println!("No active session");
                }
            }
            "SystemSessionChanged" => {
                println!("System session changed");
                let _ = player_manager.update_system_session().await;
                let system_session = player_manager.get_system_session().await;
                if let Some(player) = system_session {
                    stop_flag.store(true, Ordering::Relaxed);
                    // notify the player loop to stop
                    notify.notify_one();
                    println!("Stopped player loop");
                    stop_flag.store(false, Ordering::Relaxed);
                    let stop_flag_clone = stop_flag.clone();
                    let notify_clone = notify.clone();
                    aumid = player.get_aumid().await;
                    tauri::async_runtime::spawn(async move {
                        start_player_loop(player, stop_flag_clone, notify_clone).await;
                    });

                    //let status = player.get_status().await;
                    // println!("{:?}", status);
                }
            }
            "SessionsChanged" => {
                println!("Sessions changed");
                player_manager.update_sessions(None).await;
                let system_session = player_manager.get_system_session().await;
                if let Some(player) = system_session {
                    // send the stop signal to the player loop
                    stop_flag.store(true, Ordering::Relaxed);
                    // notify the player loop to stop
                    notify.notify_one();
                    println!("Stopped player loop");
                    aumid = player.get_aumid().await;
                    let status = player.get_status().await;
                    // println!("{:?}", status);
                }
            }
            _ => {} // println!("{:?}", status);
        }
    }
}

async fn start_player_loop(mut player: ClPlayer, stop_flag: Arc<AtomicBool>, notify: Arc<Notify>) {
    loop {
        println!("looping");
        if stop_flag.load(Ordering::Relaxed) {
            println!("Stopping player loop");
            break;
        }
        let player_evt: String = player.poll_next_event().await;
        match player_evt.as_str() {
            "PlaybackInfoChanged" => {
                let status = player.get_status().await;
                //println!("{:?}", status);
            }
            "MediaPropertiesChanged" => {
                let status = player.get_status().await;
                //println!("{:?}", status);
            }
            "TimelinePropertiesChanged" => {
                let pos = player.get_position(false).await;
                println!("{:?}", pos);
            }
            _ => {}
        }
        println!("we got here");
        //notify.notified().await;
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
