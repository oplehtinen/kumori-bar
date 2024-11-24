use log::{error, info};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
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
    pub art_data: Option<EvArtData>,
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
    let throttle = Arc::new(Mutex::new(Throttle::new(tokio::time::Duration::from_secs(
        1,
    ))));
    let mut last_metadata = {
        let state_guard = state.lock().await;
        state_guard.clone()
    };
    manager.update_sessions(None).await;

    'poll: loop {
        info!("loop start");
        // measure time
        let start = Instant::now();
        let should_call = {
            let mut throttle = throttle.lock().await;
            throttle.allow_call().await
        };
        if !should_call {
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            let elapsed = Instant::now() - start;
            info!("loop end !should_call, elapsed: {:?}", elapsed);
            continue;
        }
        manager.update_sessions(None).await;
        let aumid = manager
            .figure_out_active_session()
            .await
            .unwrap_or_else(|| "None".to_string());
        info!("Active session set to: {:?}", aumid);
        let aumid_clone = aumid.clone();
        info!("getting player for aumid: {:?}", aumid_clone);
        let mut player = match manager.get_session(aumid).await {
            Some(player) => player,
            None => {
                error!("Failed to find player for aumid: {}", aumid_clone);
                let elapsed = Instant::now() - start;
                info!("loop end no player, elapsed: {:?}", elapsed);
                continue;
            }
        };
        info!("got player for aumid: {:?}", aumid_clone);
        let manager_poll = manager.poll_next_event();
        let player_poll = player.poll_next_event();

        tokio::select! {
            manager_result = manager_poll => {
                if handle_manager_event(manager_result.as_str(), &mut player, app_handle, aumid_clone.clone(), &mut last_metadata).await {
                    let elapsed = Instant::now() - start;
                    info!("loop end manager_event, elapsed: {:?}", elapsed);
                    continue;
                }
            },
            player_result = player_poll => {
                if handle_player_event(player_result.as_str(), &mut player, app_handle, aumid_clone.clone(), &mut last_metadata).await {
                    let elapsed = Instant::now() - start;
                    info!("loop end player_event, elapsed: {:?}", elapsed);
                    continue;
                }
            },
        }
        info!(
            "last_metadata: {:?}",
            last_metadata
                .as_ref()
                .map(|metadata| metadata.title.clone())
                .unwrap_or_else(|| "None".to_string())
        );
        let elapsed = Instant::now() - start;
        info!("loop end, reached end, elapsed: {:?}", elapsed);
    }
}

async fn handle_manager_event(
    event: &str,
    player: &mut ClPlayer,
    app_handle: &AppHandle,
    aumid: String,
    last_metadata: &mut Option<EvMetadata>,
) -> bool {
    match event {
        "ActiveSessionChanged" | "SystemSessionChanged" | "SessionsChanged" => {
            info!("Manager event: {}", event);
            update_metadata(player, app_handle, aumid, last_metadata, None).await;
            true
        }
        "Timeout" => {
            info!("Manager event: Timeout");
            false
        }
        _ => {
            info!("Unhandled manager event: {}", event);
            false
        }
    }
}

