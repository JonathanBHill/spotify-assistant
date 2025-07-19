use crate::traits::apis::Api;
use rspotify::clients::BaseClient;
use rspotify::model::{AlbumId, ArtistId, FullAlbum, FullArtist, FullPlaylist, FullTrack, PlaylistId, TrackId};
use rspotify::{scopes, ClientError};

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
pub struct FullProfiles {
    client: rspotify::AuthCodeSpotify,
}
impl Api for FullProfiles {
    fn select_scopes() -> std::collections::HashSet<std::string::String> {
        scopes!("user-library-read", "user-library-modify")
    }
}
impl FullProfiles {
    /// Asynchronously creates a new instance of `FullProfiles`.
    ///
    /// # Returns
    /// A new `FullProfiles` instance with an initialized client.
    ///
    /// The function sets up the client by calling the `set_up_client` method. It passes `false`
    /// (indicating a certain client behavior) and the result of `select_scopes` (possibly determining
    /// authorization scopes) as parameters. The resulting client is stored in the `client` field of
    /// the `FullProfiles` struct.
    ///
    /// # Example
    /// ```rust
    /// use spotify_assistant_core::actions::general::FullProfiles;
    /// async fn main() {
    ///     let full_profiles = FullProfiles::new().await;
    /// }
    /// ```
    pub async fn new() -> Self {
        FullProfiles {
            client: Self::set_up_client(false, Some(Self::select_scopes())).await,
        }
    }

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
    ///
    /// ```rust
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
            Ok(id) => { id }
            Err(err) => { panic!("Error: {:?}", err) }
        };
        match self.client.artist(artist_id).await {
            Ok(artist) => { artist }
            Err(err) => { panic!("Error: {:?}", err) }
        }
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
    ///
    /// ```ignore
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
        self.client.album(album_id, Some(Self::market())).await
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
    /// ```ignore
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
        self.client.track(track_id, Some(Self::market())).await
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
    /// ```ignore
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
    pub async fn playlist(&self, playlist_id: PlaylistId<'static>) -> Result<FullPlaylist, ClientError> {
        self.client.playlist(playlist_id, None, Some(Self::market())).await
    }
}
