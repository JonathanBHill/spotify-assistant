use crate::traits::apis::Api;
use rspotify::clients::BaseClient;
use rspotify::model::{AlbumId, ArtistId, FullAlbum, FullArtist, FullPlaylist, FullTrack, Market, PlaylistId, TrackId};
use rspotify::{scopes, AuthCodeSpotify, ClientError};
use std::pin::Pin;

/// A structure representing full profiles with Spotify authorization.
///
/// This struct encapsulates a Spotify client using the `AuthCodeSpotify` type
/// from the `rspotify` crate. It is used to manage and interact with user profiles
/// that require authorized access to the Spotify Web API.
///
/// # Fields
/// - `client`: An instance of `rspotify::AuthCodeSpotify` that handles
///   authenticated requests to the Spotify Web API.
///
/// # Notes
/// Ensure the `client` field is initialized with proper credentials and
/// authorization to successfully access and manage Spotify profiles.
#[derive(Debug)]
pub struct FullProfiles<C = AuthCodeSpotify> {
    client: C,
}
impl Api for FullProfiles<AuthCodeSpotify> {
    fn select_scopes() -> std::collections::HashSet<std::string::String> {
        scopes!("user-library-read", "user-library-modify")
    }
}
pub trait FullProfilesClient {
    fn artist<'a>(
        &'a self,
        artist_id: ArtistId<'a>,
    ) -> Pin<Box<dyn Future<Output = Result<FullArtist, ClientError>> + Send + 'a>>;

    fn album<'a>(
        &'a self,
        album_id: AlbumId<'a>,
        market: Option<Market>,
    ) -> Pin<Box<dyn Future<Output = Result<FullAlbum, ClientError>> + Send + 'a>>;

    fn track<'a>(
        &'a self,
        track_id: TrackId<'a>,
        market: Option<Market>,
    ) -> Pin<Box<dyn Future<Output = Result<FullTrack, ClientError>> + Send + 'a>>;

    fn playlist<'a>(
        &'a self,
        playlist_id: PlaylistId<'a>,
        fields: Option<&'a str>,
        market: Option<Market>,
    ) -> Pin<Box<dyn Future<Output = Result<FullPlaylist, ClientError>> + Send + 'a>>;
}

impl FullProfilesClient for AuthCodeSpotify {
    fn artist<'a>(
        &'a self,
        artist_id: ArtistId<'a>,
    ) -> Pin<Box<dyn Future<Output = Result<FullArtist, ClientError>> + Send + 'a>> {
        Box::pin(BaseClient::artist(self, artist_id))
    }

    fn album<'a>(
        &'a self,
        album_id: AlbumId<'a>,
        market: Option<Market>,
    ) -> Pin<Box<dyn Future<Output = Result<FullAlbum, ClientError>> + Send + 'a>> {
        Box::pin(BaseClient::album(self, album_id, market))
    }

    fn track<'a>(
        &'a self,
        track_id: TrackId<'a>,
        market: Option<Market>,
    ) -> Pin<Box<dyn Future<Output = Result<FullTrack, ClientError>> + Send + 'a>> {
        Box::pin(BaseClient::track(self, track_id, market))
    }

    fn playlist<'a>(
        &'a self,
        playlist_id: PlaylistId<'a>,
        fields: Option<&'a str>,
        market: Option<Market>,
    ) -> Pin<Box<dyn Future<Output = Result<FullPlaylist, ClientError>> + Send + 'a>> {
        Box::pin(BaseClient::playlist(self, playlist_id, fields, market))
    }
}

impl<C> FullProfiles<C> {
    pub fn with_client(client: C) -> Self {
        Self { client }
    }
}
impl<C: FullProfilesClient> FullProfiles<C> {

