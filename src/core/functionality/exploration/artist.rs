use std::borrow::Borrow;
use std::collections::HashSet;

use futures::stream::StreamExt;
use rspotify::{AuthCodeSpotify, scopes};
use rspotify::clients::BaseClient;
use rspotify::model::{
    AlbumId, ArtistId, FullAlbum, FullArtist, FullTrack, PlayableId, SimplifiedAlbum,
    SimplifiedTrack, TrackId,
};
use tracing::{debug, error, info, Level};

use crate::core::enums::validation::BatchLimits;
use crate::core::models::traits::Api;

pub struct ArtistXplr {
    client: AuthCodeSpotify,
    log_level: Level,
    artist_id: ArtistId<'static>,
    artist: FullArtist,
    discography_full_album: Vec<FullAlbum>,
    discography_simple_album_owned: Vec<SimplifiedAlbum>,
    discography_simple_album_guest: Vec<SimplifiedAlbum>,
    discography_album_ids_owned: Vec<AlbumId<'static>>,
    discography_album_ids_guest: Vec<AlbumId<'static>>,
    discography_full_tracks: Vec<FullTrack>,
    discography_simple_tracks: Vec<SimplifiedTrack>,
    discography_track_ids: Vec<TrackId<'static>>,
}

impl Api for ArtistXplr {
    fn select_scopes() -> HashSet<String> {
        scopes!("user-follow-read")
    }
}

