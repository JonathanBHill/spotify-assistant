use futures::{StreamExt, TryStreamExt};
use rspotify::clients::{BaseClient, OAuthClient};
use rspotify::model::{FullPlaylist, PlaylistId, SimplifiedPlaylist};
use rspotify::{scopes, AuthCodeSpotify};
use std::collections::{HashMap, HashSet};
use tracing::{event, info, Level};

use crate::traits::apis::Api;

pub struct LikedSongs {
    client: AuthCodeSpotify,
    id: PlaylistId<'static>,
    full_playlist: FullPlaylist,
}

impl Api for LikedSongs {
    fn select_scopes() -> HashSet<String> {
        scopes!("user-library-read", "user-library-modify")
    }
}
impl LikedSongs {
    pub fn full_playlist(&self) -> FullPlaylist {
        self.full_playlist.clone()
    }
    pub fn id(&self) -> PlaylistId {
        self.id.clone()
    }
    pub fn client(&self) -> AuthCodeSpotify {
        self.client.clone()
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
    pub async fn myrr(&self) -> FullPlaylist {
        let span = tracing::span!(Level::INFO, "UserPlaylists.myrr");
        let _enter = span.enter();
        let rr_id = PlaylistId::from_id("46mIugmIiN2HYVwAwlaBAr").unwrap();
        let rr_pl = self
            .client
            .playlist(rr_id.clone(), None, Some(Self::market()))
            .await
            .expect("Could not retrieve playlists");
        rr_pl
    }
    pub async fn get_user_playlists(&self) -> HashMap<String, PlaylistId> {
        let span = tracing::span!(Level::INFO, "UserPlaylists.myrr");
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
        let playlists = match self
            .client
            .current_user_playlists_manual(Some(50), None)
            .await
        {
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
