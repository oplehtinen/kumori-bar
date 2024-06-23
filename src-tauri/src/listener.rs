pub const KOMOREBI_NAMED_PIPE: &str = r"\\.\pipe\bar-komorebi";
pub const APP_NAME: &str = "bar";
const KOMOREBI_CLI_EXE: &str = "komorebic";
const CREATE_NO_WINDOW: u32 = 0x08000000;
use ::windows::core::PCSTR;
use ::windows::Win32;

use std::os::windows::process::CommandExt;
use std::process::ExitStatus;
use std::process::{Command, Stdio};
use std::{ffi::c_void, sync::Once, thread, time::Duration};
use tauri::{AppHandle, Manager};
use Win32::{
    Foundation::{HANDLE, INVALID_HANDLE_VALUE},
    Storage::FileSystem::{ReadFile, PIPE_ACCESS_DUPLEX},
    System::Pipes::{
        ConnectNamedPipe, CreateNamedPipeA, DisconnectNamedPipe, NMPWAIT_USE_DEFAULT_WAIT,
        PIPE_READMODE_BYTE, PIPE_TYPE_BYTE, PIPE_WAIT,
    },
};

pub fn subscribe(pipe_name: &str) -> ExitStatus {
    let mut cmd = Command::new(KOMOREBI_CLI_EXE);
    cmd.arg("subscribe");
    cmd.arg(pipe_name);
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());
    cmd.creation_flags(CREATE_NO_WINDOW);
    cmd.spawn().unwrap().wait().unwrap()
}

pub fn unmanage_app_exe() -> ExitStatus {
    let mut cmd = Command::new(KOMOREBI_CLI_EXE);
    cmd.arg("float-rule");
    cmd.arg("exe");
    cmd.arg(format!("{}.exe", APP_NAME));
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());
    cmd.creation_flags(CREATE_NO_WINDOW);
    cmd.spawn().unwrap().wait().unwrap()
}

const PIPE_BUFFER: u32 = 1048576;
const PIPE_BUFFER_USIZE: usize = 1048576;
const KOMOREBI_INIT_ONCE: Once = Once::new();
const KOMOREBI_IGNORED_EVENTS: &'static [&'static str] =
    &["Cloak", "Uncloak", "AddSubscriber", "FocusChange", "State"];

fn create_named_pipe(pipe_name: &str) -> ::windows::core::Result<HANDLE> {
    log::info!("KomorebiEventListener: creating named pipe: {}.", pipe_name);
    unsafe {
        CreateNamedPipeA(
            PCSTR::from_raw(pipe_name.as_ptr() as *const u8),
            PIPE_ACCESS_DUPLEX,
            PIPE_TYPE_BYTE | PIPE_READMODE_BYTE | PIPE_WAIT,
            1,
            PIPE_BUFFER,
            PIPE_BUFFER,
            NMPWAIT_USE_DEFAULT_WAIT,
            None,
        )
    }
}

fn connect_named_pipe(named_pipe: HANDLE) -> bool {
    log::info!("KomorebiEventListener: Connecting named pipe.");
    unsafe { ConnectNamedPipe(named_pipe, None).as_bool() }
}

fn disconnect_named_pipe(named_pipe: HANDLE) -> () {
    log::warn!("KomorebiEventListener: Disconnecting named pipe.");
    unsafe {
        DisconnectNamedPipe(named_pipe);
    }
}

fn read_named_pipe(named_pipe: HANDLE, data_buffer: &mut [u8; PIPE_BUFFER_USIZE]) -> bool {
    unsafe {
        ReadFile(
            named_pipe,
            Some(data_buffer.as_mut_ptr() as *mut c_void),
            PIPE_BUFFER,
            None,
            None,
        )
        .as_bool()
    }
}

fn poll_named_pipe(named_pipe: HANDLE, app_handle: &AppHandle) -> () {
    let mut data_buffer = [0; PIPE_BUFFER_USIZE];

    while read_named_pipe(named_pipe, &mut data_buffer) {
        if let Ok(json_string) = String::from_utf8(data_buffer.to_vec()) {
            let notification_msgs = json_string.trim().trim_matches(char::from(0)).to_string();

            for notification in notification_msgs.split("\n") {
                if notification.len() > 0 {
                    let payload_value: serde_json::Value =
                        serde_json::from_str(notification).unwrap();
                    let event_type: String =
                        payload_value["event"]["type"].to_string().replace("\"", "");

                    if KOMOREBI_IGNORED_EVENTS.contains(&event_type.as_str()) {
                        continue;
                    };

                    let komorebi_event = format!("Komorebi{}", event_type);
                    let payload = payload_value["event"]["payload"].clone();
                    log::info!("KomorebiEventListener: emitting event: {}", komorebi_event);
                    let _ = app_handle.emit_all("komorebi_status", payload);
                }
            }
        }

        data_buffer = [0; PIPE_BUFFER_USIZE];
    }
}

fn wait_until_subscribed(pipe_name: &str) -> () {
    let mut is_first_try = true;

    log::info!("KomorebiEventListener: waiting to subscribe to named pipe.");

    while subscribe(pipe_name).code().unwrap_or(1) != 0 {
        if is_first_try {
            log::warn!("KomorebiEventListener: subscribe failed. Retrying in 1s indefinitely... Is komorebic.exe added to System PATH?");
            is_first_try = false;
        }

        thread::sleep(Duration::from_secs(1));
    }

    log::info!("KomorebiEventListener: successfully subscribed to named pipe.");

    if unmanage_app_exe().code().unwrap_or(1) == 0 {
        log::info!(
            "KomorebiEventListener: added ignore rule for process {}.exe.",
            APP_NAME
        );
    } else {
        log::warn!(
            "KomorebiEventListener: Failed to add ignore rule for process {}.exe.",
            APP_NAME
        );
    }
}

fn komorebi_event_listener(app_handle: &AppHandle) -> () {
    match create_named_pipe(KOMOREBI_NAMED_PIPE) {
        Ok(named_pipe) => {
            let (_, pipe_name) = KOMOREBI_NAMED_PIPE.rsplit_once('\\').unwrap();

            while named_pipe != INVALID_HANDLE_VALUE {
                let _app_handle = app_handle.clone();
                let pipe_thread: thread::JoinHandle<()> = thread::spawn(move || {
                    if connect_named_pipe(named_pipe) {
                        log::info!(
                            "KomorebiEventListener: pipe connected. Listening for komorebi events."
                        );
                        poll_named_pipe(named_pipe, &_app_handle);
                    } else {
                        log::warn!("KomorebiEventListener: failed to connect named pipe. Disconnecting and trying again.");
                        disconnect_named_pipe(named_pipe);
                        let _ = _app_handle.emit_all("KomorebiOffline", ());
                    }
                });

                thread::sleep(Duration::from_millis(500));
                wait_until_subscribed(pipe_name);
                let _ = app_handle.emit_all("KomorebiOnline", ());
                pipe_thread.join().unwrap();
            }
        }
        Err(e) => log::error!("{}", e.to_string()),
    }
}

#[tauri::command]
pub fn komorebi_init_event_listener(app_handle: AppHandle) -> () {
    println!("haloo");
    KOMOREBI_INIT_ONCE.call_once(move || {
        log::info!("KomorebiEventListener: Initialisng Event Listener.");

        tauri::async_runtime::spawn(async move {
            komorebi_event_listener(&app_handle);
        });
    });
}
