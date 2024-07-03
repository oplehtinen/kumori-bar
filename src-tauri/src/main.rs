// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    os::windows::process::CommandExt,
    process::{Command, Output},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
pub mod constants;
pub mod flags;
mod listener;
mod player;
use crate::listener::komorebi_init_event_listener;
use constants::KOMOREBI_CLI_EXE;
use player::{next, play_pause, poll_manager_and_player_concurrently, previous, EvMetadata};
use tauri::{
    AppHandle, CustomMenuItem, Manager, State, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
};
use tokio::sync::Mutex;
static MANAGER_LOOP_RUNNING: AtomicBool = AtomicBool::new(false);
use winplayer_lib::clplayermanager::ClPlayerManager;
use winplayer_lib::playermanager::PlayerManager;
#[derive(Default)]
struct LastMetadata(Arc<Mutex<Option<EvMetadata>>>);
fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator);
    let system_tray = SystemTray::new().with_menu(tray_menu);
    tauri::Builder::default()
        .manage(LastMetadata::default())
        .invoke_handler(tauri::generate_handler![
            get_komorebi_status,
            switch_to_workspace,
            komorebi_init_event_listener,
            set_komorebi_offset,
            get_player_status,
            next,
            play_pause,
            previous
        ])
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
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
async fn get_player_status<'a>(
    app_handle: AppHandle,
    state: State<'a, LastMetadata>,
) -> Result<(), ()> {
    // Check if the manager loop is already running
    if MANAGER_LOOP_RUNNING
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .is_ok()
    {
        let metadata_clone = state.0.clone();
        tokio::spawn(async move {
            let player_manager: Arc<Mutex<PlayerManager>> = Arc::new(Mutex::new(
                PlayerManager::new().await.unwrap_or_else(|| {
                    eprintln!("Failed to create PlayerManager");
                    std::process::exit(1);
                }),
            ));
            let cl_player_manager: ClPlayerManager = ClPlayerManager::new(player_manager);

            // Start the manager loop
            // start_manager_loop(cl_player_manager, &app_handle).await;
            poll_manager_and_player_concurrently(cl_player_manager, &app_handle, metadata_clone)
                .await;

            // After the loop completes, reset the flag
            MANAGER_LOOP_RUNNING.store(false, Ordering::SeqCst);
        });
    } else {
        println!("Manager loop is already running.");
    }

    Ok(())
}

#[tauri::command]
fn get_komorebi_status() -> String {
    let output: Output = execute_komorebi_command("state", &[]);
    match String::from_utf8(output.stdout) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Error converting output to string: {}", e);
            String::new()
        }
    }
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
