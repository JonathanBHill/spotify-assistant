use std::collections::HashSet;

use rspotify::model::{AlbumId, ArtistId, FullPlaylist, FullTrack, PlayableItem, PlaylistId, TrackId};
use rspotify::prelude::*;
use rspotify::{scopes, AuthCodeSpotify};
use tracing::{error, info, Level};

use crate::enums::pl::PlaylistType;
use crate::models::blacklist::{Blacklist, BlacklistArtist};
use crate::traits::apis::Api;
use crate::utilities::filesystem::files::ProjectFiles;
use crate::utilities::general::print_separator;

pub struct Updater {
    client: AuthCodeSpotify,
    ref_id: PlaylistId<'static>,
    target_id: PlaylistId<'static>,
    ref_pl: FullPlaylist,
    target_pl: FullPlaylist,
}

impl Api for Updater {
    fn select_scopes() -> HashSet<String> {
        scopes!(
            "playlist-read-private",
            "playlist-read-collaborative",
            "playlist-modify-public",
            "playlist-modify-private"
        )
    }
}
impl Updater {
    pub async fn release_radar() -> Self {
        let client = Self::set_up_client(false, Some(Self::select_scopes())).await;
        let ref_id = PlaylistType::StockRR.get_id();
        let target_id = PlaylistType::MyRR.get_id();
        let target_pl = client
            .playlist(target_id.clone(), None, Some(Updater::market()))
            .await.expect("Could not retrieve custom playlists");
        let ref_pl = client
            .playlist(ref_id.clone(), None, Some(Updater::market()))
            .await.expect("Could not retrieve stock playlists");
        Updater {
            client,
            ref_id,
            target_id,
            ref_pl,
            target_pl,
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
    pub async fn get_snapshot(&self) -> String {
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
        let blacklist = Blacklist::new().artists();
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
    pub async fn update_playlist(&self) {
        let span = tracing::span!(Level::DEBUG, "rr_update");
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