    /// Fetches detailed information about an artist by their Spotify ID.
    ///
    /// This function takes an artist's Spotify ID as a string, validates
    /// and converts it into an `ArtistId` type, and then performs an
    /// asynchronous request to retrieve the artist's full details.
    ///
    /// # Arguments
    ///
    /// - `artist_id`: A `String` representing the Spotify ID of the artist
    ///   whose information is to be fetched.
    ///
    /// # Returns
    ///
    /// - Returns a `FullArtist` object containing detailed information about
    ///   the artist, such as their name, genres, popularity, and more.
    ///
    /// # Panics
    ///
    /// - If the given `artist_id` is invalid and cannot be converted to
    ///   an `ArtistId`, the function will panic with an error message.
    /// - If the request to fetch the artist's details fails (e.g., due
    ///   to network issues or API errors), the function will panic with
    ///   the corresponding error message.
    ///
    /// # Example
    /// ```no_run,ignore
    /// use spotify_assistant_core::actions::general::FullProfiles;
    /// async fn main() {
    ///     let full_profiles = FullProfiles::new().await;
    ///     let artist_details = full_profiles.artist("3TVXtAsR1Inumwj472S9r4".to_string()).await;
    ///     println!("Artist name: {}", artist_details.name);
    /// }
    /// ```
    ///
    /// # Note
    ///
    /// Ensure to handle panics appropriately or consider refactoring this
    /// function to return a `Result` type for better error handling.
    pub async fn artist(&self, artist_id: String) -> FullArtist {
        let artist_id = match ArtistId::from_id(artist_id) {
            Ok(id) => id,
            Err(err) => {
                panic!("Error: {:?}", err)
            }
        };
        self.client.artist(artist_id).await.unwrap_or_else(|err| {
            panic!("Error: {:?}", err)
        })
    }

    /// Retrieves detailed information about a specific album by its unique identifier.
    ///
    /// This asynchronous function queries the Spotify Web API to fetch all available details
    /// about the album, such as its name, artists, tracks, release date, and more.
    ///
    /// # Arguments
    ///
    /// * `album_id` - A unique identifier for the album (`AlbumId<'static>`). This must be
    ///                a valid Spotify album ID.
    ///
    /// # Returns
    ///
    /// * `Result<FullAlbum, ClientError>` - If successful, returns a `FullAlbum` object
    ///   containing detailed information about the album. If an error occurs while querying
    ///   the API, returns a `ClientError`.
    ///
    /// # Example
    /// ```no_run,ignore
    /// use spotify_assistant_core::actions::general::FullProfiles;
    /// async fn main() {
    ///     let full_profiles = FullProfiles::new().await;
    ///     let album_id = AlbumId::from_static("some_album_id");
    ///     match full_profiles.album(album_id).await {
    ///         Ok(full_album) => {
    ///             println!("Album name: {}", full_album.name);
    ///         }
    ///         Err(err) => {
    ///             eprintln!("Error fetching album: {}", err);
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// # Note
    ///
    /// This method automatically applies the market context using the `Self::market()`
    /// function, which customizes the request to retrieve data specific to the market
    /// (e.g., region or country) configured for the client.
    ///
    /// # Errors
    ///
    /// This method returns a `ClientError` if:
    /// - There is a network issue during the request.
    /// - The provided `album_id` is invalid or refers to a non-existent album.
    /// - A server-side error occurs on Spotify's API.
    pub async fn album(&self, album_id: AlbumId<'static>) -> Result<FullAlbum, ClientError> {
        self.client
            .album(album_id, Some(FullProfiles::<AuthCodeSpotify>::market()))
            .await
    }

    /// Fetches detailed information about a specific track using its unique identifier.
    ///
    /// # Parameters
    /// - `track_id`: A unique identifier (`TrackId`) for the track to retrieve. This must have a
    ///   `'static` lifetime.
    ///
    /// # Returns
    /// - `Result<FullTrack, ClientError>`:
    ///   - `Ok(FullTrack)`: Contains the detailed track information if retrieved successfully.
    ///   - `Err(ClientError)`: If an error occurred during the request (e.g., network issues or
    ///     invalid `track_id`).
    ///
    /// # Notes
    /// - The function internally uses the `Client` to make an asynchronous request to fetch the track
    ///   information.
    /// - The request is scoped with a specific market, which is determined by `Self::market()`.
    ///
    /// # Example
    /// ```no_run,ignore
    /// use spotify_assistant_core::actions::general::FullProfiles;
    /// async fn main() {
    ///     use rspotify::model::TrackId;
    /// let full_profiles = FullProfiles::new().await;
    ///     let track_id: TrackId = TrackId::from("track-id");
    ///     let result = full_profiles.track(track_id).await;
    ///     match result {
    ///         Ok(track) => println!("Track fetched: {:?}", track),
    ///         Err(e) => eprintln!("Error fetching track: {:?}", e),
    ///     }
    /// }
    /// ```
    ///
    /// # Errors
    /// This function will return an error (`ClientError`) if:
    /// - The `TrackId` is invalid or not recognized.
    /// - The client fails to connect or authenticate with the API.
    /// - The requested track is not available in the specified market.
    ///
    /// # Async
    /// This function is asynchronous and requires `.await` to operate. Ensure that you are executing
    /// it within an asynchronous runtime.
    pub async fn track(&self, track_id: TrackId<'static>) -> Result<FullTrack, ClientError> {
        self.client
            .track(track_id, Some(FullProfiles::<AuthCodeSpotify>::market()))
            .await
    }

