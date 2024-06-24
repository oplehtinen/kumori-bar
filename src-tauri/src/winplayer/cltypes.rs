use chrono::{DateTime, Utc};
use windows::Storage::Streams::Buffer;

use crate::winplayer::types::{ArtData, Capabilities, Metadata, Position, Status};

#[derive(Debug)]
pub struct ClArtData {
    pub data: Vec<u8>,
    pub mimetype: String,
}

impl From<ArtData> for ClArtData {
    fn from(value: ArtData) -> Self {
        ClArtData {
            data: value.data,
            mimetype: value.mimetype,
        }
    }
}

#[derive(Debug)]
pub struct ClMetadata {
    pub album: Option<String>,
    pub album_artist: Option<String>,
    pub album_artists: Option<Vec<String>>,
    pub artist: String,
    pub artists: Vec<String>,
    pub art_data: Option<ClArtData>,
    pub id: Option<String>,
    pub length: f64,
    pub title: String,
}

impl From<Metadata> for ClMetadata {
    fn from(value: Metadata) -> Self {
        ClMetadata {
            album: value.album,
            album_artist: value.album_artist,
            album_artists: value.album_artists,
            artist: value.artist,
            artists: value.artists,
            art_data: 'rt: {
                if let Some(art_data) = value.art_data {
                    break 'rt Some(ClArtData::from(art_data));
                };
                None
            },
            id: value.id,
            length: value.length,
            title: value.title,
        }
    }
}

#[derive(Debug)]
pub struct ClCapabilities {
    pub can_control: bool,
    pub can_play_pause: bool,
    pub can_go_next: bool,
    pub can_go_previous: bool,
    pub can_seek: bool,
}

impl From<Capabilities> for ClCapabilities {
    fn from(value: Capabilities) -> Self {
        ClCapabilities {
            can_control: value.can_control,
            can_play_pause: value.can_play_pause,
            can_go_next: value.can_go_next,
            can_go_previous: value.can_go_previous,
            can_seek: value.can_seek,
        }
    }
}

#[derive(Debug)]
pub struct ClPosition {
    pub how_much: f64,
    pub when: DateTime<Utc>,
}

impl From<Position> for ClPosition {
    fn from(value: Position) -> Self {
        ClPosition {
            how_much: value.how_much,
            when: value.when,
        }
    }
}

#[derive(Debug)]
pub struct ClStatus {
    pub metadata: Option<ClMetadata>,
    pub capabilities: ClCapabilities,
    pub status: String,
    pub is_loop: String,
    pub shuffle: bool,
    pub volume: f64, // tanto sta a -1 lmao
    pub elapsed: Option<ClPosition>,
    pub app: Option<String>, // App User Model ID
}

impl From<Status> for ClStatus {
    fn from(value: Status) -> Self {
        ClStatus {
            metadata: 'rt: {
                if let Some(metadata) = value.metadata {
                    break 'rt Some(ClMetadata::from(metadata));
                };
                None
            },
            capabilities: ClCapabilities::from(value.capabilities),
            status: value.status,
            is_loop: value.is_loop,
            shuffle: value.shuffle,
            volume: value.volume,
            elapsed: 'rt: {
                if let Some(elapsed) = value.elapsed {
                    break 'rt Some(ClPosition::from(elapsed));
                };
                None
            },
            app: value.app,
        }
    }
}
