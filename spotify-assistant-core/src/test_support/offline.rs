//! Shared offline Spotify test fixtures and helpers used by multiple test modules.
//!
//! This module centralizes construction of rspotify model objects and a dummy
//! AuthCodeSpotify client so individual tests don’t need to duplicate builders.
//! All functions here are deterministic and avoid any network or environment IO.

#![allow(dead_code)]

use rspotify::model::{
    AlbumId, FullAlbum, FullArtist, SavedTrack, SimplifiedAlbum, SimplifiedArtist,
    SimplifiedTrack,
};
use rspotify::prelude::Id;
use rspotify::{AuthCodeSpotify, Config, Credentials, OAuth};
use serde_json::json;

/// A namespace type containing only associated helper functions.
///
/// Usage example:
/// let client = OfflineObjects::dummy_client();
/// let (album_id, full_album) = OfflineObjects::sample_full_album();
pub struct OfflineObjects;

impl OfflineObjects {
    /// Returns a Spotify client that never performs network I/O.
    pub fn dummy_client() -> AuthCodeSpotify {
        let creds = Credentials::new("test_id", "test_secret");
        let oauth = OAuth { scopes: Default::default(), ..Default::default() };
        let config = Config::default();
        AuthCodeSpotify::with_config(creds, oauth, config)
    }

    pub fn artist_simple(id: &str, name: &str) -> SimplifiedArtist {
        serde_json::from_value(json!({
            "external_urls": {"spotify": "https://example.com/artist"},
            "href": format!("https://api.spotify.com/v1/artists/{id}"),
            "id": id,
            "name": name,
        }))
            .expect("valid SimplifiedArtist JSON")
    }

    pub fn artist_full(id: &str, name: &str) -> FullArtist {
        serde_json::from_value(json!({
            "external_urls": {"spotify": "https://example.com/artist"},
            "followers": {"href": null, "total": 12345},
            "genres": ["Rock", "Alt"],
            "href": format!("https://api.spotify.com/v1/artists/{id}"),
            "id": id,
            "images": [],
            "name": name,
            "popularity": 60,
            "type": "artist",
            "uri": format!("spotify:artist:{id}"),
        }))
            .expect("valid FullArtist JSON")
    }

    pub fn album_simplified(
        id: &str,
        name: &str,
        album_type: &str,
        release_date: &str,
        artist_id: &str,
        artist_name: &str,
    ) -> SimplifiedAlbum {
        serde_json::from_value(json!({
            "album_group": null,
            "album_type": album_type,
            "artists": [{
                "external_urls": {"spotify": "https://example.com/artist"},
                "href": format!("https://api.spotify.com/v1/artists/{artist_id}"),
                "id": artist_id,
                "name": artist_name,
            }],
            "available_markets": [],
            "external_urls": {"spotify": "https://example.com/album"},
            "href": format!("https://api.spotify.com/v1/albums/{id}"),
            "id": id,
            "images": [],
            "name": name,
            "release_date": release_date,
            "release_date_precision": "day",
            "restrictions": null,
            "total_tracks": 2,
        }))
            .expect("valid SimplifiedAlbum JSON")
    }

    pub fn track_simplified(id: &str, name: &str, artist_id: &str, artist_name: &str) -> SimplifiedTrack {
        serde_json::from_value(json!({
            "artists": [{
                "external_urls": {"spotify": "https://example.com/artist"},
                "href": format!("https://api.spotify.com/v1/artists/{artist_id}"),
                "id": artist_id,
                "name": artist_name,
            }],
            "available_markets": [],
            "disc_number": 1,
            "duration_ms": 120000,
            "explicit": false,
            "external_urls": {"spotify": "https://example.com/track"},
            "href": format!("https://api.spotify.com/v1/tracks/{id}"),
            "id": id,
            "is_local": false,
            "is_playable": true,
            "linked_from": null,
            "name": name,
            "preview_url": null,
            "track_number": 1,
        }))
            .expect("valid SimplifiedTrack JSON")
    }

    pub fn full_album_with(
        album_id: &AlbumId<'static>,
        name: &str,
        genres: &[&str],
        artist: SimplifiedArtist,
        tracks: Vec<SimplifiedTrack>,
    ) -> FullAlbum {
        let tracks_href = format!(
            "https://api.spotify.com/v1/albums/{}/tracks",
            album_id.id()
        );
        serde_json::from_value(json!({
            "album_type": "album",
            "total_tracks": tracks.len(),
            "available_markets": [],
            "external_urls": {"spotify": "https://example.com/album"},
            "href": format!("https://api.spotify.com/v1/albums/{}", album_id.id()),
            "id": album_id.id(),
            "images": [],
            "name": name,
            "release_date": "2024-01-01",
            "release_date_precision": "day",
            "restrictions": null,
            "type": "album",
            "artists": [artist],
            "genres": genres,
            "label": "Example Label",
            "popularity": 50,
            "tracks": {
                "href": tracks_href,
                "limit": 50,
                "next": null,
                "offset": 0,
                "previous": null,
                "total": tracks.len(),
                "items": tracks,
            },
            "copyrights": [
                {"text": "℗ 2024 Example Label", "type": "C"}
            ],
            "external_ids": {"isrc": "USS1Z2400001"}
        }))
            .expect("valid FullAlbum JSON")
    }