    /// Retrieves the full details of a playlist from the Spotify API.
    ///
    /// # Parameters
    /// - `playlist_id`: The unique identifier for the playlist to fetch.
    ///
    /// # Returns
    /// - If successful, returns a `Result` containing a `FullPlaylist` object, which includes the playlist's metadata, tracks, and other details.
    /// - If an error occurs, returns a `ClientError`.
    ///
    /// # Example
    /// ```no_run,ignore
    /// use rspotify::model::PlaylistId;
    /// use spotify_assistant_core::actions::general::FullProfiles;
    /// async fn main() {
    ///     let full_profiles = FullProfiles::new().await;
    ///     let playlist_id = PlaylistId::new("some_playlist_id").unwrap();
    ///     let playlist = full_profiles.playlist(playlist_id).await?;
    ///     println!("Playlist name: {}", playlist.name);
    /// }
    /// ```
    ///
    /// # Notes
    /// - The `market` parameter is used to adjust for market-specific content.
    /// - This function relies on the underlying client's `playlist` method to perform the request.
    ///
    /// # Errors
    /// Returns a `ClientError` if the API request fails or if the playlist cannot be found.
    pub async fn playlist(
        &self,
        playlist_id: PlaylistId<'static>,
    ) -> Result<FullPlaylist, ClientError> {
        self.client
            .playlist(
                playlist_id,
                None,
                Some(FullProfiles::<AuthCodeSpotify>::market()),
            )
            .await
    }
}

impl FullProfiles<AuthCodeSpotify> {
    pub async fn new() -> Self {
        Self::with_client(Self::set_up_client(false, Some(Self::select_scopes())).await)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration as ChronoDuration;
    use futures::FutureExt;
    use rspotify::model::{AlbumType, Copyright, CopyrightType, DatePrecision, Followers, Image, Page, PlaylistItem, PublicUser, SimplifiedAlbum, SimplifiedArtist, SimplifiedTrack, Type, UserId};
    use std::collections::HashMap;
    use std::pin::Pin;
    use std::sync::Mutex;

    struct MockFullProfilesClient {
        artist: Mutex<Option<Result<FullArtist, ClientError>>>,
        album: Mutex<Option<Result<FullAlbum, ClientError>>>,
        track: Mutex<Option<Result<FullTrack, ClientError>>>,
        playlist: Mutex<Option<Result<FullPlaylist, ClientError>>>,
    }

    impl MockFullProfilesClient {
        fn new(
            artist: Option<Result<FullArtist, ClientError>>,
            album: Option<Result<FullAlbum, ClientError>>,
            track: Option<Result<FullTrack, ClientError>>,
            playlist: Option<Result<FullPlaylist, ClientError>>,
        ) -> Self {
            Self {
                artist: Mutex::new(artist),
                album: Mutex::new(album),
                track: Mutex::new(track),
                playlist: Mutex::new(playlist),
            }
        }
    }

    impl FullProfilesClient for MockFullProfilesClient {
        fn artist<'a>(
            &'a self,
            _artist_id: ArtistId<'a>,
        ) -> Pin<Box<dyn Future<Output = Result<FullArtist, ClientError>> + Send + 'a>> {
            let response = self
                .artist
                .lock()
                .unwrap()
                .take()
                .expect("artist response missing");
            Box::pin(async move { response })
        }

