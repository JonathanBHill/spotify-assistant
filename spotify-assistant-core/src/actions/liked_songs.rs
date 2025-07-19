use crate::enums::fs::ProjectDirectories;
use crate::traits::apis::Api;
use rspotify::model::SavedTrack;
use rspotify::prelude::OAuthClient;
use rspotify::AuthCodeSpotify;
use std::io::Write;
use std::path::PathBuf;
use std::{fs, io};
use tracing::event;

pub struct UserLikedSongs {
    client: AuthCodeSpotify,
    tracks: Vec<SavedTrack>,
    saved_tracks_path: PathBuf,
}
impl Api for UserLikedSongs {
    fn select_scopes() -> std::collections::HashSet<String> {
        rspotify::scopes!(
            "user-library-read"
        )
    }
}
impl UserLikedSongs {
    pub async fn new() -> Self {
        let client = Self::set_up_client(false, Some(Self::select_scopes())).await;
        let data_dir = ProjectDirectories::Data;
        let saved_tracks_path = data_dir.path().join("liked_songs.json");
        if let Ok(tracks) = Self::load_from_file() {
            println!("Loaded liked songs from cache.");
            Self { client, tracks, saved_tracks_path }
        } else {
            let tracks = match Self::fetch_all(&client).await {
                Ok(tracks) => {
                    event!(tracing::Level::INFO, "Successfully fetched liked songs.");
                    tracks
                },
                Err(err) => {
                    event!(tracing::Level::ERROR, "Failed to fetch liked songs: {:?}", err);
                    Vec::new()
                },
            };
            let self_obj = Self {
                client,
                tracks,
                saved_tracks_path,
            };
            match self_obj.save_to_file() {
                Ok(_) => event!(tracing::Level::INFO, "Liked songs saved to cache."),
                Err(err) => event!(tracing::Level::ERROR, "Failed to save liked songs to cache: {:?}", err),
            };
            // Self {client, tracks, saved_tracks_path}
            self_obj
        }
    }
    async fn fetch_all(client: &AuthCodeSpotify) -> Result<Vec<SavedTrack>, rspotify::ClientError> {
        let span = tracing::span!(tracing::Level::INFO, "UserLikedSongs.fetch_all");
        let _enter = span.enter();
        let mut tracks = Vec::new();
        let mut offset = 0;

        loop {
            let page = match client.current_user_saved_tracks_manual(
                Some(Self::market()), Some(50), Some(offset)
            ).await {
                Ok(page) => {
                    event!(tracing::Level::INFO, "Fetched page with {} items", page.items.len());
                    page
                },
                Err(err) => {
                    event!(tracing::Level::ERROR, "Failed to fetch liked songs: {:?}", err);
                    return Err(err)
                },
            };
            if page.items.is_empty() {
                event!(tracing::Level::INFO, "No more liked songs to fetch.");
                break;
            }
            offset += page.items.len() as u32;
            tracks.extend(page.items);
            if page.next.is_none() {
                event!(tracing::Level::INFO, "Reached the end of liked songs.");
                break;
            }
        }

        Ok(tracks)
    }
    pub fn tracks(&self) -> Vec<SavedTrack> {
        self.tracks.clone()
    }
    fn save_to_file(&self) -> io::Result<()> {
        let json = serde_json::to_string_pretty(&self.tracks)?;
        let mut file = fs::File::create(self.saved_tracks_path.clone())?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    fn load_from_file() -> io::Result<Vec<SavedTrack>> {
        let data_dir = ProjectDirectories::Data;
        let liked_songs_path = data_dir.path().join("liked_songs.json");
        let contents = fs::read_to_string(liked_songs_path)?;
        let tracks: Vec<SavedTrack> = serde_json::from_str(&contents)?;
        Ok(tracks)
    }
}
