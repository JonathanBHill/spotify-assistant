use std::collections::HashSet;

use rspotify::{AuthCodeSpotify, scopes};
use rspotify::clients::BaseClient;
use rspotify::model::{AlbumId, ArtistId, FullPlaylist, FullTrack, PlayableItem, PlaylistId, SimplifiedAlbum, SimplifiedArtist, TrackId};
use tracing::{event, Level};

use crate::core::models::traits::Api;

pub struct ExplorePlaylist {
    pub client: AuthCodeSpotify,
    pub playlist_id: PlaylistId<'static>,
    pub full_playlist: FullPlaylist,
    duplicates: bool,
}

impl Api for ExplorePlaylist {
    fn select_scopes() -> HashSet<String> {
        scopes!(
            "playlist-read-private",
            "playlist-read-collaborative",
            "playlist-modify-public",
            "playlist-modify-private"
        )
    }
}

impl ExplorePlaylist {
    pub async fn new(playlist_id: PlaylistId<'static>, duplicates: bool) -> Self {
        let span = tracing::span!(Level::INFO, "ExplorePlaylist.new");
        let _enter = span.enter();

        let client = Self::set_up_client(
            false, Some(Self::select_scopes())
        ).await;
        event!(Level::DEBUG, "Client was initialized");
        let full_playlist = client.playlist(
            playlist_id.clone(), None, Some(Self::market())
        ).await.expect("Could not retrieve playlist");
        event!(Level::DEBUG, "Playlist data has been retrieved.");
        ExplorePlaylist {
            client,
            playlist_id,
            full_playlist,
            duplicates,
        }
    }
    pub fn tracks(&self) -> Vec<FullTrack> {
        self.full_playlist.tracks.items.iter().map(|track|
            match &track.track {
                Some(PlayableItem::Track(track)) => {
                    track.clone()
                }
                _ => {
                    event!(Level::ERROR, "Could not get track from playlist");
                    panic!("Could not get track from playlist");
                }
            }).collect::<Vec<FullTrack>>()
    }
    pub fn albums(&self) -> Vec<SimplifiedAlbum> {
        self.tracks().iter().map(|track| track.album.clone()).collect::<Vec<SimplifiedAlbum>>()
    }
    pub fn artists(&self) -> Vec<SimplifiedArtist> {
        let span = tracing::span!(Level::INFO, "ExplorePlaylist.artists");
        let _enter = span.enter();
        let pl_name = self.full_playlist.name.clone();
        event!(Level::INFO, "Retrieving artists from the '{pl_name}' playlist");

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
    pub fn album_ids(&self) -> Vec<AlbumId> {
        self.tracks().iter().map(|track| {
            track.album.id.clone().expect("Could not get album id from track")
        }).collect::<Vec<AlbumId>>()
    }
    pub async fn artist_ids_expanded(&self) -> Vec<ArtistId> {
        let span = tracing::span!(Level::INFO, "ExplorePlaylist.artist_ids_expanded");
        let _enter = span.enter();

        let mut artist_ids = Vec::new();
        for album_chunk in self.album_ids().chunks(20) {
            event!(Level::DEBUG, "Current album chunk: {:?}", album_chunk.to_vec());
            let albums = self.client.albums(album_chunk.to_vec(), Some(Self::market())).await.expect("Could not retrieve albums");
            albums.iter().for_each(|album| {
                artist_ids.extend(album.artists.iter().map(|artist| artist.id.clone().expect("Could not get artist id from album")).collect::<Vec<ArtistId>>());
            });
        };
        if !self.duplicates {
            artist_ids = Self::clean_duplicates_hashset(artist_ids);
        }
        artist_ids
    }
    pub fn artist_ids_original(&self) -> Vec<ArtistId> {
        let span = tracing::span!(Level::INFO, "ExplorePlaylist.artist_ids_original");
        let _enter = span.enter();
        self.tracks().iter().flat_map(|track| {
            track.artists.iter().flat_map(|artist| {
                Some(artist.id.clone().expect("Could not get artist id from track"))
            }).collect::<Vec<ArtistId>>()
        }).collect::<Vec<ArtistId>>()
    }
    pub async fn track_ids_expanded(&self) -> Vec<TrackId> {
        let span = tracing::span!(Level::INFO, "ExplorePlaylist.track_ids_expanded");
        let _enter = span.enter();

        let mut track_ids = Vec::new();
        for album_chunk in self.album_ids().chunks(20) {
            event!(Level::DEBUG, "Current album chunk: {:?}", album_chunk.to_vec());
            let albums = self.client.albums(album_chunk.to_vec(), Some(Self::market())).await.expect("Could not retrieve albums");
            albums.iter().for_each(|album| {
                track_ids.extend(album.tracks.items.iter().map(|track| track.id.clone().expect("Could not get track id from album")).collect::<Vec<TrackId>>());
            });
        };
        if !self.duplicates {
            track_ids = Self::clean_duplicates_hashset(track_ids);
        }
        track_ids
    }
    pub fn track_ids_original(&self) -> Vec<TrackId> {
        let span = tracing::span!(Level::INFO, "ExplorePlaylist.track_ids_original");
        let _enter = span.enter();
        self.tracks().iter().map(|track| {
            track.id.clone().expect("Could not get track id from track")
        }).collect::<Vec<TrackId>>()
    }
}
