use ::std::sync::Arc;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{AppHandle, Manager};
use tokio::sync::{Mutex, Notify};
use tokio::time::timeout;
use WinPlayer_Rust::clplayer::ClPlayer;
use WinPlayer_Rust::clplayermanager::ClPlayerManager;
#[derive(Serialize, Deserialize)]
pub struct EvArtData {
    pub data: Vec<u8>,
    pub mimetype: String,
}
#[derive(Serialize, Deserialize)]
pub struct EvMetadata {
    pub album: Option<String>,
    pub album_artist: Option<String>,
    pub album_artists: Option<Vec<String>>,
    pub artist: String,
    pub artists: Vec<String>,
    pub art_data: Option<EvArtData>, // Ensure ClArtData also derives Serialize
    pub id: Option<String>,
    pub length: f64,
    pub title: String,
}
pub async fn start_manager_loop(mut player_manager: ClPlayerManager, app_handle: &AppHandle) {
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
                    println!("Active session: {:?}", aumid);
                    let status = player.get_status().await;
                    let metadata = status.metadata.unwrap();
                    let payload = metadata_to_json(metadata);
                    let _ = app_handle.emit_all("player_status", payload);
                    // println!("{:?}", metadata.title);
                    //println!("{:?}", status);

                    stop_flag.store(false, Ordering::Relaxed);
                    let stop_flag_clone = stop_flag.clone();
                    let notify_clone = notify.clone();
                    let app_handle_clone = app_handle.clone();
                    tauri::async_runtime::spawn(async move {
                        start_player_loop(player, stop_flag_clone, notify_clone, &app_handle_clone)
                            .await;
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
                    let app_handle_clone = app_handle.clone();
                    aumid = player.get_aumid().await;
                    println!("Active session: {:?}", aumid);
                    tauri::async_runtime::spawn(async move {
                        start_player_loop(player, stop_flag_clone, notify_clone, &app_handle_clone)
                            .await;
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
                    println!("Active session: {:?}", aumid);
                    let status = player.get_status().await;
                    // println!("{:?}", status);
                }
            }
            _ => {} // println!("{:?}", status);
        }
    }
}

pub async fn start_player_loop(
    mut player: ClPlayer,
    stop_flag: Arc<AtomicBool>,
    notify: Arc<Notify>,
    app_handle: &AppHandle,
) {
    loop {
        println!("looping");
        if stop_flag.load(Ordering::Relaxed) {
            println!("Stopping player loop");
            break;
        }
        let player_evt: String = player.poll_next_event().await;
        match player_evt.as_str() {
            "PlaybackInfoChanged" => {
                println!("Playback info changed");
                let status = player.get_status().await;
                //println!("{:?}", status);
            }
            "MediaPropertiesChanged" => {
                println!("Media properties changed");
                let status = player.get_status().await;
                let payload_json = metadata_to_json(status.metadata.unwrap());
                let _ = app_handle.emit_all("player_status", payload_json);
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

fn metadata_to_json(metadata: WinPlayer_Rust::cltypes::ClMetadata) -> Value {
    let payload = EvMetadata {
        album: metadata.album,
        album_artist: metadata.album_artist,
        album_artists: metadata.album_artists,
        artist: metadata.artist,
        artists: metadata.artists,
        art_data: metadata.art_data.map(|art_data| EvArtData {
            data: art_data.data,
            mimetype: art_data.mimetype,
        }),
        id: metadata.id,
        length: metadata.length,
        title: metadata.title,
    };
    let json = serde_json::to_value(&payload).unwrap();
    return json;
}
