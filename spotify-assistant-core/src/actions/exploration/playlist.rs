use futures::StreamExt;
use rspotify::clients::{BaseClient, OAuthClient};
use rspotify::model::{AlbumId, ArtistId, FullAlbum, FullPlaylist, FullTrack, PlayableItem, PlaylistId, SimplifiedAlbum, SimplifiedArtist, TrackId};
use rspotify::{scopes, AuthCodeSpotify};
use std::collections::{HashMap, HashSet};
use tracing::{event, Level};

use crate::traits::apis::Api;

pub struct PlaylistXplr {
    pub client: AuthCodeSpotify,
    pub playlist_id: PlaylistId<'static>,
    pub full_playlist: FullPlaylist,
    pub tracks: Vec<FullTrack>,
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
        let full_playlist = Self::instantiate_playlist(client.clone(), playlist_id.clone()).await;
        let tracks = Self::instantiate_playlist_tracks(client.clone(), playlist_id.clone()).await;
        PlaylistXplr {
            client,
            playlist_id,
            full_playlist,
            tracks,
            duplicates,
        }
    }
    async fn instantiate_playlist_tracks(client: AuthCodeSpotify, playlist_id: PlaylistId<'_>) -> Vec<FullTrack> {
        let span = tracing::span!(Level::INFO, "ExplorePlaylist.instantiate_playlist_tracks");
        let _enter = span.enter();
        event!(Level::TRACE, "Retrieving playlist tracks");
        let mut playlist_items = client.playlist_items(playlist_id, None, Some(Self::market()));
        let mut tracks = Vec::new();
        while let Some(items) = playlist_items.next().await {
            match items {
                Ok(item) => {
                    match item.track {
                        Some(PlayableItem::Track(track)) => tracks.push(track),
                        _ => { panic!("Could not get track from playlists") }
                    };
                }
                Err(err) => {
                    event!(Level::ERROR, "Could not retrieve playlist items: {:?}", err);
                    continue;
                }
            }
        }
        event!(Level::TRACE, "Playlist tracks have been retrieved.");
        tracks
    }
    async fn instantiate_playlist(client: AuthCodeSpotify, playlist_id: PlaylistId<'_>) -> FullPlaylist {
        let span = tracing::span!(Level::INFO, "ExplorePlaylist.instantiate_playlist");
        let _enter = span.enter();
        event!(Level::TRACE, "Retrieving playlist data");
        match client.playlist(playlist_id, None, Some(Self::market())).await {
            Ok(pl) => {
                event!(Level::TRACE, "Playlist data has been retrieved.");
                pl
            },
            Err(err) => {
                event!(Level::ERROR, "Could not retrieve playlist: {:?}", err);
                panic!("Could not retrieve playlist: {:?}", err);
            }
        }
    }
    pub fn tracks(&self) -> Vec<FullTrack> {
        self.tracks.clone()
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
    pub async fn find_liked_songs(&self) -> HashMap<&str, Vec<FullTrack>> {
        let span = tracing::span!(Level::INFO, "ExplorePlaylist.find_liked_songs");
        let _enter = span.enter();
        let mut liked = Vec::new();
        let mut not_liked = Vec::new();
        let batch_size = 50;

        let mut liked_songz = Vec::new();
        for chunk in self.track_ids().chunks(batch_size) {
            let chunk_iter = chunk.iter().cloned();
            match self.client.current_user_saved_tracks_contains(chunk_iter).await {
                Ok(liked) => {
                    event!(Level::DEBUG, "batch size: {:?}", liked.len());
                    liked.iter().for_each(|liked| { liked_songz.push(liked.to_owned()) });
                }
                Err(err) => {
                    event!(Level::ERROR, "Could not retrieve liked songs: {:?}", err);
                    panic!();
                }
            }
        }
        event!(Level::DEBUG, "Liked songs length: {:?}", liked_songz.len());
        event!(Level::DEBUG, "Track ids length: {:?}", self.track_ids().len());
        let zipped = self.tracks().into_iter().zip(liked_songz.into_iter()).collect::<Vec<(FullTrack, bool)>>();
        zipped.into_iter().for_each(|(track, is_liked)| {
            if is_liked {
                liked.push(track.to_owned());
            } else {
                not_liked.push(track.to_owned());
            }
        });
        HashMap::from([("liked", liked), ("not_liked", not_liked)])
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
            let albums: Vec<FullAlbum> = match self.client.albums(album_chunk.to_vec(), Some(Self::market())).await {
                Ok(albums) => albums,
                Err(err) => {
                    event!(Level::ERROR, "Could not retrieve albums: {:?}", err);
                    panic!("Could not retrieve albums: {:?}", err);
                }
            };
            albums.iter().for_each(|album| {
                let album_artists = album.tracks.items.iter().flat_map(|track| {
                    track.artists.to_vec()
                }).collect::<Vec<SimplifiedArtist>>();
                let artist_ids = album_artists.iter().map(|artist| {
                    match artist.id.clone() {
                        Some(id) => id,
                        None => panic!("Could not get artist id from album")
                    }
                }).collect::<Vec<ArtistId>>();
                let cleaned_artist_ids = if !self.duplicates {
                    Self::clean_duplicate_id_vector(artist_ids.clone())
                } else {
                    artist_ids.clone()
                };
                let mut pushed: Vec<ArtistId> = Vec::new();
                let cleaned_artists = album_artists.iter().filter_map(|artist| {
                    let artist_id_to_filter = match artist.id.clone() {
                        Some(id) => id,
                        None => panic!("Could not get artist id from album")
                    };
                    let contains = cleaned_artist_ids.contains(&artist_id_to_filter);
                    let is_duplicate = pushed.contains(&artist_id_to_filter);
                    if contains && !is_duplicate {
                        pushed.push(artist_id_to_filter);
                        Some(artist.to_owned())
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
    pub fn track_ids(&self) -> Vec<TrackId> {
        let span = tracing::span!(Level::INFO, "ExplorePlaylist.track_ids_original");
        let _enter = span.enter();
        event!(Level::INFO, "Retrieving track ids from the playlist. Track count: {:?}", self.tracks().len());
        self.tracks()
            .iter()
            .map(|track| match track.id.clone() {
                Some(id) => id,
                None => panic!("Could not get track id from track")
            })
            .collect::<Vec<TrackId>>()
    }
}
