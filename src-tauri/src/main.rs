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
use log::{error, info};
use player::{next, play_pause, poll_manager_and_player_concurrently, previous, EvMetadata};
use tauri::{
    menu::{MenuBuilder, PredefinedMenuItem},
    tray::TrayIconBuilder,
    AppHandle, Manager, PhysicalSize, State,
};
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

static MANAGER_LOOP_RUNNING: AtomicBool = AtomicBool::new(false);
use enigo::{
    Direction::{Press, Release},
    Enigo, Key, Keyboard, Settings,
};
use winplayer_lib::clplayermanager::ClPlayerManager;
use winplayer_lib::playermanager::PlayerManager;
#[derive(Default)]
struct LastMetadata(Arc<Mutex<Option<EvMetadata>>>);

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    #[cfg(debug_assertions)] // only enable instrumentation in development builds
    let devtools = tauri_plugin_devtools::init();
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_oauth::init())
        .setup(|app| {
            let quit_p = PredefinedMenuItem::quit(app, Some("Quit"))?;
            let menu = MenuBuilder::new(app).items(&[&quit_p]).build()?;

            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .on_menu_event(move |_app, event| match event.id().as_ref() {
                    "quit" => {
                        info!("unhandled event {event:?}");
                    }
                    _ => {}
                })
                .icon(app.default_window_icon().unwrap().clone())
                .build(app)?;

            let Some(window) = app.get_webview_window("main") else {
                return Ok(());
            };
            // let monitors = app.available_monitors();
            let Ok(Some(monitor)) = app.primary_monitor() else {
                return Ok(());
            };
            let size = monitor.size();
            let monitor_width = size.width;
            let new_size = PhysicalSize {
                width: monitor_width,
                height: 80,
            };
            let _ = window.set_size(new_size);
            let _ = window.set_position(tauri::PhysicalPosition::new(0, 0));
            Ok(())
        })
        .manage(LastMetadata::default())
        .invoke_handler(tauri::generate_handler![
            get_komorebi_status,
            switch_to_workspace,
            komorebi_init_event_listener,
            set_komorebi_offset,
            get_player_status,
            next,
            play_pause,
            previous,
            simulate_windows_tab
        ]);
    #[cfg(debug_assertions)]
    {
        builder = builder.plugin(devtools);
    }

    builder
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
                    error!("Failed to create PlayerManager");
                    std::process::exit(1);
                }),
            ));
            let cl_player_manager: ClPlayerManager = ClPlayerManager::new(player_manager);

            info!("Starting manager loop");
            poll_manager_and_player_concurrently(cl_player_manager, &app_handle, metadata_clone)
                .await;
            // After the loop completes, reset the flag
            MANAGER_LOOP_RUNNING.store(false, Ordering::SeqCst);
        });
    } else {
        info!("Manager loop is already running.");
    }

    Ok(())
}

#[tauri::command]
async fn get_komorebi_status() -> String {
    let output = tokio::time::timeout(Duration::from_secs(5), async {
        execute_komorebi_command("state", &[])
    })
    .await;

    match output {
        Ok(output) => match String::from_utf8(output.stdout) {
            Ok(result) => result,
            Err(e) => {
                error!("Error converting output to string: {}", e);
                String::new()
            }
        },
        Err(_) => {
            error!("Timeout while waiting for komorebi command to complete");
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
        info!("{}", arg);
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
#[tauri::command]
async fn simulate_windows_tab() {
    sleep(Duration::from_millis(100)).await;
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.key(Key::Meta, Press).unwrap();
    enigo.key(Key::Tab, Press).unwrap();
    sleep(Duration::from_millis(100)).await;
    enigo.key(Key::Meta, Release).unwrap();
    enigo.key(Key::Tab, Release).unwrap();
}
