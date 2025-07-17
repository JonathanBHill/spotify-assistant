use std::collections::HashSet;

use crate::actions::exploration::playlist::PlaylistXplr;
use crate::enums::pl::PlaylistType;
use crate::models::blacklist::{Blacklist, BlacklistArtist};
use crate::traits::apis::Api;
use rspotify::model::{AlbumId, FullPlaylist, FullTrack, PlayableItem, PlaylistId, TrackId};
use rspotify::prelude::*;
use rspotify::{scopes, AuthCodeSpotify};
use tracing::{error, event, info, Level};

#[derive(Debug)]
pub struct Editor {
    client: AuthCodeSpotify,
    ref_id: PlaylistId<'static>,
    target_id: PlaylistId<'static>,
    ref_pl: FullPlaylist,
    target_pl: FullPlaylist,
}

impl Api for Editor {
    fn select_scopes() -> HashSet<String> {
        scopes!(
            "playlist-read-private",
            "playlist-read-collaborative",
            "playlist-modify-public",
            "playlist-modify-private"
        )
    }
}
impl Editor {
    pub async fn release_radar() -> Self {
        let client = Self::set_up_client(false, Some(Self::select_scopes())).await;
        let ref_id = PlaylistType::StockRR.get_id();
        let target_id = PlaylistType::MyRR.get_id();
        let target_pl = client
            .playlist(target_id.clone(), None, Some(Editor::market()))
            .await.expect("Could not retrieve target playlist");
        let ref_pl = match client.playlist(ref_id.clone(), None, Some(Editor::market())).await {
            Ok(pl) => { pl }
            Err(err) => {
                error!("Error: {:?}", err);
                panic!("Could not retrieve reference playlist");
            }
        };
        Editor {
            client,
            ref_id,
            target_id,
            ref_pl,
            target_pl,
        }
    }
    pub async fn new(ref_id: PlaylistId<'static>, target_id: PlaylistId<'static>) -> Self {
        let client = Self::set_up_client(false, Some(Self::select_scopes())).await;
        let target_pl = client
            .playlist(target_id.clone(), None, Some(Editor::market()))
            .await.expect("Could not retrieve target playlist");
        let ref_pl = client
            .playlist(ref_id.clone(), None, Some(Editor::market()))
            .await.expect("Could not retrieve reference playlist");
        Editor {
            client,
            ref_id,
            target_id,
            ref_pl,
            target_pl,
        }
    }
    pub async fn remove_liked_songs(&mut self) {
        let span = tracing::span!(Level::DEBUG, "remove_liked_songs");
        let _enter = span.enter();

        let xplr = PlaylistXplr::new(self.target_id.clone(), false).await;
        let is_liked_hashmap = xplr.find_liked_songs().await;
        let liked = is_liked_hashmap.get("liked").unwrap();
        let liked_song_ids = liked.iter().map(|track| {
            match PlayableItem::Track(track.clone()).id() {
                None => { panic!("Track does not have an ID.") }
                Some(id) => { id.into_static() }
            }
        }).collect::<Vec<PlayableId>>();
        event!(
            Level::INFO, "Removing liked songs from {:?}. Current track number: {:?} | Snapshot ID: {:?}",
            self.target_pl.name, self.target_pl.tracks.total, self.get_target_snapshot()
        );
        // println!("Target ID: {:?} | Snapshot ID: {:?}", self.target_id, self.get_target_snapshot());
        event!(Level::DEBUG, "Liked songs count: {:?}", liked_song_ids.len());
        for batch in liked_song_ids.chunks(100) {
            match self.client.playlist_remove_all_occurrences_of_items(
                self.target_id.clone(),
                batch.to_vec(),
                Some(self.get_target_snapshot().as_str())
            ).await {
                Ok(snapshot_id) => {
                    self.target_pl = match self.client.playlist(self.target_id.clone(), None, Some(Self::market()))
                                               .await {
                        Ok(pl) => { pl }
                        Err(err) => {
                            error!("Error: {:?}", err);
                            panic!("Could not retrieve target playlist");
                        }
                    };
                    event!(
                        Level::INFO, "Removed liked songs from {:?}. Updated track number: {:?} | Snapshot ID: {:?}",
                        self.target_pl.name, self.target_pl.tracks.total, snapshot_id
                    );
                }
                Err(err) => {
                    error!("Error: {:?}", err);
                    panic!("Could not remove liked songs");
                }
            };
        }
    }
    pub fn reference_id(&self) -> PlaylistId {
        self.ref_id.clone()
    }
    pub fn target_id(&self) -> PlaylistId {
        self.target_id.clone()
    }
    pub fn reference_playlist(&self) -> FullPlaylist {
        self.ref_pl.clone()
    }
    pub fn target_playlist(&self) -> FullPlaylist {
        self.target_pl.clone()
    }
    pub fn get_target_snapshot(&self) -> String {
        let snapshot = self.target_pl.snapshot_id.clone();
        println!("Snapshot ID: {:?}", snapshot);
        snapshot
    }
    fn get_track_album_id(&self, full_track: &FullTrack) -> AlbumId {
        match full_track.album.id.clone() {
            None => { panic!("Track does not have an album ID.") }
            Some(album_id) => { album_id }
        }
    }
    pub async fn get_reference_track_album_ids_filtered(&self) -> Vec<AlbumId> {
        let span = tracing::span!(Level::DEBUG, "Editor.get_reference_track_album_ids_filtered");
        let _enter = span.enter();

        let blacklist = Blacklist::new().artists();
        event!(Level::DEBUG, "Current blacklist: {:?}", blacklist);
        self.ref_pl
            .tracks
            .items
            .iter()
            .filter_map(|track| match track.track {
                Some(PlayableItem::Track(ref track)) => {
                    let lead_artist_id = track
                        .artists
                        .first().unwrap().id.clone().expect("Could not clone artist ID").to_string();
                    let lead_artist_name = track
                        .artists
                        .first().unwrap().name.clone();
                    let hypothetical_blacklist_artist = BlacklistArtist::new(lead_artist_name.clone(), lead_artist_id.clone());
                    if blacklist.contains(&hypothetical_blacklist_artist) {
                        event!(
                            Level::INFO, "Artist {:?} is blacklisted. Skipping album ID retrieval.", lead_artist_name
                        );
                        None
                    } else {
                        let album_id = self.get_track_album_id(track);
                        Some(album_id)
                    }
                }
                _ => None,
            })
            .collect()
    }
    pub async fn get_album_tracks_from_reference(&self) -> Vec<TrackId> {
        let album_ids = self.get_reference_track_album_ids_filtered().await;
        let mut return_vector = Vec::new();
        let mut album_track_ids = Vec::new();
        for chunk in album_ids.chunks(20) {
            let albums = self
                .client
                .albums(chunk.to_vec(), Some(Self::market()))
                .await
                .expect("Could not retrieve albums from album IDs");

            albums.iter().for_each(|album| {
                let album_track_ids_vec = album
                    .tracks
                    .items
                    .iter()
                    .map(|track| track.id.clone().expect("Could not clone track ID"))
                    .collect::<Vec<TrackId>>();

                return_vector = Self::append_uniques(&return_vector, &album_track_ids_vec);
                album_track_ids.extend(album_track_ids_vec);
            });
        }
        album_track_ids = Self::clean_duplicate_id_vector(album_track_ids);
        println!("Return length: {:?} | ID length {:?}", return_vector.len(), album_track_ids.len());
        return_vector
    }
    pub async fn wipe_reference_playlist(&self) {
        let span = tracing::span!(Level::DEBUG, "Editor.wipe_reference_playlist");
        let _enter = span.enter();
        let ref_id = self.ref_id.clone();
        let xplorer = PlaylistXplr::new(ref_id.clone(), false).await;
        let track_ids = xplorer.tracks
                               .iter().map(|track| {
            match PlayableItem::Track(track.clone()).id() {
                None => { panic!("Track does not have an ID.") }
                Some(id) => { id.into_static() }
            }
        }).collect::<Vec<PlayableId>>();
        for batch in track_ids.chunks(100) {
            match self.client.playlist_remove_all_occurrences_of_items(ref_id.clone(), batch.to_vec(), None).await {
                Ok(_) => {
                    event!(Level::INFO, "Removed tracks from reference playlist.");
                }
                Err(err) => {
                    error!("Error: {:?}", err);
                    panic!("Could not remove tracks from reference playlist");
                }
            }
        }
    }
    pub async fn update_playlist(&self) {
        let span = tracing::span!(Level::DEBUG, "Editor.update_playlist");
        let _enter = span.enter();
        let ids = self.get_album_tracks_from_reference().await;
        if self.target_id.clone() == PlaylistType::StockRR.get_id() {
            error!(
                "Your Stock Release Radar ID was used: {playlist_id}",
                playlist_id = self.target_id.id()
            );
            panic!("You must ensure that you are calling the update method with your full version release radar ID instead of your stock version's.")
        } else {
            info!(
                "Your Full Release Radar playlists will be updated with {number} songs",
                number = ids.len()
            );
        }
        let mut first_chunk = true;
        for chunk in ids.chunks(20) {
            let chunk_iterated = chunk.iter().map(|track| PlayableId::Track(track.as_ref()));

            if first_chunk {
                let local_time = chrono::Local::now();
                let local_time_string = local_time.format("%m/%d/%Y").to_string();
                let description = format!(
                    "Release Radar playlists with songs from albums included. Created on 11/02/2023. Updated on {}.",
                    local_time_string
                );
                self.client
                    .playlist_change_detail(self.target_id.clone(), None, None, Some(description.as_str()), None)
                    .await.expect("Couldn't update description");
                self.client
                    .playlist_replace_items(self.target_id.clone(), chunk_iterated)
                    .await.expect("Track IDs should be assigned to chunk_iterated as type TrackID");
                first_chunk = false;
            } else {
                self.client
                    .playlist_add_items(self.target_id.clone(), chunk_iterated, None)
                    .await.expect("Track IDs should be assigned to chunk_iterated as type TrackID");
            }
        }
        self.wipe_reference_playlist().await;
    }
    fn append_uniques<'a>(existing: &Vec<TrackId<'a>>, new: &[TrackId<'a>]) -> Vec<TrackId<'a>> {
        let mut extended = existing.to_owned();
        let intersection: Vec<TrackId> = existing
            .iter()
            .filter(|x| new.contains(x))
            .cloned()
            .collect();
        extended.extend(new.iter().filter(|x| !intersection.contains(x)).cloned());
        extended
    }
}