    /// Convenience sample full album used by multiple tests.
    pub fn sample_full_album() -> (AlbumId<'static>, FullAlbum) {
        let album_id = AlbumId::from_id("ABCDEFGHIJKLMNOPQRSTUVWXYZ12").unwrap();
        let artist = Self::artist_simple("ARTIST1234567890123456", "Example Artist");
        let tracks = vec![
            Self::track_simplified("TRACKID123456789012345678", "Track One", "ARTIST1234567890123456", "Example Artist"),
            Self::track_simplified("TRACKID223456789012345678", "Track Two", "ARTIST1234567890123456", "Example Artist"),
        ];
        let full_album = Self::full_album_with(&album_id, "Example Album", &["Electro", "Breaks"], artist, tracks);
        (album_id, full_album)
    }

    pub fn sample_full_artist() -> FullArtist {
        Self::artist_full("ARTIST1234567890123456", "Example Artist")
    }

    pub fn sample_simplified_album_for(artist_id: &str, artist_name: &str) -> Vec<SimplifiedAlbum> {
        vec![
            Self::album_simplified(
                "ALBUMAAAAAAAAAAAAAAAAAA",
                "Example Album",
                "album",
                "2024-05-01",
                artist_id,
                artist_name,
            ),
            Self::album_simplified(
                "ALBUMBbbbbbbbbbbbbbbbbb",
                "Old Single",
                "single",
                "2022-03-10",
                artist_id,
                artist_name,
            ),
        ]
    }

    pub fn sample_saved_track(label: &str) -> SavedTrack {
        let (track_id, artist_id, album_id) = match label {
            "one" => ("AAAAAAAAAAAAAAAAAAAAAA", "BBBBBBBBBBBBBBBBBBBBBB", "CCCCCCCCCCCCCCCCCCCCCC"),
            "two" => ("DDDDDDDDDDDDDDDDDDDDDD", "EEEEEEEEEEEEEEEEEEEEEE", "FFFFFFFFFFFFFFFFFFFFFF"),
            _ => ("GGGGGGGGGGGGGGGGGGGGGG", "HHHHHHHHHHHHHHHHHHHHHH", "IIIIIIIIIIIIIIIIIIIIII"),
        };
        let artist_href = format!("https://api.spotify.com/v1/artists/{artist_id}");
        let album_href = format!("https://api.spotify.com/v1/albums/{album_id}");
        let track_href = format!("https://api.spotify.com/v1/tracks/{track_id}");
        let track_name = format!("Example Track {label}");
        serde_json::from_value(json!({
            "added_at": "2024-01-01T00:00:00Z",
            "track": {
                "album": {
                    "album_group": null,
                    "album_type": "album",
                    "artists": [{
                        "external_urls": {"spotify": "https://example.com/artist"},
                        "href": artist_href,
                        "id": artist_id,
                        "name": "Example Artist"
                    }],
                    "available_markets": [],
                    "external_urls": {"spotify": "https://example.com/album"},
                    "href": album_href,
                    "id": album_id,
                    "images": [],
                    "name": "Example Album",
                    "release_date": "2024-01-01",
                    "release_date_precision": "day",
                    "restrictions": null
                },
                "artists": [{
                    "external_urls": {"spotify": "https://example.com/artist"},
                    "href": artist_href,
                    "id": artist_id,
                    "name": "Example Artist"
                }],
                "available_markets": [],
                "disc_number": 1,
                "duration_ms": 180000,
                "explicit": false,
                "external_ids": {"isrc": "USS1Z2400001"},
                "external_urls": {"spotify": "https://example.com/track"},
                "href": track_href,
                "id": track_id,
                "is_local": false,
                "is_playable": true,
                "linked_from": null,
                "restrictions": null,
                "name": track_name,
                "popularity": 42,
                "preview_url": null,
                "track_number": 1,
                "type": "track"
            }
        }))
            .expect("valid saved track JSON")
    }

    pub fn sample_saved_tracks() -> Vec<SavedTrack> {
        vec![Self::sample_saved_track("one"), Self::sample_saved_track("two")]
    }
}