impl ArtistXplr {
    pub async fn new(artist_id: ArtistId<'static>, log_level: Option<Level>) -> Self {
        let span = match log_level.unwrap_or(Level::ERROR) {
            Level::ERROR => tracing::span!(Level::ERROR, "ArtistXplr.new"),
            Level::INFO => tracing::span!(Level::INFO, "ArtistXplr.new"),
            Level::DEBUG => tracing::span!(Level::DEBUG, "ArtistXplr.new"),
            Level::TRACE => tracing::span!(Level::TRACE, "ArtistXplr.new"),
            _ => tracing::span!(Level::INFO, "ArtistXplr.new"),
        };
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
        info!("Data has been retrieved for the artist, '{}'.", artist.name);
        let mut new_self = ArtistXplr {
            client: client.clone(),
            log_level: log_level.unwrap_or(Level::ERROR),
            artist_id: artist_id.clone(),
            artist: artist.clone(),
            discography_simple_album_owned: Vec::new(),
            discography_simple_album_guest: Vec::new(),
            discography_album_ids_owned: Vec::new(),
            discography_album_ids_guest: Vec::new(),
            discography_full_album: Vec::new(),
            discography_simple_tracks: Vec::new(),
            discography_track_ids: Vec::new(),
            discography_full_tracks: Vec::new(),
        };
        let (
            discography_album_ids_owned,
            discography_simple_album_owned,
            discography_album_ids_guest,
            discography_simple_album_guest,
        ) = new_self.simple_album_id_discography().await;
        new_self.discography_simple_album_owned = discography_simple_album_owned;
        new_self.discography_simple_album_guest = discography_simple_album_guest;
        new_self.discography_album_ids_owned = discography_album_ids_owned;
        new_self.discography_album_ids_guest = discography_album_ids_guest;
        let discography_full_album = new_self.discography_full_albums(None).await;
        new_self.discography_full_album = discography_full_album;
        let (discography_track_ids, discography_simple_tracks) =
            new_self.simple_track_ids_discography().await;
        new_self.discography_simple_tracks = discography_simple_tracks;
        new_self.discography_track_ids = discography_track_ids;
        let discography_full_tracks = new_self.discography_for_full_tracks().await;
        new_self.discography_full_tracks = discography_full_tracks;
        info!("All discography information was successfully obtained.");
        new_self
    }
    pub async fn discography_for_full_tracks(&self) -> Vec<FullTrack> {
        let span = match self.log_level {
            Level::ERROR => tracing::span!(Level::ERROR, "ArtistXplr.track-disco-full"),
            Level::INFO => tracing::span!(Level::INFO, "ArtistXplr.track-disco-full"),
            Level::DEBUG => tracing::span!(Level::DEBUG, "ArtistXplr.track-disco-full"),
            Level::TRACE => tracing::span!(Level::TRACE, "ArtistXplr.track-disco-full"),
            _ => tracing::span!(Level::INFO, "ArtistXplr.track-disco-full"),
        };
        let _enter = span.enter();

        let mut return_tracks = Vec::new();
        let limit = BatchLimits::Tracks.get_limit();
        for id_chunk in self.discography_track_ids.chunks(limit) {
            let full_tracks = match self
                .client
                .tracks(id_chunk.to_vec(), Some(Self::market()))
                .await
            {
                Ok(full_tracks) => {
                    info!("{} tracks have been requested.", full_tracks.len());
                    full_tracks
                }
                Err(error) => {
                    error!(track_id = ?id_chunk, "Was not able to get data for the requested track");
                    panic!("Error: {:?}", error);
                }
            };
            return_tracks.extend(full_tracks);
        }
        return_tracks
    }
    pub async fn simple_track_ids_discography(
        &self,
    ) -> (Vec<TrackId<'static>>, Vec<SimplifiedTrack>) {
        let span = match self.log_level {
            Level::ERROR => tracing::span!(Level::ERROR, "ArtistXplr.simple-track-id-disco"),
            Level::INFO => tracing::span!(Level::INFO, "ArtistXplr.simple-track-id-disco"),
            Level::DEBUG => tracing::span!(Level::DEBUG, "ArtistXplr.simple-track-id-disco"),
            Level::TRACE => tracing::span!(Level::TRACE, "ArtistXplr.simple-track-id-disco"),
            _ => tracing::span!(Level::INFO, "ArtistXplr.simple-track-id-disco"),
        };
        let _enter = span.enter();

        let full_albums = self.discography_full_album.clone();
        let mut album_tracks: Vec<SimplifiedTrack> = Vec::new();
        let mut album_track_ids: Vec<TrackId> = Vec::new();
        for album in full_albums {
            let tracks = album.tracks.clone();
            if tracks.offset > 50 {
                info!(total = ?tracks.total, "Album has more than 50 tracks, manual iteration is required.");
                let loops = tracks.offset / 50;
                let mut manual_album = Vec::new();
                for index in 0..loops {
                    let offset = index * 50;
                    let album_iteration = match self
                        .client
                        .album_track_manual(
                            album.id.clone(),
                            Some(Self::market()),
                            Some(BatchLimits::AlbumTracks.get_limit() as u32),
                            Some(offset),
                        )
                        .await
                    {
                        Ok(album) => {
                            debug!(offset = ?offset, iterations = ?loops, current_item_count = ?album.items.len());
                            album.items
                        }
                        Err(error) => {
                            error!(album_id = ?album.id.clone(), "Was not able to get data for the requested album");
                            panic!("Error: {:?}", error);
                        }
                    };
                    manual_album.extend(album_iteration);
                }
                manual_album.iter().for_each(|track| {
                    album_tracks.push(track.clone());
                    album_track_ids.push(track.id.clone().unwrap());
                });
            } else {
                album.tracks.items.iter().for_each(|track| {
                    album_tracks.push(track.clone());
                    album_track_ids.push(track.id.clone().unwrap());
                });
            }
        }
        (album_track_ids, album_tracks)
    }
    async fn discography_full_albums(&self, guest: Option<bool>) -> Vec<FullAlbum> {
        let span = match self.log_level {
            Level::ERROR => tracing::span!(Level::ERROR, "ArtistXplr.discography-full-albums"),
            Level::INFO => tracing::span!(Level::INFO, "ArtistXplr.discography-full-albums"),
            Level::DEBUG => tracing::span!(Level::DEBUG, "ArtistXplr.discography-full-albums"),
            Level::TRACE => tracing::span!(Level::TRACE, "ArtistXplr.discography-full-albums"),
            _ => tracing::span!(Level::INFO, "ArtistXplr.discography-full-albums"),
        };
        let _enter = span.enter();

        let mut return_albums: Vec<FullAlbum> = Vec::new();
        let limit = BatchLimits::Albums.get_limit();
        let discography_album_ids = match guest {
            Some(true) => self.discography_album_ids_guest.clone(),
            _ => self.discography_album_ids_owned.clone(),
        };
        for id_chunk in discography_album_ids.chunks(limit) {
            let full_album = match self
                .client
                .albums(id_chunk.to_vec(), Some(Self::market()))
                .await
            {
                Ok(full_albums) => {
                    info!("{} albums have been requested.", full_albums.len());
                    full_albums
                }
                Err(error) => {
                    error!(album_id = ?id_chunk, "Was not able to get data for the requested album");
                    panic!("Error: {:?}", error);
                }
            };
            return_albums.extend(full_album);
        }
        return_albums
    }
    pub async fn simple_album_id_discography(
        &self,
    ) -> (
        Vec<AlbumId<'static>>,
        Vec<SimplifiedAlbum>,
        Vec<AlbumId<'static>>,
        Vec<SimplifiedAlbum>,
    ) {
        let span = match self.log_level {
            Level::ERROR => tracing::span!(Level::ERROR, "ArtistXplr.discography-simple&ids"),
            Level::INFO => tracing::span!(Level::INFO, "ArtistXplr.discography-simple&ids"),
            Level::DEBUG => tracing::span!(Level::DEBUG, "ArtistXplr.discography-simple&ids"),
            Level::TRACE => tracing::span!(Level::TRACE, "ArtistXplr.discography-simple&ids"),
            _ => tracing::span!(Level::INFO, "ArtistXplr.discography-simple&ids"),
        };
        let _enter = span.enter();

        let mut albums =
            self.client
                .artist_albums(self.artist_id.clone(), None, Some(Self::market()));
        let mut index = 0;

        let mut albums_as_host: Vec<SimplifiedAlbum> = Vec::new();
        let mut albums_as_guest: Vec<SimplifiedAlbum> = Vec::new();
        let mut album_ids_as_host: Vec<AlbumId> = Vec::new();
        let mut album_ids_as_guest: Vec<AlbumId> = Vec::new();
        while let Some(albums_page) = albums.next().await {
            match albums_page {
                Ok(album) => {
                    let album_type = match album.album_type.borrow() {
                        Some(borrowed_album_type) => borrowed_album_type.to_string(),
                        None => "None".to_string(),
                    };
                    let album_group = match album.album_group.borrow() {
                        Some(borrowed_album_group) => borrowed_album_group.to_string(),
                        None => "None".to_string(),
                    };
                    let album_type = format!("{}/{}", album_type, album_group);
                    info!("{} ({}): {}", index + 1, album_type, album.name);
                    println!("{:?}", album.name);
                    index += 1;
                    match album_group.as_str() {
                        "album" => {
                            album_ids_as_host.push(album.id.clone().expect("oof"));
                            albums_as_host.push(album);
                        }
                        "single" => {
                            album_ids_as_host.push(album.id.clone().expect("oof"));
                            albums_as_host.push(album);
                        }
                        "appears_on" => {
                            album_ids_as_guest.push(album.id.clone().expect("oof"));
                            albums_as_guest.push(album);
                        }
                        _ => {
                            album_ids_as_guest.push(album.id.clone().expect("oof"));
                            albums_as_guest.push(album);
                        }
                    }
                }
                Err(error) => {
                    error!(artist_id = ?self.artist_id.clone(), "Was not able to get data for the requested artist");
                    panic!("Error: {:?}", error);
                }
            }
        }
        (
            album_ids_as_host,
            albums_as_host,
            album_ids_as_guest,
            albums_as_guest,
        )
    }
    pub async fn related_artists(&self) -> Vec<FullArtist> {
        let span = match self.log_level {
            Level::ERROR => tracing::span!(Level::ERROR, "ArtistXplr.related-artists"),
            Level::INFO => tracing::span!(Level::INFO, "ArtistXplr.related-artists"),
            Level::DEBUG => tracing::span!(Level::DEBUG, "ArtistXplr.related-artists"),
            Level::TRACE => tracing::span!(Level::TRACE, "ArtistXplr.related-artists"),
            _ => tracing::span!(Level::INFO, "ArtistXplr.related-artists"),
        };
        let _enter = span.enter();

        match self
            .client
            .artist_related_artists(self.artist_id.clone())
            .await
        {
            Ok(related) => {
                for (index, artist) in related.iter().enumerate() {
                    // let t = artist.followers.total
                    info!(
                        "{}: {} | {:?} | {} | {} |",
                        index,
                        artist.name,
                        artist.genres,
                        artist.popularity,
                        artist.followers.total
                    );
                }
                related
            }
            Err(error) => {
                error!(artist_id = ?self.artist_id.clone(), "Was not able to get data for the requested artist");
                panic!("Error: {:?}", error);
            }
        }
    }
    pub async fn top_tracks_as_playable_ids(&self) -> Vec<PlayableId> {
        match self
            .client
            .artist_top_tracks(self.artist_id.clone(), Some(Self::market()))
            .await
        {
            Ok(top_tracks) => top_tracks
                .iter()
                .map(|track| {
                    info!("{:?}", track.name);
                    let track_id = match track.id.clone() {
                        Some(id) => id,
                        None => panic!("Could not get track ID for track {}", track.name),
                    };
                    PlayableId::Track(track_id)
                })
                .collect::<Vec<PlayableId>>(),
            Err(error) => {
                error!(artist_id = ?self.artist_id.clone(), "Was not able to get data for the requested artist");
                panic!("Error: {:?}", error);
            }
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
        let artist_xplr = ArtistXplr::new(artist_id.clone(), None).await;
        println!("{:?}", artist_xplr.genres());
        assert_eq!(artist_xplr.artist_id, artist_id);
    }

    #[tokio::test]
    async fn test_album_methods() {
        let artist_id = ArtistId::from_id("7u160I5qtBYZTQMLEIJmyz").unwrap();
        let artist_xplr = ArtistXplr::new(artist_id.clone(), None).await;
        let albums = artist_xplr.discography_full_album.clone();
        let artists = albums[0].artists.clone();
        let main_artist_id = artists[0].clone().id.unwrap();
        assert_eq!(main_artist_id, artist_id);
    }
}
