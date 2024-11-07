use std::collections::HashSet;

use rspotify::{AuthCodeSpotify, scopes};
use rspotify::clients::OAuthClient;
use rspotify::model::{FullPlaylist, PlaylistId, SimplifiedPlaylist};
use tracing::{info, Level};

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

pub struct UserPlaylists {
    client: AuthCodeSpotify,
}

impl Api for UserPlaylists {
    fn select_scopes() -> HashSet<String> {
        scopes!("playlist-read-private")
    }
}

impl UserPlaylists {
    pub async fn new() -> Self {
        let span = tracing::span!(Level::INFO, "UserPlaylists.new");
        let _enter = span.enter();

        let client = Self::set_up_client(false, Some(Self::select_scopes())).await;
        UserPlaylists { client }
    }
    pub async fn get_user_playlists(&self) -> Vec<SimplifiedPlaylist> {
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
                // playlist.public
                info!(
                    "{index}: Name: {:?} | Public: {:?}",
                    playlist.name, playlist.public
                );
                // pl_vec.insert(index, playlist.clone());
            });
        info!("Total playlists: {}", playlists.total);
        pl_vec
    }
}
