use std::collections::HashSet;

use rspotify::{AuthCodeSpotify, scopes};
use rspotify::clients::BaseClient;
use rspotify::model::{
    AlbumId, ArtistId, FullAlbum, FullArtist, FullTrack, Id, SimplifiedArtist, SimplifiedTrack,
    TrackId,
};
use tracing::{error, info, Level};

use crate::traits::apis::Api;

pub struct AlbumXplr {
    client: AuthCodeSpotify,
    album_id: AlbumId<'static>,
    full_album: FullAlbum,
}

impl Api for AlbumXplr {
    fn select_scopes() -> HashSet<String> {
        scopes!(
            "playlists-read-private",
            "playlists-read-collaborative",
            "playlists-modify-public",
            "playlists-modify-private"
        )
    }
}

impl AlbumXplr {
    pub async fn new(album_id: AlbumId<'static>) -> Self {
        let span = tracing::span!(Level::INFO, "AlbumXplr.new");
        let _enter = span.enter();

        let client = Self::set_up_client(false, Some(Self::select_scopes())).await;
        let full_album = match client.album(album_id.clone(), Some(Self::market())).await {
            Ok(album) => {
                info!(
                    "Data has been retrieved for the album, '{}'.",
                    album.name.clone()
                );
                album
            }
            Err(error) => {
                error!(album_id = ?album_id.clone(), "Was not able to get data for the requested album");
                panic!("Error: {:?}", error);
            }
        };
        info!(
            "Data has been retrieved for the album, '{}'.",
            full_album.name
        );
        AlbumXplr {
            client,
            album_id,
            full_album,
        }
    }
    pub fn album_id(&self) -> AlbumId {
        self.album_id.clone()
    }
    pub fn simple_artists(&self) -> Vec<SimplifiedArtist> {
        self.full_album.artists.to_vec()
    }
    pub fn artist_ids(&self, for_tracks: bool) -> Vec<ArtistId> {
        match for_tracks {
            true => {
                let tracks = self.simple_tracks();
                let mut all_ids = Vec::new();
                tracks.iter().for_each(|track| {
                    track.artists.iter().for_each(|artist| {
                        if !all_ids.contains(&artist.id.clone().expect("Could not get artist ID")) {
                            all_ids.push(artist.id.clone().expect("Could not get artist ID"));
                        }
                    });
                });
                all_ids
            }
            false => self
                .simple_artists()
                .iter()
                .map(|artist| artist.id.clone().expect("Could not get artist ID"))
                .collect(),
        }
    }
    pub async fn full_artists(&self, for_tracks: bool) -> Vec<FullArtist> {
        let span = tracing::span!(Level::INFO, "AlbumXplr.full_artists");
        let _enter = span.enter();

        let all_ids = self.artist_ids(for_tracks);
        let mut full_artists = vec![];
        let id_chunks = all_ids.chunks(50);
        for id_chunk in id_chunks {
            match self.client.artists(id_chunk.iter().cloned()).await {
                Ok(artist_batch) => artist_batch.iter().for_each(|full_artist| {
                    info!(
                        "Data has been retrieved for the artist, '{}'.",
                        full_artist.name.clone()
                    );
                    full_artists.push(full_artist.clone());
                }),
                Err(error) => {
                    error!(artist_id = ?id_chunk, "Was not able to get data for the requested artist");
                    panic!("Error: {:?}", error);
                }
            };
        }
        full_artists
    }
    pub fn simple_tracks(&self) -> Vec<SimplifiedTrack> {
        self.full_album.tracks.items.to_vec()
    }
    pub fn track_ids(&self) -> Vec<TrackId> {
        self.simple_tracks()
            .iter()
            .map(|track| track.id.clone().expect("Could not get track ID"))
            .collect()
    }
    pub async fn full_tracks(&self) -> Vec<FullTrack> {
        let span = tracing::span!(Level::INFO, "AlbumXplr.full_tracks");
        let _enter = span.enter();

        let simplified = self.simple_tracks();
        let mut full_tracks = vec![];
        for track in simplified {
            match track.id {
                Some(id) => {
                    let full_track = match self.client.track(id.clone(), Some(Self::market())).await
                    {
                        Ok(full_track) => {
                            info!(
                                "Data has been retrieved for the track, '{}'.",
                                full_track.name.clone()
                            );
                            full_track
                        }
                        Err(error) => {
                            error!(
                                track_id = id.id(),
                                "Was not able to get data for the requested track"
                            );
                            panic!("Error: {:?}", error);
                        }
                    };
                    full_tracks.push(full_track);
                }
                None => {
                    error!(
                        track_id = "None",
                        "Was not able to get data for the requested track"
                    );
                    panic!("Could not get track ID from the simplified track vector.")
                }
            };
        }
        full_tracks
    }
    pub fn genres(&self) -> Vec<String> {
        self.full_album.genres.clone()
    }
}

#[cfg(test)]
mod tests {
    use rspotify::model::AlbumId;

    use super::*;

    #[tokio::test]
    async fn test_new() {
        let album_id = AlbumId::from_id("24AElprtNKLkp18RzoddPb").unwrap();
        let album_xplr = AlbumXplr::new(album_id.clone()).await;
        assert_eq!(album_xplr.album_id, album_id);
    }

    #[tokio::test]
    async fn test_genres() {
        let album_id = AlbumId::from_id("24AElprtNKLkp18RzoddPb").unwrap();
        let album_xplr = AlbumXplr::new(album_id.clone()).await;
        let genres = album_xplr.genres();
        println!("{:?}", genres);
        assert_eq!(genres.len(), 0);
    }

    #[tokio::test]
    async fn test_track_methods() {
        let album_id = AlbumId::from_id("24AElprtNKLkp18RzoddPb").unwrap();
        let album_xplr = AlbumXplr::new(album_id.clone()).await;

        let track_ids = album_xplr.track_ids();
        let simple_tracks = album_xplr.simple_tracks();
        let full_tracks = album_xplr.full_tracks().await;
        assert_eq!(track_ids.len(), 13);
        assert_eq!(simple_tracks.len(), 13);
        assert_eq!(full_tracks.len(), 13);
    }

    #[tokio::test]
    async fn test_artist_methods() {
        let album_id = AlbumId::from_id("24AElprtNKLkp18RzoddPb").unwrap();
        let album_xplr = AlbumXplr::new(album_id.clone()).await;

        let simple_artists = album_xplr.simple_artists();
        assert_eq!(simple_artists.len(), 1);
        assert_eq!(simple_artists[0].name, "Kraddy".to_string());

        println!("Working on method, 'artist_ids'");
        let album_collaborators = album_xplr.artist_ids(false);
        let track_collaborators = album_xplr.artist_ids(true);
        assert_eq!(album_collaborators.len(), 1);
        assert_eq!(track_collaborators.len(), 14);

        println!("Working on method, 'full_artists'");
        let full_artists = album_xplr.full_artists(false).await;
        let full_collaborators = album_xplr.full_artists(true).await;
        assert_eq!(full_artists.len(), 1);
        assert_eq!(full_collaborators.len(), 14);
    }
}
