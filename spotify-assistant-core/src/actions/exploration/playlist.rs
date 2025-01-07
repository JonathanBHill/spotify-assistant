use std::collections::{HashMap, HashSet};

use rspotify::clients::{BaseClient, OAuthClient};
use rspotify::model::{AlbumId, ArtistId, FullAlbum, FullPlaylist, FullTrack, PlayableItem, PlaylistId, SimplifiedAlbum, SimplifiedArtist, TrackId};
use rspotify::{scopes, AuthCodeSpotify};
use tracing::{event, Level};

use crate::traits::apis::Api;

pub struct PlaylistXplr {
    pub client: AuthCodeSpotify,
    pub playlist_id: PlaylistId<'static>,
    pub full_playlist: FullPlaylist,
    duplicates: bool,
}

impl Api for PlaylistXplr {
    fn select_scopes() -> HashSet<String> {
        scopes!(
            "playlist-read-private",
            "playlist-read-collaborative",
            "playlist-modify-public",
            "playlist-modify-private"
        )
    }
}

impl PlaylistXplr {
    pub async fn new(playlist_id: PlaylistId<'static>, duplicates: bool) -> Self {
        let span = tracing::span!(Level::INFO, "ExplorePlaylist.new");
        let _enter = span.enter();

        let client = Self::set_up_client(false, Some(Self::select_scopes())).await;
        event!(Level::TRACE, "Client was initialized");
        let full_playlist = client
            .playlist(playlist_id.clone(), None, Some(Self::market()))
            .await
            .expect("Could not retrieve playlists");
        event!(Level::TRACE, "Playlist data has been retrieved.");
        PlaylistXplr {
            client,
            playlist_id,
            full_playlist,
            duplicates,
        }
    }
    pub fn tracks(&self) -> Vec<FullTrack> {
        self.full_playlist
            .tracks
            .items
            .iter()
            .map(|track| match &track.track {
                Some(PlayableItem::Track(track)) => track.clone(),
                _ => {
                    event!(Level::ERROR, "Could not get track from playlists");
                    panic!("Could not get track from playlists");
                }
            })
            .collect::<Vec<FullTrack>>()
    }
    pub fn albums(&self) -> Vec<SimplifiedAlbum> {
        self.tracks()
            .iter()
            .map(|track| track.album.clone())
            .collect::<Vec<SimplifiedAlbum>>()
    }
    pub fn artists(&self) -> Vec<SimplifiedArtist> {
        let span = tracing::span!(Level::INFO, "ExplorePlaylist.artists");
        let _enter = span.enter();
        let pl_name = self.full_playlist.name.clone();
        event!(
            Level::INFO,
            "Retrieving artists from the '{pl_name}' playlists"
        );

        let mut artists_vec = Vec::new();
        self.tracks().iter().for_each(|track| {
            event!(Level::DEBUG, "Track: {:?}", track.clone().name);
            track.artists.iter().for_each(|artist| {
                event!(Level::DEBUG, "\tArtist: {:?}", artist.clone().name);
                artists_vec.push(artist.clone());
            });
        });
        artists_vec
    }
    pub fn artists_by_track(&self) -> HashMap<String, Vec<SimplifiedArtist>> {
        let span = tracing::span!(Level::INFO, "ExplorePlaylist.artists_by_track");
        let _enter = span.enter();
        let pl_name = self.full_playlist.name.clone();
        event!(
            Level::INFO,
            "Retrieving artists from the '{pl_name}' playlists"
        );
        let mut artists_map = HashMap::new();
        self.tracks().iter().for_each(|track| {
            let track_name = track.name.clone();
            let artists = track.artists.clone();
            event!(Level::TRACE, "Adding {:?} artists from track {:?}", artists.len(), track_name);
            artists_map.insert(track_name.clone(), artists.clone());
            event!(Level::DEBUG,
                "Hashmap length: {:?} | Key: {:?} | Length of value {:?}",
                artists_map.len(), track_name, artists.len()
            );
        });
        artists_map
    }
    pub fn album_ids(&self) -> Vec<AlbumId> {
        self.tracks()
            .iter()
            .map(|track| {
                track
                    .album
                    .id
                    .clone()
                    .expect("Could not get album id from track")
            })
            .collect::<Vec<AlbumId>>()
    }
    pub async fn artist_ids_expanded(&self) -> Vec<ArtistId> {
        let span = tracing::span!(Level::INFO, "ExplorePlaylist.artist_ids_expanded");
        let _enter = span.enter();

        let mut artist_ids = Vec::new();
        for album_chunk in self.album_ids().chunks(20) {
            event!(
                Level::DEBUG,
                "Current album chunk: {:?}",
                album_chunk.to_vec()
            );
            let albums = self
                .client
                .albums(album_chunk.to_vec(), Some(Self::market()))
                .await
                .expect("Could not retrieve albums");
            albums.iter().for_each(|album| {
                artist_ids.extend(
                    album
                        .artists
                        .iter()
                        .map(|artist| {
                            artist
                                .id
                                .clone()
                                .expect("Could not get artist id from album")
                        })
                        .collect::<Vec<ArtistId>>(),
                );
            });
        }
        if !self.duplicates {
            artist_ids = Self::clean_duplicate_id_vector(artist_ids);
        }
        artist_ids
    }
    pub fn artist_ids_original(&self) -> Vec<ArtistId> {
        let span = tracing::span!(Level::INFO, "ExplorePlaylist.artist_ids_original");
        let _enter = span.enter();
        self.tracks()
            .iter()
            .flat_map(|track| {
                track
                    .artists
                    .iter()
                    .flat_map(|artist| {
                        Some(
                            artist
                                .id
                                .clone()
                                .expect("Could not get artist id from track"),
                        )
                    })
                    .collect::<Vec<ArtistId>>()
            })
            .collect::<Vec<ArtistId>>()
    }
    pub async fn track_ids_expanded(&self) -> Vec<TrackId> {
        let span = tracing::span!(Level::INFO, "ExplorePlaylist.track_ids_expanded");
        let _enter = span.enter();

        let mut track_ids: Vec<TrackId> = Vec::new();
        for album_chunk in self.album_ids().chunks(20) {
            event!(
                Level::DEBUG,
                "Current album chunk: {:?}",
                album_chunk.to_vec()
            );
            let albums: Vec<FullAlbum> = self
                .client
                .albums(album_chunk.to_vec(), Some(Self::market()))
                .await
                .expect("Could not retrieve albums");
            albums.iter().for_each(|album| {
                track_ids.extend(
                    album
                        .tracks
                        .items
                        .iter()
                        .map(|track| track.id.clone().expect("Could not get track id from album"))
                        .collect::<Vec<TrackId>>(),
                );
            });
        }
        if !self.duplicates {
            track_ids = Self::clean_duplicate_id_vector(track_ids);
        }
        track_ids
    }
    pub async fn artists_by_album(&self) -> HashMap<String, Vec<SimplifiedArtist>> {
        let span = tracing::span!(Level::INFO, "ExplorePlaylist.artists_by_album");
        let _enter = span.enter();

        let mut artists_map = HashMap::new();
        for album_chunk in self.album_ids().chunks(20) {
            event!(
                Level::DEBUG,
                "Current album chunk: {:?}",
                album_chunk.to_vec()
            );
            let albums: Vec<FullAlbum> = self
                .client
                .albums(album_chunk.to_vec(), Some(Self::market()))
                .await
                .expect("Could not retrieve albums");
            albums.iter().for_each(|album| {
                let album_artists = album
                    .tracks
                    .items
                    .iter()
                    .flat_map(|track| {
                        let x = track.artists
                                     .iter()
                                     .map(|artist| artist.clone())
                                     .collect::<Vec<SimplifiedArtist>>();
                        x
                    })
                    .collect::<Vec<SimplifiedArtist>>();
                let artist_ids = album_artists
                    .iter()
                    .map(|artist| artist.id.clone().expect("Could not get artist id from album"))
                    .collect::<Vec<ArtistId>>();
                let cleaned_artist_ids = if !self.duplicates {
                    Self::clean_duplicate_id_vector(artist_ids.clone())
                } else {
                    artist_ids.clone()
                };
                let mut pushed = Vec::new();
                let cleaned_artists = album_artists.clone()
                                                   .into_iter()
                                                   .filter_map(|artist| {
                                                       let artist_id_to_filter = artist.id.clone().expect("Could not get artist id from album");
                                                       let contains = cleaned_artist_ids.contains(&artist_id_to_filter);
                                                       let is_duplicate = pushed.contains(&artist_id_to_filter);
                                                       if contains && !is_duplicate {
                                                           pushed.push(artist_id_to_filter);
                                                           Some(artist)
                                                       } else {
                                                           None
                                                       }
                                                   }).collect::<Vec<SimplifiedArtist>>();
                event!(
                    Level::DEBUG,
                    "raw_id_length: {:?} | cleaned_length: {:?} | artist_raw_length: {:?} | artist_cleaned_length: {:?}",
                    artist_ids.len(),
                    cleaned_artist_ids.len(),
                    album_artists.clone().len(),
                    cleaned_artists.len()
                );
                artists_map.insert(album.name.clone(), cleaned_artists);
            });
        }
        artists_map
    }
    pub fn track_ids_original(&self) -> Vec<TrackId> {
        let span = tracing::span!(Level::INFO, "ExplorePlaylist.track_ids_original");
        let _enter = span.enter();
        self.tracks()
            .iter()
            .map(|track| track.id.clone().expect("Could not get track id from track"))
            .collect::<Vec<TrackId>>()
    }
}
