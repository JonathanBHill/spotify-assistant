use std::collections::HashSet;

use dotenv::dotenv;
use rspotify::model::{FullPlaylist, FullTrack, PlayableItem};
use rspotify::{scopes, AuthCodeSpotify};

use crate::traits::apis::Api;

#[allow(dead_code)]
pub struct ComparePlaylists {
    client: AuthCodeSpotify,
    playlist: FullPlaylist,
    pub stored_tracks: Vec<FullTrack>,
}

impl PartialEq for ComparePlaylists {
    fn eq(&self, other: &Self) -> bool {
        let pl_ids = self.playlist.id == other.playlist.id;
        pl_ids
    }
}
impl Api for ComparePlaylists {
    fn select_scopes() -> HashSet<String> {
        scopes!(
            "playlists-read-private",
            "playlists-read-collaborative",
            "playlists-modify-public",
            "playlists-modify-private"
        )
    }
}
#[allow(dead_code)]
impl ComparePlaylists {
    pub async fn new(playlist: FullPlaylist) -> Self {
        dotenv().ok();
        let client = Self::set_up_client(false, Some(Self::select_scopes())).await;
        let tracks = playlist.tracks.items.iter().map(|track| {
            match track.track.clone() {
                Some(track) => match track {
                    PlayableItem::Track(track) => track,
                    PlayableItem::Episode(episode) => {
                        eprintln!("Error: Incorrect item returned. An episode was provided: {:?}", episode.name);
                        panic!("Could not get full track")
                    },
                },
                None => panic!("Could not get track"),
            }
        }).collect::<Vec<FullTrack>>();
        ComparePlaylists {
            client,
            playlist: playlist.clone(),
            stored_tracks: tracks,
        }
    }
    pub fn eq_len(&self, other: &Self) -> bool {
        println!("Playlist lengths are equal: {:?}", self.playlist.tracks.total == other.playlist.tracks.total);
        println!("Playlist 1 length: {:?}", self.playlist.tracks.total);
        println!("Playlist 2 length: {:?}", other.playlist.tracks.total);
        self.playlist.tracks.total == other.playlist.tracks.total
    }
    pub fn comp_metadata(&self, other: &Self) -> bool {
        let eq_id = self.playlist.id == other.playlist.id;
        if eq_id {
            println!("Playlists are equal.");
            println!("Playlist ID: {:?}", self.playlist.id);
            println!("Playlist name: {:?}", self.playlist.name);
            println!("Playlist owner ID: {:?}", self.playlist.owner.id);
        } else {
            println!("Playlists are not equal.");
            println!("Playlist 1 ID: {:?}", self.playlist.id);
            println!("Playlist 2 ID: {:?}", other.playlist.id);
            println!("Playlist 1 name: {:?}", self.playlist.name);
            println!("Playlist 2 name: {:?}", other.playlist.name);
            println!("Playlist 1 owner ID: {:?}", self.playlist.owner.id);
            println!("Playlist 2 owner ID: {:?}", other.playlist.owner.id);
        }
        eq_id
    }
    fn combine_vectors<T: Clone>(v1: Vec<T>, v2: Vec<T>, v3: Vec<T>, headers: (T, T, T), default: T) -> Vec<(T, T, T)> {
        let first_len = std::cmp::max(v1.len(), v2.len());
        let len = std::cmp::max(first_len, v3.len());
        let mut combined = Vec::with_capacity(len);
        combined.push(headers);

        for i in 0..len {
            let elem1 = v1.get(i).cloned().unwrap_or_else(|| default.clone());
            let elem2 = v2.get(i).cloned().unwrap_or_else(|| default.clone());
            let elem3 = v3.get(i).cloned().unwrap_or_else(|| default.clone());
            combined.push((elem1, elem2, elem3));
        }
        combined
    }
}
