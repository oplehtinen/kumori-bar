use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::{AppHandle, Emitter, State};
use tokio::sync::Mutex;
use tokio::time::Instant;
use windows::Media::Control::GlobalSystemMediaTransportControlsSessionPlaybackStatus;
use winplayer_lib::clplayer::ClPlayer;
use winplayer_lib::clplayermanager::ClPlayerManager;
use winplayer_lib::playermanager::PlayerManager;

use crate::LastMetadata;
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct EvArtData {
    pub data: Vec<u8>,
    pub mimetype: String,
}
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct EvMetadata {
    pub album: Option<String>,
    pub album_artist: Option<String>,
    pub album_artists: Option<Vec<String>>,
    pub artist: String,
    pub artists: Vec<String>,
    pub art_data: Option<EvArtData>, // Ensure ClArtData also derives Serialize
    pub id: Option<String>,
    pub player_aumid: String,
    pub length: f64,
    pub title: String,
    pub playing: bool,
    aumid: String,
}
struct Throttle {
    last_call: Instant,
    interval: Duration,
}

impl Throttle {
    fn new(interval: Duration) -> Self {
        Throttle {
            last_call: Instant::now() - interval, // Initialize to allow immediate first call
            interval,
        }
    }

    // Check if the function can be called
    async fn allow_call(&mut self) -> bool {
        let now = Instant::now();
        if now - self.last_call >= self.interval {
            self.last_call = now;
            true
        } else {
            false
        }
    }
}
pub async fn poll_manager_and_player_concurrently<'a>(
    mut manager: ClPlayerManager,
    app_handle: &AppHandle,
    state: Arc<tauri::async_runtime::Mutex<std::option::Option<EvMetadata>>>,
) {
    // get the state of LastMetadata
    let state = state.lock().await;
    let mut last_metadata = state.clone();
    manager.update_sessions(None).await;
    let throttle = Arc::new(Mutex::new(Throttle::new(tokio::time::Duration::from_secs(
        1,
    ))));
    //let mut last_metadata: Option<EvMetadata> = None;
    loop {
        println!("loop start");
        // Step 1: Determine the active session
        let should_call = {
            let mut throttle = throttle.lock().await;
            throttle.allow_call().await
        };
        if !should_call {
            tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
            continue;
        }
        manager.update_sessions(None).await;
        let aumid = manager
            .figure_out_active_session()
            .await
            .unwrap_or_else(|| "None".to_string());
        // Step 2: Retrieve the ClPlayer instance
        println!("Active session set to: {:?}", aumid);
        let aumid_clone = aumid.clone();
        println!("getting player for aumid: {:?}", aumid_clone);
        let mut player = match manager.get_session(aumid).await {
            Some(player) => player,
            None => {
                eprintln!("Failed to find player for aumid: {}", aumid_clone);
                continue; // Skip to the next iteration on error
            }
        };
        println!("got player for aumid: {:?}", aumid_clone);
        // Step 3: Concurrently poll the ClPlayerManager and ClPlayer
        let manager_poll = manager.poll_next_event(); // Assuming this is an async method
        let player_poll = player.poll_next_event(); // Assuming this is also an async method
                                                    // Use tokio::join! for concurrent execution
        tokio::select! {
            manager_result = manager_poll => {
                match manager_result.as_str() {
                    "ActiveSessionChanged" => {
                        println!("Active session changed");
                        update_metadata(&player, app_handle, aumid_clone,  &mut last_metadata).await;
                    }
                    "SystemSessionChanged" => {
                        println!("System session changed");
                        update_metadata(&player, app_handle, aumid_clone,  &mut last_metadata).await;
                    }
                    "SessionsChanged" => {
                        println!("Sessions changed");
                        update_metadata(&player, app_handle, aumid_clone,  &mut last_metadata).await;
                    }
                    "Timeout" => {
                        println!("Timeout");
                       // update_metadata(&player, app_handle, aumid_clone).await;
                    }
                    _ => {
                        println!("Unhandled event: {}", manager_result);
                        //update_metadata(&player, app_handle, aumid_clone).await;
                    }
                }
            },
            player_result = player_poll => {

            match player_result.as_str() {
                "PlaybackInfoChanged" => {
                    println!("Playback info changed");
                    update_metadata(&player, app_handle, aumid_clone,  &mut last_metadata).await;
                }
                "MediaPropertiesChanged" => {
                    println!("Media properties changed");
                    update_metadata(&player, app_handle, aumid_clone,  &mut last_metadata).await;
                }
                "TimelinePropertiesChanged" => {
                    println!("Timeline properties changed");


                    update_metadata(&player, app_handle, aumid_clone,  &mut last_metadata).await;
                }
                "Timeout" => {
                    println!("Timeout");
                    update_metadata(&player, app_handle, aumid_clone,  &mut last_metadata).await;
                }
                _ => {
                    println!("Unhandled event: {}", player_result);
                   // update_metadata(&player, app_handle, aumid_clone).await;
                }
            }

            // Optional: Add a delay to prevent the loop from running too frequently
            tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;
            println!("loop end");
            },
        }
        println!(
            "last_metadata: {:?}",
            last_metadata
                .as_ref()
                .map(|metadata| metadata.title.clone())
                .unwrap_or_else(|| "None".to_string())
        );
    }
}
fn metadata_to_json(metadata: EvMetadata) -> Value {
    let payload = metadata;
    let json = match serde_json::to_value(&payload) {
        Ok(value) => value,
        Err(e) => {
            // Log the error or handle it as needed
            eprintln!("Error serializing payload: {}", e); // Example of logging the error to stderr
            serde_json::Value::Null
        }
    };
    return json;
}

