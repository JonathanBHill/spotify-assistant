// use std::borrow::Borrow;
use std::collections::{BTreeMap, HashMap, HashSet};

use chrono::{NaiveDate, NaiveDateTime};
use futures::stream::StreamExt;
use pbr::ProgressBar;
use rspotify::clients::pagination::Paginator;
use rspotify::clients::BaseClient;
use rspotify::model::{AlbumId, ArtistId, FullAlbum, FullArtist, FullTrack, PlayableId, SimplifiedAlbum, SimplifiedTrack, TrackId};
use rspotify::{scopes, AuthCodeSpotify, ClientError, ClientResult};
use tracing::{error, info, Level};

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
    pub async fn new(artist_id: ArtistId<'static>) -> Result<Self, ClientError> {
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
                return Err(error);
            }
        };
        let albums = Self::albums(client.artist_albums(artist_id.clone(), None, Some(Self::market()))).await;
        info!("Data has been retrieved for the artist, '{}'.", artist.name);
        Ok(ArtistXplorer {
            client,
            artist_id,
            artist,
            albums,
        })
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
    pub fn albums_by_date(&self, unit: Option<&str>) -> BTreeMap<String, Vec<SimplifiedAlbum>> {
        let span = tracing::span!(Level::INFO, "ArtistXplorer.albums_by_date");
        let _enter = span.enter();

        let mut final_hash: BTreeMap<String, Vec<SimplifiedAlbum>> = BTreeMap::new();
        let mut annual_album_group: Vec<SimplifiedAlbum> = Vec::new();
        self.albums.clone().iter().for_each(|album| {
            let release_date = match album.release_date.clone() {
                Some(date) => date.split("-").map(|s| s.to_string()).collect::<Vec<String>>(),
                None => panic!("Could not get release date for album {}", album.name)
            };
            info!("Name: {:?} | Release Date: {:?}", album.name, release_date);
            let btree_key = match unit {
                Some("month") => {
                    let key = release_date[1].to_string();
                    key
                },
                Some("yearmonth") | Some("monthyear") => {
                    let year = release_date[0].to_string();
                    let month = release_date[1].to_string();
                    format!("{}_{}", year, month)
                },
                _ => {
                    release_date[0].to_string()
                },
            };
            match final_hash.get_mut::<str>(&btree_key) {
                Some(albums) => albums.push(album.clone()),
                None => {
                    annual_album_group.push(album.clone());
                    final_hash.insert(btree_key.to_string(), annual_album_group.clone());
                }
            };
        });
        final_hash
    }
    pub fn albums_by_type(&self, no_print: bool) -> HashMap<&'static str, Vec<SimplifiedAlbum>> {
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
                    albums.push(album.clone());
                },
                "single" => {
                    singles.push(album.clone());
                },
                "compilation" => {
                    compilations.push(album.clone());
                },
                "appears_on" => {
                    appears_on.push(album.clone());
                },
                _ => {
                    error!("Album type is not available for album: {:?}", album.name);
                },
            };
            if !no_print {
                info!("Name: {:?} | Type: {:?}", album.name, alb_type);
            }
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
    pub fn album_slice(&self, cutoff: Option<NaiveDateTime>) -> Self {
        let span = tracing::span!(Level::INFO, "ArtistXplorer.filter_albums");
        let _enter = span.enter();
        let cutoff = match cutoff {
            Some(date) => NaiveDate::from(date),
            None => {
                let now = chrono::Local::now();
                let cutoff_date = now.date_naive() - chrono::Duration::days(365);
                cutoff_date
            }
        };

        let final_vec = self.albums.clone().iter().filter_map(|album| {
            info!("{:?}", album.name);
            let release_date = match album.release_date.clone() {
                Some(date) => {
                    match NaiveDate::parse_from_str(date.as_str(), "%Y-%m-%d") {
                        Ok(dttime) => { dttime }
                        Err(e) => { panic!("Could not parse date: {:?}", e) }
                    }
                },
                None => panic!("Could not get release date for album {}", album.name)
            };
            if release_date > cutoff {
                Some(album.clone())
            } else {
                None
            }
        }).collect::<Vec<SimplifiedAlbum>>();
        let test = ArtistXplorer {
            client: self.client.clone(),
            artist_id: self.artist_id.clone(),
            artist: self.artist.clone(),
            albums: final_vec.clone(),
        };
        test
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
    pub async fn total_tracks(&self) -> usize {
        let span = tracing::span!(Level::INFO, "ArtistXplorer.full_tracks");
        let _enter = span.enter();

        let albums = self.full_albums().await;
        self.artist.name.clone();
        info!("{} albums queried for {}", albums.len(), self.artist.name.clone());
        albums.clone().iter().fold(0, |acc, album| {
            info!("Running total: {}", acc + album.tracks.total);
            acc + album.tracks.total
        }) as usize
    }
    pub async fn full_tracks(&self) -> Vec<FullTrack> {
        let span = tracing::span!(Level::INFO, "ArtistXplorer.full_tracks");
        let _enter = span.enter();

        let mut full_tracks = Vec::new();
        let limit = BatchLimits::Tracks.get_limit();
        let albums = self.full_albums().await;
        let track_ids = albums.clone().iter().flat_map(|album| {
            album.tracks.items.clone().iter().map(|track| {
                match track.id.clone() {
                    Some(id) => id,
                    None => panic!("Could not get track ID for track {}", track.name)
                }
            }).collect::<Vec<TrackId>>()
        }).collect::<Vec<TrackId>>();
        let chunked_ids = track_ids.chunks(limit);
        let loops = chunked_ids.len();
        let wait_threshold = 200;
        let count = 25;
        for (index, track_id_chunk) in track_ids.chunks(limit).enumerate() {
            let full_track = match self.client.tracks(track_id_chunk.to_vec(), Some(Self::market())).await {
                Ok(full_tracks) => {
                    let remaining = (loops - (index + 1)) * limit;
                    info!("{} tracks have been requested. {} remaining tracks", full_tracks.len(), remaining);
                    full_tracks
                }
                Err(error) => {
                    panic!("ERROR: Was not able to get album from the requested artist.\nError information: {:?}", error)
                }
            };
            full_tracks.extend(full_track);
            let mut pb = ProgressBar::new(count);
            pb.format("╢▌▌░╟");
            if track_ids.len() > wait_threshold {
                for _ in 0..count {
                    pb.inc();
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                }
            }
            pb.finish_print("Done");
        }
        full_tracks
    }
    pub async fn collaborators(&self) -> Vec<FullArtist> {
        let span = tracing::span!(Level::INFO, "ArtistXplorer.collaborations");
        let _enter = span.enter();

        let mut collaborations = Vec::new();
        let mut artists = self.albums.clone().iter().flat_map(|album| {
            album.artists.clone().iter().map(|artist| {
                match artist.id.clone() {
                    None => panic!("Could not get artist ID for artist {}", artist.name),
                    Some(id) => id
                }
            }).collect::<Vec<ArtistId>>()
        }).collect::<Vec<ArtistId>>();
        info!("Artist length: {:?}", artists.len());
        artists = Self::clean_duplicate_id_vector(artists);
        artists.retain(|artist| *artist != self.artist_id.clone());
        info!("Artist length: {:?}", artists.len());
        let limit = BatchLimits::Artists.get_limit();
        for artist_id_chunk in artists.chunks(limit) {
            let full_artists_vec = match self.client.artists(artist_id_chunk.to_vec()).await {
                Ok(full_artists) => {
                    info!("{} artists have been requested.", full_artists.len());
                    info!("{:?}", full_artists.iter().map(|artist| artist.name.clone()).collect::<Vec<String>>());
                    full_artists
                }
                Err(error) => panic!("ERROR: Was not able to get album from the requested artist.\nError information: {:?}", error)
            };
            collaborations.extend(full_artists_vec);
        }
        collaborations
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

        for album_id in self.album_ids() {
            let mut altracks = self.client.album_track(album_id.clone(), Some(Self::market()));

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
        let artist_xplr = match ArtistXplorer::new(artist_id.clone()).await {
            Ok(xplorer) => { xplorer }
            Err(err) => {
                eprintln!("Client Error: {:?}", err);
                return;
            }
        };
        println!("{:?}", artist_xplr.genres());
        assert_eq!(artist_xplr.artist_id, artist_id);
    }

    #[tokio::test]
    async fn test_album_methods() {
        let artist_id = ArtistId::from_id("7u160I5qtBYZTQMLEIJmyz").unwrap();
        let artist_xplr = match ArtistXplorer::new(artist_id.clone()).await {
            Ok(xplorer) => { xplorer }
            Err(err) => {
                eprintln!("Client Error: {:?}", err);
                return;
            }
        };
        let albums = artist_xplr.full_albums().await;
        let artists = albums[0].artists.clone();
        let main_artist_id = artists[0].clone().id.unwrap();
        assert_eq!(main_artist_id, artist_id);
    }
}
