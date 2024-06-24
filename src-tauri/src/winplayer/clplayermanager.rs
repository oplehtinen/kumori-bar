use crate::winplayer::clplayer;
use crate::winplayer::playermanager::{ManagerEvent, PlayerManager};

use super::clplayer::ClPlayer;

pub struct ClPlayerManager {
    player_manager: PlayerManager,
}

impl ClPlayerManager {
    pub fn new(player_manager: PlayerManager) -> Self {
        ClPlayerManager { player_manager }
    }

    pub async unsafe fn poll_next_event(&mut self) -> String {
        match self.player_manager.poll_next_event().await {
            Some(ManagerEvent::ActiveSessionChanged) => String::from("ActiveSessionChanged"),
            Some(ManagerEvent::SystemSessionChanged) => String::from("SystemSessionChanged"),
            Some(ManagerEvent::SessionsChanged) => String::from("SessionsChanged"),
            None => String::from("None"),
        }
    }
    pub fn get_active_session(&self) -> Option<ClPlayer> {
        if let Some(player) = self.player_manager.get_active_session() {
            return Some(ClPlayer::new(player));
        }
        None
    }

    pub fn get_session(&self, aumid: String) -> Option<ClPlayer> {
        if let Some(player) = self.player_manager.get_session(&aumid) {
            return Some(ClPlayer::new(player));
        }
        None
    }

    pub fn get_sessions_keys(&self) -> Vec<String> {
        self.player_manager.get_sessions_keys()
    }

    pub fn get_system_session(&self) -> Option<ClPlayer> {
        if let Some(player) = self.player_manager.get_system_session() {
            return Some(ClPlayer::new(player));
        }
        None
    }

    pub fn update_system_session(&mut self) {
        self.player_manager.update_system_session()
    }

    pub fn update_sessions(&mut self, denylist: Option<Vec<String>>) {
        self.player_manager.update_sessions(denylist.as_ref())
    }
}