async fn update_metadata(
    player: &ClPlayer,
    app_handle: &AppHandle,
    aumid: String,
    last_metadata: &mut Option<EvMetadata>,
) {
    println!("Updating metadata");
    tokio::time::sleep(tokio::time::Duration::from_millis(400)).await;
    let status = player.get_status().await;
    let metadata = match status.metadata {
        Some(metadata) => metadata,
        None => {
            eprintln!("Error: Metadata is None");
            return;
        }
    };
    let mut ev_metadata = EvMetadata {
        album: metadata.album.clone(),
        album_artist: metadata.album_artist.clone(),
        album_artists: metadata.album_artists.clone(),
        artist: metadata.artist.clone(),
        artists: metadata.artists.clone(),
        art_data: metadata.art_data.map(|art_data| EvArtData {
            data: art_data.data,
            mimetype: art_data.mimetype,
        }),
        playing: false,
        player_aumid: aumid.clone(),
        id: metadata.id.clone(),
        aumid,
        length: metadata.length,
        title: metadata.title.clone(),
    };
    if status.status == GlobalSystemMediaTransportControlsSessionPlaybackStatus(4) {
        ev_metadata.playing = true;
    } else {
        ev_metadata.playing = false;
    }
    if let Some(last_metadata) = last_metadata {
        if *last_metadata != ev_metadata {
            println!("Metadata has changed");
            *last_metadata = ev_metadata.clone();
            let payload = metadata_to_json(ev_metadata);
            let title = payload
                .get("title")
                .cloned()
                .unwrap_or_else(|| Value::from("None"));
            println!("sending song as {:?}", title);
            let _ = app_handle.emit("song_change", true);
            let _ = app_handle.emit("player_status", payload);
        } else {
            println!("Metadata has not changed");
            println!("No last_metadata to compare, setting new value");
            *last_metadata = ev_metadata.clone();
            let payload = metadata_to_json(ev_metadata);
            let cur_postion = player.get_position(true).await;
            if let Some(position) = cur_postion {
                let seek_percentage = position.how_much / last_metadata.length;
                println!("Seek percentage: {}", seek_percentage);
                if seek_percentage < 0.01 {
                    println!("Song has started");
                    tokio::time::sleep(tokio::time::Duration::from_millis(400)).await;
                    let _ = app_handle.emit("song_change", true);
                }
            }

            let _ = app_handle.emit("player_status", payload);
        }
    } else {
        println!("No last_metadata to compare, setting new value");
        *last_metadata = Some(ev_metadata.clone());
        let payload = metadata_to_json(ev_metadata);
        let cur_postion = player.get_position(true).await;
        if let Some(position) = cur_postion {
            if let Some(last_metadata) = last_metadata.as_ref() {
                let seek_percentage = position.how_much / last_metadata.length;
                println!("Seek percentage: {}", seek_percentage);
                if seek_percentage < 0.01 {
                    println!("Song has started");
                    tokio::time::sleep(tokio::time::Duration::from_millis(400)).await;
                    let _ = app_handle.emit("song_change", true);
                }
            } else {
                eprintln!("Error: last_metadata is None");
            }
        }

        let _ = app_handle.emit("player_status", payload);
    }

    println!("Metadata: {:?}", metadata.title);
}