async fn handle_player_event(
    event: &str,
    player: &mut ClPlayer,
    app_handle: &AppHandle,
    aumid: String,
    last_metadata: &mut Option<EvMetadata>,
) -> bool {
    match event {
        "PlaybackInfoChanged" | "MediaPropertiesChanged" | "TimelinePropertiesChanged" => {
            info!("Player event: {}", event);
            update_metadata(player, app_handle, aumid, last_metadata, None).await;
            true
        }
        "Timeout" => {
            info!("Player event: Timeout");
            false
        }
        _ => {
            info!("Unhandled player event: {}", event);
            false
        }
    }
}
fn metadata_to_json(metadata: EvMetadata) -> Value {
    let payload = metadata;
    let json = match serde_json::to_value(&payload) {
        Ok(value) => value,
        Err(e) => {
            error!("Error serializing payload: {}", e);
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
    toggle_play_state: Option<bool>,
) {
    info!("Updating metadata");
    let status = player.get_status().await;
    let metadata = match status.metadata {
        Some(metadata) => metadata,
        None => {
            error!("Error: Metadata is None");
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

    if let Some(toggle_play_state) = toggle_play_state {
        ev_metadata.playing = toggle_play_state
    } else {
        if status.status == GlobalSystemMediaTransportControlsSessionPlaybackStatus(4) {
            ev_metadata.playing = true;
        } else {
            ev_metadata.playing = false;
        }
    }

    if let Some(last_metadata) = last_metadata {
        if *last_metadata != ev_metadata {
            info!("Last metadata found, metadata HAS changed");
            let new_metadata = ev_metadata;
            *last_metadata = new_metadata.clone();
            let payload = metadata_to_json(new_metadata);
            let title = payload
                .get("title")
                .cloned()
                .unwrap_or_else(|| Value::from("None"));
            info!("sending song as {:?}", title);
            let _ = app_handle.emit("player_status", payload);
        } else {
            info!("Metadata has NOT changed");
            let new_metadata = ev_metadata;
            *last_metadata = new_metadata.clone();
            let payload = metadata_to_json(new_metadata);
            let cur_postion = player.get_position(true).await;
            if let Some(position) = cur_postion {
                let seek_percentage = position.how_much / last_metadata.length;
                info!("Seek percentage: {}", seek_percentage);
                if seek_percentage < 0.01 {
                    info!("Song has started");
                    let _ = app_handle.emit("player_status", payload);
                }
            }
        }
    } else {
        info!("No last_metadata to compare, setting new value");
        let new_metadata = ev_metadata;
        *last_metadata = Some(new_metadata.clone());
        let payload = metadata_to_json(new_metadata);
        let cur_postion = player.get_position(true).await;
        if let Some(position) = cur_postion {
            if let Some(last_metadata) = last_metadata.as_ref() {
                let seek_percentage = position.how_much / last_metadata.length;
                info!("Seek percentage: {}", seek_percentage);
                if seek_percentage < 0.01 {
                    info!("Song has started");
                    tokio::time::sleep(tokio::time::Duration::from_millis(400)).await;
                    let _ = app_handle.emit("song_change", true);
                }
            } else {
                error!("Error: last_metadata is None");
            }
        }

        let _ = app_handle.emit("player_status", payload);
    }

    info!("Metadata: {:?}", metadata.title);
}

async fn get_player_and_manager(aumid: String) -> Result<(ClPlayer, ClPlayerManager), ()> {
    let player_manager: Arc<Mutex<PlayerManager>> = match PlayerManager::new().await {
        Some(manager) => Arc::new(Mutex::new(manager)),
        None => {
            error!("Failed to create PlayerManager");
            return Err(());
        }
    };
    let mut cl_player_manager: ClPlayerManager = ClPlayerManager::new(player_manager);
    cl_player_manager.update_sessions(None).await;
    info!("aumid: {:?}", aumid);
    let player = match cl_player_manager.get_session(aumid).await {
        Some(player) => player,
        None => {
            error!("Failed to find player for aumid");
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
    toggle_play_state: Option<bool>,
) -> bool {
    let (player, mut manager) = match get_player_and_manager(aumid.to_string()).await {
        Ok((player, manager)) => (Arc::new(Mutex::new(player)), manager),
        Err(_) => return false,
    };

    info!("player_command");

    let command_result = {
        let player_clone = Arc::clone(&player);
        command(player_clone).await
    };

    info!("player_command result: {:?}", command_result);

    if !command_result {
        error!("Failed to execute command");
        return false;
    }

    {
        let _update_result = {
            info!("Attempting to lock state");
            let mut state_guard = state.0.lock().await;
            let last_metadata: &mut Option<EvMetadata> = &mut *state_guard;
            let should_toggle = toggle_play_state.unwrap_or(false);
            let player_lock = player.lock().await;
            if should_toggle {
                let metadata_check = match last_metadata {
                    Some(metadata_check) => metadata_check,
                    None => {
                        error!("Error: Metadata is None");
                        return false;
                    }
                };
                let toggle_play_state = metadata_check.playing;
                info!("Updating metadata because of a command");
                info!("Playing status is {:?}", toggle_play_state);
                update_metadata(
                    &*player_lock,
                    &app_handle,
                    aumid,
                    last_metadata,
                    Some(!toggle_play_state),
                )
                .await
            } else {
                info!("Updating metadata because of a command");
                update_metadata(&*player_lock, &app_handle, aumid, last_metadata, None).await
            }
            manager.update_sessions(None).await;
        };
    }

    info!("Command executed successfully");
    true
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
        Some(false),
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
        Some(true),
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
        Some(false),
    )
    .await)
}
