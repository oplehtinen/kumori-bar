use crate::winplayer::clplayer;
use crate::winplayer::playermanager::{ManagerEvent, PlayerManager};
use std::sync::Arc;
use tokio::sync::Mutex;

use super::clplayer::ClPlayer;

pub struct ClPlayerManager {
    player_manager: Arc<Mutex<PlayerManager>>,
}

impl ClPlayerManager {
    pub fn new(player_manager: Arc<Mutex<PlayerManager>>) -> Self {
        ClPlayerManager { player_manager }
    }

    pub async fn poll_next_event(&mut self) -> String {
        match self.player_manager.lock().await.poll_next_event().await {
            Some(ManagerEvent::ActiveSessionChanged) => String::from("ActiveSessionChanged"),
            Some(ManagerEvent::SystemSessionChanged) => String::from("SystemSessionChanged"),
            Some(ManagerEvent::SessionsChanged) => String::from("SessionsChanged"),
            None => String::from("None"),
        }
    }
    pub async fn get_active_session(&self) -> Option<ClPlayer> {
        if let Some(player) = self.player_manager.lock().await.get_active_session() {
            return Some(ClPlayer::new(player));
        }
        None
    }

    pub async fn get_session(&self, aumid: String) -> Option<ClPlayer> {
        if let Some(player) = self.player_manager.lock().await.get_session(&aumid) {
            return Some(ClPlayer::new(player));
        }
        None
    }

    pub async fn get_sessions_keys(&self) -> Vec<String> {
        self.player_manager.lock().await.get_sessions_keys()
    }

    pub async fn get_system_session(&self) -> Option<ClPlayer> {
        if let Some(player) = self.player_manager.lock().await.get_system_session() {
            return Some(ClPlayer::new(player));
        }
        None
    }

    pub async fn update_system_session(&mut self) {
        self.player_manager.lock().await.update_system_session()
    }

    pub async fn update_sessions(&mut self, denylist: Option<Vec<String>>) {
        self.player_manager
            .lock()
            .await
            .update_sessions(denylist.as_ref())
    }
}