async fn get_player_and_manager(aumid: String) -> Result<(ClPlayer, ClPlayerManager), ()> {
    let player_manager: Arc<Mutex<PlayerManager>> = match PlayerManager::new().await {
        Some(manager) => Arc::new(Mutex::new(manager)),
        None => {
            eprintln!("Failed to create PlayerManager");
            return Err(());
        }
    };
    let mut cl_player_manager: ClPlayerManager = ClPlayerManager::new(player_manager);
    cl_player_manager.update_sessions(None).await;
    println!("aumid: {:?}", aumid);
    let player = match cl_player_manager.get_session(aumid).await {
        Some(player) => player,
        None => {
            eprintln!("Failed to find player for aumid");
            return Err(());
        }
    };
    Ok((player, cl_player_manager))
}

async fn player_command<'a>(
    app_handle: AppHandle,
    aumid: String,
    command: impl FnOnce(Arc<Mutex<ClPlayer>>) -> Pin<Box<dyn Future<Output = bool> + Send + 'a>>,
    state: State<'a, LastMetadata>,
) -> bool {
    let (player, mut manager) = match get_player_and_manager(aumid.to_string()).await {
        Ok((player, manager)) => (Arc::new(Mutex::new(player)), manager),
        Err(_) => return false,
    };
    println!("player_command");

    let result = command(Arc::clone(&player)).await;
    println!("player_command result: {:?}", result);
    let state = state.0.lock().await;
    let last_metadata: &mut Option<EvMetadata> = &mut state.clone();
    if result {
        println!("Command executed successfully");
        let player_lock = player.lock().await;
        manager.update_sessions(None).await;
        println!("updating metadata because of a command");
        update_metadata(&*player_lock, &app_handle, aumid, last_metadata).await;
        true
    } else {
        eprintln!("Failed to execute command");
        false
    }
    // The function body was missing, adding a closing brace to complete the function
}

#[tauri::command]
pub async fn next(
    app_handle: AppHandle,
    aumid: String,
    state: State<'_, LastMetadata>,
) -> Result<bool, ()> {
    Ok(player_command(
        app_handle,
        aumid,
        |player_arc_mutex| {
            Box::pin(async move {
                let player = player_arc_mutex.lock().await;
                player.next().await
            })
        },
        state,
    )
    .await)
}
#[tauri::command]
pub async fn play_pause(
    app_handle: AppHandle,
    aumid: String,
    state: State<'_, LastMetadata>,
) -> Result<bool, ()> {
    Ok(player_command(
        app_handle,
        aumid,
        |player_arc_mutex| {
            Box::pin(async move {
                let player = player_arc_mutex.lock().await;
                player.play_pause().await
            })
        },
        state,
    )
    .await)
}
#[tauri::command]
pub async fn previous(
    app_handle: AppHandle,
    aumid: String,
    state: State<'_, LastMetadata>,
) -> Result<bool, ()> {
    Ok(player_command(
        app_handle,
        aumid,
        |player_arc_mutex| {
            Box::pin(async move {
                let player = player_arc_mutex.lock().await;
                player.previous().await
            })
        },
        state,
    )
    .await)
}
