use ::std::sync::Arc;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tauri::{AppHandle, Manager};
use tokio::sync::{Mutex, Notify};
use tokio::time::timeout;
use WinPlayer_Rust::clplayer::ClPlayer;
use WinPlayer_Rust::clplayermanager::ClPlayerManager;
use WinPlayer_Rust::player;
use WinPlayer_Rust::util::get_session_player_name;
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
pub async fn poll_manager_and_player_concurrently(
    mut manager: ClPlayerManager,
    app_handle: &AppHandle,
) {
    let stop_flag = Arc::new(AtomicBool::new(false));
    let notify = Arc::new(Notify::new());
    let stopped = Arc::new(AtomicBool::new(true));
    loop {
        println!("loop start");
        // Step 1: Determine the active session
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
                        update_metadata(&player, app_handle).await;
                    }
                    "SystemSessionChanged" => {
                        println!("System session changed");
                        update_metadata(&player, app_handle).await;
                    }
                    "SessionsChanged" => {
                        println!("Sessions changed");
                        update_metadata(&player, app_handle).await;
                    }
                    "Timeout" => {
                        println!("Timeout");
                        update_metadata(&player, app_handle).await;
                    }
                    _ => {
                        println!("Unhandled event: {}", manager_result);
                        update_metadata(&player, app_handle).await;
                    }
                }
            },
            player_result = player_poll => {

            match player_result.as_str() {
                "PlaybackInfoChanged" => {
                    println!("Playback info changed");
                    update_metadata(&player, app_handle).await;
                }
                "MediaPropertiesChanged" => {
                    println!("Media properties changed");
                    update_metadata(&player, app_handle).await;
                }
                "TimelinePropertiesChanged" => {
                    println!("Timeline properties changed");
                    update_metadata(&player, app_handle).await;
                }
                "Timeout" => {
                    println!("Timeout");
                    update_metadata(&player, app_handle).await;
                }
                _ => {
                    println!("Unhandled event: {}", player_result);
                    update_metadata(&player, app_handle).await;
                }
            }

            // Optional: Add a delay to prevent the loop from running too frequently
            //tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            println!("loop end");
            },
        }
    }
}

pub async fn start_player_loop(
    mut player: ClPlayer,
    stop_flag: Arc<AtomicBool>,
    notify: Arc<Notify>,
    stopped: &Arc<AtomicBool>,
    app_handle: &AppHandle,
) {
    loop {
        println!("looping player");
        if stop_flag.load(Ordering::SeqCst) {
            println!("Stopping player loop");
            stopped.store(true, Ordering::SeqCst); // Signal that the loop has stopped
            notify.notify_one(); // Notify waiting tasks
            break;
        }
        notify.notify_one();
        notify.notified().await;

        let player_evt: String = timeout(Duration::from_secs(2), player.poll_next_event())
            .await
            .unwrap_or_else(|_| String::new());
        match player_evt.as_str() {
            "PlaybackInfoChanged" => {
                println!("Playback info changed");
                let _ = update_metadata(&player, app_handle).await;
                //println!("{:?}", status);
            }
            "MediaPropertiesChanged" => {
                println!("Media properties changed");
                let _ = update_metadata(&player, app_handle).await;
                //println!("{:?}", status);
            }
            "TimelinePropertiesChanged" => {
                println!("Timeline properties changed");
                let _ = update_metadata(&player, app_handle).await;
                /* let pos = player.get_position(false).await;
                println!("{:?}", pos); */
            }
            _ => {}
        }
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
async fn update_metadata(player: &ClPlayer, app_handle: &AppHandle) {
    println!("Updating metadata");
    let status = player.get_status().await;
    let metadata = status.metadata.unwrap();
    let payload = metadata_to_json(metadata);
    let _ = app_handle.emit_all("player_status", payload);
}