        fn album<'a>(
            &'a self,
            _album_id: AlbumId<'a>,
            _market: Option<Market>,
        ) -> Pin<Box<dyn Future<Output = Result<FullAlbum, ClientError>> + Send + 'a>> {
            let response = self
                .album
                .lock()
                .unwrap()
                .take()
                .expect("album response missing");
            Box::pin(async move { response })
        }

        fn track<'a>(
            &'a self,
            _track_id: TrackId<'a>,
            _market: Option<Market>,
        ) -> Pin<Box<dyn Future<Output = Result<FullTrack, ClientError>> + Send + 'a>> {
            let response = self
                .track
                .lock()
                .unwrap()
                .take()
                .expect("track response missing");
            Box::pin(async move { response })
        }

        fn playlist<'a>(
            &'a self,
            _playlist_id: PlaylistId<'a>,
            _fields: Option<&'a str>,
            _market: Option<Market>,
        ) -> Pin<Box<dyn Future<Output = Result<FullPlaylist, ClientError>> + Send + 'a>> {
            let response = self
                .playlist
                .lock()
                .unwrap()
                .take()
                .expect("playlist response missing");
            Box::pin(async move { response })
        }
    }

    fn sample_simplified_artist() -> SimplifiedArtist {
        SimplifiedArtist {
            external_urls: HashMap::new(),
            href: None,
            id: Some(ArtistId::from_id("0123456789abcdef012345").unwrap()),
            name: "Test Artist".to_string(),
        }
    }

    fn sample_followers() -> Followers {
        Followers { total: 10 }
    }

    fn sample_image() -> Image {
        Image {
            height: Some(640),
            url: "https://example.com/image.jpg".to_string(),
            width: Some(640),
        }
    }

    fn sample_full_artist() -> FullArtist {
        FullArtist {
            external_urls: HashMap::new(),
            followers: sample_followers(),
            genres: vec!["pop".to_string()],
            href: "https://example.com/artist".to_string(),
            id: ArtistId::from_id("0123456789abcdef012345").unwrap(),
            images: vec![sample_image()],
            name: "Test Artist".to_string(),
            popularity: 50,
        }
    }

    fn sample_full_album() -> FullAlbum {
        FullAlbum {
            artists: vec![sample_simplified_artist()],
            album_type: AlbumType::Album,
            available_markets: Some(vec!["US".to_string()]),
            copyrights: vec![Copyright {
                text: "(c) 2024".to_string(),
                _type: CopyrightType::Copyright,
            }],
            external_ids: HashMap::new(),
            external_urls: HashMap::new(),
            genres: vec![],
            href: "https://example.com/album".to_string(),
            id: AlbumId::from_id("0123456789abcdef012345").unwrap(),
            images: vec![sample_image()],
            name: "Test Album".to_string(),
            popularity: 42,
            release_date: "2024-01-01".to_string(),
            release_date_precision: DatePrecision::Day,
            tracks: Page {
                href: "https://example.com/tracks".to_string(),
                items: vec![sample_simplified_track()],
                limit: 1,
                next: None,
                offset: 0,
                previous: None,
                total: 1,
            },
            label: Some("Test Label".to_string()),
        }
    }

    fn sample_simplified_track() -> SimplifiedTrack {
        SimplifiedTrack {
            album: Some(sample_simplified_album()),
            artists: vec![sample_simplified_artist()],
            available_markets: Some(vec!["US".to_string()]),
            disc_number: 1,
            duration: ChronoDuration::seconds(200),
            explicit: false,
            external_urls: HashMap::new(),
            href: Some("https://example.com/track".to_string()),
            id: Some(TrackId::from_id("0123456789abcdef012345").unwrap()),
            is_local: false,
            is_playable: Some(true),
            linked_from: None,
            restrictions: None,
            name: "Track".to_string(),
            preview_url: None,
            track_number: 1,
        }
    }

    fn sample_simplified_album() -> SimplifiedAlbum {
        SimplifiedAlbum {
            album_group: None,
            album_type: Some("album".to_string()),
            artists: vec![sample_simplified_artist()],
            available_markets: vec!["US".to_string()],
            external_urls: HashMap::new(),
            href: Some("https://example.com/simple_album".to_string()),
            id: Some(AlbumId::from_id("0123456789abcdef012345").unwrap()),
            images: vec![sample_image()],
            name: "Simplified Album".to_string(),
            release_date: Some("2024-01-01".to_string()),
            release_date_precision: Some("day".to_string()),
            restrictions: None,
        }
    }

    fn sample_full_track() -> FullTrack {
        FullTrack {
            album: sample_simplified_album(),
            artists: vec![sample_simplified_artist()],
            available_markets: vec!["US".to_string()],
            disc_number: 1,
            duration: ChronoDuration::seconds(200),
            explicit: false,
            external_ids: HashMap::new(),
            external_urls: HashMap::new(),
            href: Some("https://example.com/track".to_string()),
            id: Some(TrackId::from_id("0123456789abcdef012345").unwrap()),
            is_local: false,
            is_playable: Some(true),
            linked_from: None,
            restrictions: None,
            name: "Full Track".to_string(),
            popularity: 30,
            preview_url: None,
            track_number: 1,
            r#type: Type::Artist,
        }
    }

    fn sample_public_user() -> PublicUser {
        PublicUser {
            display_name: Some("Tester".to_string()),
            external_urls: HashMap::new(),
            followers: Some(sample_followers()),
            href: "https://example.com/user".to_string(),
            id: UserId::from_id("testuser").unwrap(),
            images: vec![sample_image()],
        }
    }

    fn sample_full_playlist() -> FullPlaylist {
        FullPlaylist {
            collaborative: false,
            description: Some("Sample playlist".to_string()),
            external_urls: HashMap::new(),
            followers: sample_followers(),
            href: "https://example.com/playlist".to_string(),
            id: PlaylistId::from_id("0123456789abcdef012345").unwrap(),
            images: vec![sample_image()],
            name: "Playlist".to_string(),
            owner: sample_public_user(),
            public: Some(true),
            snapshot_id: "snapshot".to_string(),
            tracks: Page {
                href: "https://example.com/playlist/tracks".to_string(),
                items: vec![PlaylistItem::default()],
                limit: 1,
                next: None,
                offset: 0,
                previous: None,
                total: 1,
            },
        }
    }

    #[tokio::test]
    async fn artist_invalid_id_panics() {
        let profiles =
            FullProfiles::with_client(MockFullProfilesClient::new(None, None, None, None));

        let result = std::panic::AssertUnwindSafe(profiles.artist("invalid".into()))
            .catch_unwind()
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn artist_returns_mock_data() {
        let expected = sample_full_artist();
        let profiles = FullProfiles::with_client(MockFullProfilesClient::new(
            Some(Ok(expected.clone())),
            None,
            None,
            None,
        ));

        let actual = profiles.artist("0123456789abcdef012345".to_string()).await;

        assert_eq!(actual, expected);
    }

    #[tokio::test]
    async fn album_returns_mock_data() {
        let expected = sample_full_album();
        let profiles = FullProfiles::with_client(MockFullProfilesClient::new(
            None,
            Some(Ok(expected.clone())),
            None,
            None,
        ));

        let actual = profiles
            .album(AlbumId::from_id("0123456789abcdef012345").unwrap())
            .await
            .unwrap();

        assert_eq!(actual, expected);
    }

    #[tokio::test]
    async fn track_returns_mock_data() {
        let expected = sample_full_track();
        let profiles = FullProfiles::with_client(MockFullProfilesClient::new(
            None,
            None,
            Some(Ok(expected.clone())),
            None,
        ));

        let actual = profiles
            .track(TrackId::from_id("0123456789abcdef012345").unwrap())
            .await
            .unwrap();

        assert_eq!(actual, expected);
    }

    #[tokio::test]
    async fn playlist_returns_mock_data() {
        let expected = sample_full_playlist();
        let profiles = FullProfiles::with_client(MockFullProfilesClient::new(
            None,
            None,
            None,
            Some(Ok(expected.clone())),
        ));

        let actual = profiles
            .playlist(PlaylistId::from_id("0123456789abcdef012345").unwrap())
            .await
            .unwrap();

        assert_eq!(actual, expected);
    }
}
