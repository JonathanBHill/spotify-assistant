use std::collections::HashSet;

use rspotify::clients::{BaseClient, OAuthClient};
use rspotify::model::{AlbumId, FullPlaylist, Id, PlayableId, PlayableItem, PlaylistId, TrackId};
use rspotify::{scopes, AuthCodeSpotify};
use tracing::{error, info, Level};

use crate::core::models::enums::PlaylistType;
use crate::core::models::traits::{Api, Querying};
use crate::core::utilities::miscellaneous::print_separator;

pub struct ReleaseRadar {
    client: AuthCodeSpotify,
    id: PlaylistId<'static>,
    full_playlist: FullPlaylist,
}

impl Api for ReleaseRadar {
    fn select_scopes() -> HashSet<String> {
        scopes!(
            "playlist-read-private",
            "playlist-read-collaborative",
            "playlist-modify-public",
            "playlist-modify-private"
        )
    }
}

impl Querying for ReleaseRadar {
    async fn new() -> Self {
        let client = Self::set_up_client(false, Some(Self::select_scopes())).await;
        let rr_id = PlaylistType::StockRR.get_id();
        let rr_pl = client
            .playlist(rr_id.clone(), None, Some(ReleaseRadar::market()))
            .await
            .expect("Could not retrieve playlist");
        ReleaseRadar {
            client,
            id: rr_id,
            full_playlist: rr_pl,
        }
    }
}

impl ReleaseRadar {
    pub async fn new_personal() -> Self {
        let client = Self::set_up_client(false, Some(Self::select_scopes())).await;
        let rr_id = PlaylistType::MyRR.get_id();
        let rr_pl = client
            .playlist(rr_id.clone(), None, Some(ReleaseRadar::market()))
            .await
            .expect("Could not retrieve playlist");
        ReleaseRadar {
            client,
            id: rr_id,
            full_playlist: rr_pl,
        }
    }
    pub async fn get_rr_track_album_ids(&self) -> Vec<AlbumId> {
        let rr_track_album_ids = self
            .full_playlist
            .tracks
            .items
            .iter()
            .filter_map(|track| match track.track {
                Some(PlayableItem::Track(ref track)) => {
                    let album_id_from_track = track
                        .album
                        .id
                        .clone()
                        .expect("Track does not have an album ID.")
                        .id()
                        .to_string();
                    let album_id = AlbumId::from_id(album_id_from_track)
                        .expect("Could not convert album ID from string");
                    Some(album_id)
                }
                _ => None,
            })
            .collect();
        rr_track_album_ids
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
    pub async fn get_album_tracks_from_rr(&self, print: bool) -> Vec<TrackId> {
        let album_ids = self.get_rr_track_album_ids().await;
        let mut return_vector = Vec::new();
        let mut album_track_ids = Vec::new();
        for chunk in album_ids.chunks(20) {
            let albums = self
                .client
                .albums(chunk.to_vec(), Some(Self::market()))
                .await
                .expect("Could not retrieve albums from album IDs");

            albums.iter().for_each(|album| {
                return_vector = Self::append_uniques(
                    &return_vector,
                    &album
                        .tracks
                        .items
                        .iter()
                        .map(|track| track.id.clone().expect("Could not clone track ID"))
                        .collect::<Vec<TrackId>>(),
                );
                album_track_ids.push(
                    album
                        .tracks
                        .items
                        .iter()
                        .map(|track| track.id.clone().expect("Could not clone track ID"))
                        .collect::<Vec<TrackId>>(),
                );
            });
        }
        if print {
            Self::print_all_album_track_ids(&album_track_ids);
        };
        return_vector
    }
    pub async fn update_rr(&self, print: bool) {
        let span = tracing::span!(Level::DEBUG, "rr_update");
        let _enter = span.enter();
        let ids = self.get_album_tracks_from_rr(false).await;
        let pl_id =
            PlaylistId::from_id(self.id.id()).expect("Could not convert string to a playlist ID");
        if pl_id.clone() == PlaylistType::StockRR.get_id() {
            error!(
                "The Spotify Release Radar ID was used: {playlist_id}",
                playlist_id = pl_id.id()
            );
            panic!("You must ensure that you are calling the update method with your playlist ID instead of Spotify's.")
        } else {
            info!(
                "Your Full Release Radar playlist will be updated with {number:?} songs",
                number = ids.len()
            );
        }
        let chunks = ids.chunks(20);
        let mut first_chunk = true;
        for chunk in chunks {
            let chunk_iterated = chunk.iter().map(|track| PlayableId::Track(track.as_ref()));

            if first_chunk {
                if print {
                    info!("Replacing playlist with the first {:?} tracks", chunk.len());
                }
                let local_time = chrono::Local::now();
                let local_time_string = local_time.format("%m/%d/%Y").to_string();
                let description = format!("Release Radar playlist with songs from albums included. Created on 11/02/2023. Updated on {}.", local_time_string);
                self.client
                    .playlist_change_detail(
                        pl_id.clone(),
                        None,
                        None,
                        Some(description.as_str()),
                        None,
                    )
                    .await
                    .expect("Couldn't update description");
                self.client
                    .playlist_replace_items(pl_id.clone(), chunk_iterated)
                    .await
                    .expect("Track IDs should be assigned to chunk_iterated as type TrackID");
                first_chunk = false;
            } else {
                if print {
                    info!("Adding {:?} tracks to the playlist", chunk.len());
                }
                self.client
                    .playlist_add_items(pl_id.clone(), chunk_iterated, Option::None)
                    .await
                    .expect("Track IDs should be assigned to chunk_iterated as type TrackID");
            }
        }
    }
    fn print_all_album_track_ids(album_track_ids: &[Vec<TrackId>]) {
        album_track_ids
            .iter()
            .enumerate()
            .for_each(|(outer_index, album)| {
                album
                    .iter()
                    .enumerate()
                    .for_each(|(inner_index, track_id)| {
                        println!(
                            "Album {:?} - Track {:?}:\t{:?}",
                            outer_index + 1,
                            inner_index + 1,
                            track_id
                        );
                    });
                print_separator();
            });
    }
}
