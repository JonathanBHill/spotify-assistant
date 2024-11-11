use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};

use futures::stream::StreamExt;
use rspotify::{AuthCodeSpotify, ClientResult, scopes};
use rspotify::clients::BaseClient;
use rspotify::clients::pagination::Paginator;
use rspotify::model::{
    AlbumId, ArtistId, FullAlbum, FullArtist, FullTrack, PlayableId, SimplifiedAlbum,
    SimplifiedTrack, TrackId,
};
use tracing::{debug, error, info, Level};

use crate::enums::validation::BatchLimits;
use crate::traits::apis::Api;

pub struct ArtistXplorer {
    client: AuthCodeSpotify,
    artist_id: ArtistId<'static>,
    pub artist: FullArtist,
    pub albums: Vec<SimplifiedAlbum>,
}

impl Api for ArtistXplorer {
    fn select_scopes() -> HashSet<String> {
        scopes!("user-follow-read")
    }
}

impl ArtistXplorer {
    pub async fn new(artist_id: ArtistId<'static>) -> Self {
        let span = tracing::span!(Level::INFO, "ArtistXplorer.new");
        let _enter = span.enter();

        let client = Self::set_up_client(false, Some(Self::select_scopes())).await;
        let artist = match client.artist(artist_id.clone()).await {
            Ok(artist) => {
                info!(
                    "Data has been retrieved for the artist, '{}'.",
                    artist.name.clone()
                );
                artist
            }
            Err(error) => {
                error!(artist_id = ?artist_id.clone(), "Was not able to get data for the requested artist");
                panic!("Error: {:?}", error);
            }
        };
        let albums = Self::albums(client.artist_albums(artist_id.clone(), None, Some(Self::market()))).await;
        info!("Data has been retrieved for the artist, '{}'.", artist.name);
        ArtistXplorer {
            client,
            artist_id,
            artist,
            albums,
        }
    }
    pub async fn albums(mut paginated_albums: Paginator<'_, ClientResult<SimplifiedAlbum>>) -> Vec<SimplifiedAlbum> {
        let span = tracing::span!(Level::INFO, "ArtistXplorer.albums");
        let _enter = span.enter();

        let mut albums: Vec<SimplifiedAlbum> = Vec::new();
        while let Some(albums_page) = paginated_albums.next().await {
            match albums_page {
                Ok(album) => albums.push(album),
                Err(error) => panic!("ERROR: Was not able to get album from the requested artist.\nError information: {:?}", error)
            }
        }
        albums
    }
    pub fn album_by_type(&self) -> HashMap<&'static str, Vec<SimplifiedAlbum>> {
        let span = tracing::span!(Level::INFO, "ArtistXplorer.album_by_type");
        let _enter = span.enter();
        let mut albums = Vec::new();
        let mut singles = Vec::new();
        let mut compilations = Vec::new();
        let mut appears_on = Vec::new();
        self.albums.clone().iter().for_each(|album| {
            info!("{:?}", album.name);
            let alb_type = match album.album_type.clone() {
                None => { "n/a".to_string() }
                Some(typeofalbum) => { typeofalbum }
            };
            match alb_type.as_str() {
                "album" => {
                    info!("Name: {:?} | Type: {:?}", album.name, alb_type);
                    albums.push(album.clone());
                },
                "single" => {
                    info!("Name: {:?} | Type: {:?}", album.name, alb_type);
                    singles.push(album.clone());
                },
                "compilation" => {
                    info!("Name: {:?} | Type: {:?}", album.name, alb_type);
                    compilations.push(album.clone());
                },
                "appears_on" => {
                    info!("Name: {:?} | Type: {:?}", album.name, alb_type);
                    appears_on.push(album.clone());
                },
                _ => {
                    error!("Album type is not available for album: {:?}", album.name);
                },
            };
        });
        HashMap::from([("album", albums), ("single", singles), ("compilation", compilations), ("appears_on", appears_on)])
    }
    pub fn album_ids(&self) -> Vec<AlbumId> {
        let span = tracing::span!(Level::INFO, "ArtistXplorer.album_ids");
        let _enter = span.enter();

        self.albums.clone().iter().map(|album| {
            info!("{:?}", album.name);
            match album.id.clone() {
                Some(id) => id,
                None => panic!("Could not get album ID for album {}", album.name)
            }
        }).collect::<Vec<AlbumId>>()
    }
    pub async fn full_albums(&self) -> Vec<FullAlbum> {
        let span = tracing::span!(Level::INFO, "ArtistXplorer.full_albums");
        let _enter = span.enter();

        let mut full_albums = Vec::new();
        let limit = BatchLimits::Albums.get_limit();
        for album_id_chunk in self.album_ids().chunks(limit) {
            let full_album = match self.client.albums(album_id_chunk.to_vec(), Some(Self::market())).await {
                Ok(full_albums) => {
                    info!("{} albums have been requested.", full_albums.len());
                    full_albums
                }
                Err(error) => panic!("ERROR: Was not able to get album from the requested artist.\nError information: {:?}", error)
            };
            full_albums.extend(full_album);
        }
        full_albums
    }
    pub async fn full_tracks(&self) -> Vec<FullTrack> {
        let span = tracing::span!(Level::INFO, "ArtistXplorer.full_tracks");
        let _enter = span.enter();

        let mut full_tracks = Vec::new();
        let limit = BatchLimits::Tracks.get_limit();
        for track_id_chunk in self.track_ids().await.chunks(limit) {
            let full_track = match self.client.tracks(track_id_chunk.to_vec(), Some(Self::market())).await {
                Ok(full_tracks) => {
                    info!("{} tracks have been requested.", full_tracks.len());
                    full_tracks
                }
                Err(error) => panic!("ERROR: Was not able to get album from the requested artist.\nError information: {:?}", error)
            };
            full_tracks.extend(full_track);
        }
        full_tracks
    }
    pub async fn track_ids(&self) -> Vec<TrackId> {
        let span = tracing::span!(Level::INFO, "ArtistXplorer.track_ids");
        let _enter = span.enter();

        let mut track_ids = Vec::new();
        for track in self.tracks().await {
            info!("{:?}", track.name);
            match track.id.clone() {
                Some(id) => track_ids.push(id),
                None => panic!("Could not get track ID for track {}", track.name)
            }
        }
        track_ids
    }
    pub async fn tracks(&self) -> Vec<SimplifiedTrack> {
        let span = tracing::span!(Level::INFO, "ArtistXplorer.tracks");
        let _enter = span.enter();

        let mut album_tracks = Vec::new();

        for album in self.full_albums().await {
            let mut altracks = self.client.album_track(album.id.clone(), Some(Self::market()));

            while let Some(tracks_page) = altracks.next().await {
                match tracks_page {
                    Ok(track) => album_tracks.push(track),
                    Err(error) => panic!("ERROR: Was not able to get album from the requested artist.\nError information: {:?}", error)
                }
            }
        }
        album_tracks
    }
    pub async fn top_tracks_as_playable_ids(&self) -> Vec<PlayableId> {
        let span = tracing::span!(Level::INFO, "ArtistXplorer.top_tracks_as_playable_ids");
        let _enter = span.enter();

        match self.client.artist_top_tracks(self.artist_id.clone(), Some(Self::market())).await {
            Ok(top_tracks) => {
                top_tracks.iter().map(|track| {
                    info!("{:?}", track.name);
                    let track_id = match track.id.clone() {
                        Some(id) => id,
                        None => panic!("Could not get track ID for track {}", track.name)
                    };
                    PlayableId::Track(track_id)
                }).collect::<Vec<PlayableId>>()
            }
            Err(error) => panic!("ERROR: Was not able to get album from the requested artist.\nError information: {:?}", error)
        }
    }
    pub async fn related_artists(&self) -> Vec<FullArtist> {
        let span = tracing::span!(Level::INFO, "ArtistXplorer.related_artists");
        let _enter = span.enter();

        match self.client.artist_related_artists(self.artist_id.clone()).await {
            Ok(related) => {
                for (index, artist) in related.iter().enumerate() {
                    info!(
                        "{}). {} - genres: {:?} | {} followers | {} popularity",
                        index, artist.name, artist.genres, artist.followers.total,
                        artist.popularity
                    );
                }
                related
            }
            Err(error) => panic!("ERROR: Was not able to get album from the requested artist.\nError information: {:?}", error)
        }
    }
    pub fn genres(&self) -> Vec<String> {
        self.artist.genres.clone()
    }
}

#[cfg(test)]
mod tests {
    use rspotify::model::ArtistId;

    use super::*;

    #[tokio::test]
    async fn test_artist_xplr() {
        let artist_id = ArtistId::from_id("7u160I5qtBYZTQMLEIJmyz").unwrap();
        let artist_xplr = ArtistXplorer::new(artist_id.clone()).await;
        println!("{:?}", artist_xplr.genres());
        assert_eq!(artist_xplr.artist_id, artist_id);
    }

    #[tokio::test]
    async fn test_album_methods() {
        let artist_id = ArtistId::from_id("7u160I5qtBYZTQMLEIJmyz").unwrap();
        let artist_xplr = ArtistXplorer::new(artist_id.clone()).await;
        let albums = artist_xplr.full_albums().await;
        let artists = albums[0].artists.clone();
        let main_artist_id = artists[0].clone().id.unwrap();
        assert_eq!(main_artist_id, artist_id);
    }
}
