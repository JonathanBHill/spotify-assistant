use futures::StreamExt;
use rspotify::clients::{BaseClient, OAuthClient};
use rspotify::model::{FullPlaylist, FullTrack, PlaylistId, SavedTrack, SimplifiedPlaylist};
use rspotify::{scopes, AuthCodeSpotify};
use std::collections::{HashMap, HashSet};
use std::time::Duration;
use tokio::time::sleep;
use tracing::{event, info, Level};

use crate::traits::apis::Api;

pub struct LikedSongs {
    client: AuthCodeSpotify,
    total_tracks: u32,
}

impl Api for LikedSongs {
    fn select_scopes() -> HashSet<String> {
        scopes!("user-library-read", "user-library-modify")
    }
}
impl LikedSongs {
    pub async fn new() -> Self {
        let span = tracing::span!(Level::INFO, "LikedSongs.new");
        let _enter = span.enter();
        let client = Self::set_up_client(false, Some(Self::select_scopes())).await;
        let total_tracks = match client.current_user_saved_tracks_manual(
            Some(Self::market()),
            None,
            None
        ).await {
            Ok(tracks) => { tracks.total }
            Err(err) => {
                event!(Level::ERROR, "Error: {:?}", err);
                panic!("Could not retrieve saved tracks.");
            }
        };
        LikedSongs {
            client,
            total_tracks,
        }
    }
    pub fn client(&self) -> AuthCodeSpotify {
        self.client.clone()
    }
    pub async fn library(&self) -> Vec<SavedTrack> {
        let span = tracing::span!(Level::INFO, "LikedSongs.library");
        let _enter = span.enter();

        let mut liked_songs = self.client.current_user_saved_tracks(Some(Self::market()));
        let mut saved_tracks: Vec<SavedTrack> = Vec::new();
        let mut retries = 3;
        while retries > 0 {
            if let Some(page) = liked_songs.next().await {
                match page {
                    Ok(saved_track) => {
                        event!(Level::INFO, "Saved track: {:?} | New vector length: {:?}", saved_track.track.name, saved_tracks.len() + 1);
                        saved_tracks.push(saved_track);
                    },
                    Err(err) => {
                        event!(Level::ERROR, "Error: {:?}", err);
                        retries -= 1;
                    }
                }
            } else {
                break;
            }
        }
        saved_tracks
    }

}

#[derive(Clone)]
pub struct UserPlaylists {
    client: AuthCodeSpotify,
}

impl Api for UserPlaylists {
    fn select_scopes() -> HashSet<String> {
        scopes!("playlist-read-private", "playlist-read-collaborative", "user-library-read")
    }
}

impl UserPlaylists {
    pub async fn new() -> Self {
        let span = tracing::span!(Level::INFO, "UserPlaylists.new");
        let _enter = span.enter();

        let client = Self::set_up_client(false, Some(Self::select_scopes())).await;
        UserPlaylists { client }
    }
    pub async fn stockrr(&self) -> FullPlaylist {
        let span = tracing::span!(Level::INFO, "UserPlaylists.stockrr");
        let _enter = span.enter();
        let rr_id = PlaylistId::from_id("37i9dQZEVXbdINACbjb1qu").unwrap();
        let rr_pl = self
            .client
            .playlist(rr_id.clone(), None, Some(Self::market()))
            .await
            .expect("Could not retrieve playlists");
        rr_pl
    }
    pub async fn custom_release_radar(&self) -> FullPlaylist {
        let span = tracing::span!(Level::INFO, "UserPlaylists.custom_release_radar");
        let _enter = span.enter();
        let rr_id = match PlaylistId::from_id("46mIugmIiN2HYVwAwlaBAr") {
            Ok(id) => id,
            Err(err) => {
                event!(Level::ERROR, "Error: {:?}", err);
                panic!("Could not retrieve playlist");
            }
        };
        match self.client.playlist(rr_id.clone(), None, Some(Self::market())).await {
            Ok(release_radar_playlist) => release_radar_playlist,
            Err(err) => {
                event!(Level::ERROR, "Error: {:?}", err);
                panic!("Could not retrieve playlists");
            }
        }
    }
    pub async fn get_user_playlists(&self) -> HashMap<String, PlaylistId> {
        let span = tracing::span!(Level::INFO, "UserPlaylists.get_user_playlists");
        let _enter = span.enter();

        let mut user_playlists = self
            .client
            .current_user_playlists();
        let mut playlists = HashMap::new();
        let mut retries = 3;
        while retries > 0 {
            if let Some(pl) = user_playlists.next().await {
                match pl {
                    Ok(simp) => {
                        playlists.insert(simp.name, simp.id);
                    },
                    Err(err) => {
                        event!(Level::ERROR, "Error retrieving playlist: {:?}", err);
                        retries -= 1;
                    }
                }
            } else {
                break;
            }
        }
        if retries == 0 {
            event!(Level::ERROR, "Failed to retrieve playlists after multiple attempts.");
        }
        playlists.clone()
    }
    pub async fn get_user_playlists_old(&self) -> Vec<SimplifiedPlaylist> {
        let span = tracing::span!(Level::INFO, "UserData.playlists");
        let _enter = span.enter();
        let playlists = match self.client.current_user_playlists_manual(Some(50), None).await {
            Ok(playlists) => playlists,
            Err(error) => panic!("Could not get playlists: {}", error),
        };

        let page_size = 50;
        let total_pl = playlists.total;
        let mut pl_vec = Vec::with_capacity(total_pl as usize);
        let pages_no_remainder = (total_pl / page_size) as i32;
        let pages = if total_pl % page_size > 0 {
            info!("pages with remainder: {}", pages_no_remainder + 1);
            pages_no_remainder + 1
        } else {
            info!("pages w/o remainder: {pages_no_remainder}");
            pages_no_remainder
        };

        for page in 0..pages {
            let offset = page_size * page as u32;
            let multiplier = page_size as usize * page as usize;
            let offset_playlists = match self
                .client
                .current_user_playlists_manual(Some(page_size), Some(offset))
                .await
            {
                Ok(page) => page.items.into_iter(),
                Err(error) => panic!("{:?}", error),
            };
            for (index, playlist) in offset_playlists.enumerate() {
                let playlist_number = index + multiplier;
                pl_vec.insert(playlist_number, playlist);
            }
            info!("Page {}/{} appended", page + 1, pages)
        }
        pl_vec
            .clone()
            .iter()
            .enumerate()
            .for_each(|(index, playlist)| {
                info!(
                    "{index}: Name: {:?} | Public: {:?}",
                    playlist.name, playlist.public
                );
            });
        info!("Total playlists: {}", playlists.total);
        pl_vec
    }
}
