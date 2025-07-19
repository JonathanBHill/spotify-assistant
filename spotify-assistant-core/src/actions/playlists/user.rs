use futures::StreamExt;
use rspotify::clients::{BaseClient, OAuthClient};
use rspotify::model::{FullPlaylist, PlaylistId, SavedTrack, SimplifiedPlaylist};
use rspotify::{scopes, AuthCodeSpotify};
use std::collections::{HashMap, HashSet};
use tracing::{event, info, Level};

use crate::traits::apis::Api;

/// Represents a collection of liked songs for a user.
///
/// This struct is used to manage and interact with the "Liked Songs" playlist
/// in a Spotify user's account. It provides the necessary client and metadata
/// to access the user's liked songs.
///
/// # Fields
/// - `client`:
///   The `AuthCodeSpotify` instance used for authenticated interactions with
///   the Spotify Web API.
/// - `total_tracks`:
///   The total number of tracks in the user's "Liked Songs" playlist.
pub struct LikedSongs {
    client: AuthCodeSpotify,
    total_tracks: u32,
}

impl Api for LikedSongs {
    fn select_scopes() -> HashSet<String> {
        scopes!("user-library-read", "user-library-modify")
    }
}
impl LikedSongs {
    /// Initializes a new instance of the `LikedSongs` struct asynchronously.
    ///
    /// This method performs the following steps:
    /// 1. Creates a tracing span for logging purposes with the level set to `INFO` and name `"LikedSongs.new"`.
    /// 2. Sets up an HTTP client for interacting with the API by calling `set_up_client`,
    ///    passing `false` for token refreshment and scopes returned by `select_scopes`.
    /// 3. Uses the client to fetch the total number of saved tracks for the current user by
    ///    making a call to `current_user_saved_tracks_manual`. This fetch is done using the current market
    ///    (retrieved from `Self::market()`), and without any additional parameters for offset or limit.
    ///    - If the call is successful, the total number of saved tracks is stored.
    ///    - If the call results in an error, the error is logged, and the function will panic with an appropriate
    ///      error message.
    /// 4. Returns a `LikedSongs` instance initialized with the created client and retrieved total track count.
    ///
    /// # Returns
    /// A `LikedSongs` struct.
    ///
    /// # Panics
    /// This function will panic if there is an error while retrieving the user's saved tracks.
    ///
    /// # Examples
    /// ```ignore
    /// let liked_songs = LikedSongs::new().await;
    /// ```
    pub async fn new() -> Self {
        let span = tracing::span!(Level::INFO, "LikedSongs.new");
        let _enter = span.enter();
        let client = Self::set_up_client(false, Some(Self::select_scopes())).await;
        let total_tracks = match client.current_user_saved_tracks_manual(
            Some(Self::market()),
            None,
            None
        ).await {
            Ok(tracks) => { tracks.total }
            Err(err) => {
                event!(Level::ERROR, "Error: {:?}", err);
                panic!("Could not retrieve saved tracks.");
            }
        };
        LikedSongs {
            client,
            total_tracks,
        }
    }

    /// Returns a clone of the `AuthCodeSpotify` client.
    ///
    /// This function provides access to the `AuthCodeSpotify` client instance, allowing
    /// interaction with the Spotify API. The instance is cloned to ensure the original
    /// remains unaltered.
    ///
    /// # Returns
    ///
    /// A cloned instance of `AuthCodeSpotify`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use spotify_assistant_core::actions::playlists::user::LikedSongs;
    /// async fn main() {
    ///     let liked_songs = LikedSongs::new().await;
    ///     let spotify_client = liked_songs.client();
    ///     // Use `spotify_client` to perform operations with the Spotify API.
    /// }
    /// ```
    pub fn client(&self) -> AuthCodeSpotify {
        self.client.clone()
    }

    /// Retrieves the user's saved (liked) tracks from the library.
    ///
    /// This asynchronous function communicates with the Spotify API to fetch the current user's
    /// saved tracks. It attempts to load all tracks while handling potential errors and retries. The
    /// fetched tracks are collected into a vector, which is returned to the caller.
    ///
    /// # Returns
    /// A `Vec<SavedTrack>` containing the user's saved tracks, where each track is represented as
    /// a `SavedTrack` struct.
    ///
    /// # Functionality
    /// - Creates a tracing span for logging and diagnostic purposes.
    /// - Fetches the current user's saved tracks, paginated via the Spotify API.
    /// - Handles errors by retrying up to three times before exhausting further retrieval attempts.
    /// - Logs detailed information about each saved track and any errors encountered.
    ///
    /// # Behavior
    /// - If a saved track is successfully retrieved, it is pushed into the `saved_tracks` vector.
    /// - If an error occurs while fetching a page of saved tracks, it is logged, and the function retries until
    ///   the retry limit is reached or the process ends.
    ///
    /// # Example Usage
    /// ```rust
    /// use spotify_assistant_core::actions::playlists::user::LikedSongs;
    /// async fn main() {
    ///     let liked_songs = LikedSongs::new().await;
    ///     let saved_tracks = liked_songs.library().await;
    ///     for track in saved_tracks {
    ///         println!("Liked track: {}", track.track.name);
    ///     }
    /// }
    /// ```
    ///
    /// # Note
    /// - Uses tracing events for detailed output of the function's processing steps.
    /// - The maximum retries count is hardcoded to `3`.
    /// - The `Self::market()` method is used to specify the market parameter if applicable, which should
    ///   return an appropriate string.
    ///
    /// # Errors
    /// - Errors encountered during the fetching process are logged, but after three retry attempts,
    ///   the function ceases further error handling. Depending on the API behavior or network state, this
    ///   could result in an incomplete list being returned.
    ///
    /// # Dependencies
    /// - This function assumes the existence of a `client` with a `current_user_saved_tracks` method that
    ///   supports asynchronous iteration.
    /// - Requires importing the `tracing` crate for logging and diagnostics.
    pub async fn library(&self) -> Vec<SavedTrack> {
        let span = tracing::span!(Level::INFO, "LikedSongs.library");
        let _enter = span.enter();

        let mut liked_songs = self.client.current_user_saved_tracks(Some(Self::market()));
        let mut saved_tracks: Vec<SavedTrack> = Vec::new();
        let mut retries = 3;
        while retries > 0 {
            if let Some(page) = liked_songs.next().await {
                match page {
                    Ok(saved_track) => {
                        event!(Level::INFO, "Saved track: {:?} | New vector length: {:?}", saved_track.track.name, saved_tracks.len() + 1);
                        saved_tracks.push(saved_track);
                    },
                    Err(err) => {
                        event!(Level::ERROR, "Error: {:?}", err);
                        retries -= 1;
                    }
                }
            } else {
                break;
            }
        }
        saved_tracks
    }

}

/// The `UserPlaylists` struct is a representation of user playlists
/// within the Spotify API integration. It enables interaction with
/// a user's playlists utilizing an authenticated Spotify client.
///
/// # Fields
///
/// * `client` - An instance of `AuthCodeSpotify` that provides the
///   necessary authenticated client to communicate with the Spotify API.
///
/// # Derives
///
/// * `Clone` - Allows for the `UserPlaylists` struct to be cloned,
///   creating a duplicate instance with the same underlying data.
///
/// # Usage
///
/// The `UserPlaylists` struct is designed to act as a bridge for
/// managing and accessing a user's playlists through the Spotify web
/// API. To use this struct, ensure that an authenticated Spotify client
/// (`AuthCodeSpotify`) is initialized and passed in when creating an instance.
#[derive(Clone)]
pub struct UserPlaylists {
    client: AuthCodeSpotify,
}

impl Api for UserPlaylists {
    fn select_scopes() -> HashSet<String> {
        scopes!("playlist-read-private", "playlist-read-collaborative", "user-library-read")
    }
}

impl UserPlaylists {
    /// Creates a new instance of `UserPlaylists`.
    ///
    /// This asynchronous function sets up a `UserPlaylists` instance by:
    /// - Creating a new tracing span with an INFO log level labeled as "UserPlaylists.new".
    /// - Entering the span for scoped logging.
    /// - Setting up an HTTP client with a specified configuration by calling `set_up_client()`.
    ///
    /// # Returns
    /// A fully initialized `UserPlaylists` instance with an associated HTTP client.
    ///
    /// # Implementation Details
    /// - `set_up_client` is configured to not enforce strict client behavior (via the `false` parameter)
    ///   and selects specific scopes required for the operation by calling `Self::select_scopes()`.
    /// - Tracing is utilized for logging purposes to help with debugging and monitoring.
    ///
    /// # Example
    /// ```rust
    /// use spotify_assistant_core::actions::playlists::user::UserPlaylists;
    /// async fn main() {
    ///     let playlists = UserPlaylists::new().await;
    /// }
    /// ```
    ///
    /// # Panics
    /// This function does not handle any panics internally. Any panic from `set_up_client` must
    /// be handled by the caller.
    pub async fn new() -> Self {
        let span = tracing::span!(Level::INFO, "UserPlaylists.new");
        let _enter = span.enter();

        let client = Self::set_up_client(false, Some(Self::select_scopes())).await;
        UserPlaylists { client }
    }

    /// Asynchronously fetches a specific playlist identified by a fixed playlist ID
    /// and returns it as a `FullPlaylist` object.
    ///
    /// # Details
    ///
    /// This function retrieves the playlist using the hardcoded playlist ID "37i9dQZEVXbdINACbjb1qu".
    /// It uses the client associated with the instance to fetch the playlist data via an asynchronous call.
    /// The market context is specified using the `Self::market()` method.
    /// The function assumes that the client can return a valid playlist and panics if the retrieval fails.
    ///
    /// ## Tracing
    /// A tracing span named "UserPlaylists.stockrr" with an `INFO` level is created and entered to support
    /// diagnostic information during the execution of this function.
    ///
    /// # Returns
    ///
    /// Returns a `FullPlaylist` instance representing the playlist associated with the hardcoded playlist ID.
    ///
    /// # Panics
    ///
    /// This function will panic if:
    /// - The playlist retrieval fails for any reason (e.g., network issues or invalid credentials).
    ///
    /// # Example
    ///
    /// ```rust
    /// use spotify_assistant_core::actions::playlists::user::UserPlaylists;
    /// async fn main() {
    ///     let playlists = UserPlaylists::new().await;
    ///     let playlist = playlists.stockrr().await;
    ///     println!("Received playlist: {}", playlist.name);
    /// }
    /// ```
    pub async fn stockrr(&self) -> FullPlaylist {
        let span = tracing::span!(Level::INFO, "UserPlaylists.stockrr");
        let _enter = span.enter();
        let rr_id = PlaylistId::from_id("37i9dQZEVXbdINACbjb1qu").unwrap();
        let rr_pl = self
            .client
            .playlist(rr_id.clone(), None, Some(Self::market()))
            .await
            .expect("Could not retrieve playlists");
        rr_pl
    }

    /// Asynchronously retrieves and returns the "Custom Release Radar" playlist for the user.
    ///
    /// # Description
    /// This function fetches a predefined playlist using a hard-coded playlist ID. The playlist ID
    /// corresponds to the "Custom Release Radar" for the user. The function utilizes the `self.client`
    /// to perform the playlist retrieval operation. If the playlist ID is invalid or if the playlist
    /// retrieval fails, the function will log the respective error and terminate with a panic.
    ///
    /// # Behavior
    /// - A tracing span is created with the name `UserPlaylists.custom_release_radar` for logging purposes.
    /// - The function attempts to parse the playlist ID ("46mIugmIiN2HYVwAwlaBAr") to a `PlaylistId`.
    ///   - If parsing fails, an error event is logged, and the function panics.
    /// - The function then requests the playlist data from the client.
    ///   - If the retrieval is successful, the playlist is returned.
    ///   - If the retrieval fails, an error event is logged, and the function panics.
    ///
    /// # Returns
    /// Returns a `FullPlaylist` object, which contains details of the "Custom Release Radar" playlist.
    ///
    /// # Panics
    /// - If the static playlist ID is invalid and cannot be converted into a `PlaylistId`.
    /// - If the playlist retrieval operation fails.
    ///
    /// # Logging
    /// - Logs an informational span indicating the function's execution flow.
    /// - Logs error events and their details if playlist ID parsing or playlist retrieval fails.
    ///
    /// # Examples
    /// ```no_run
    /// use spotify_assistant_core::actions::playlists::user::UserPlaylists;
    /// async fn main() {
    ///     let playlists = UserPlaylists::new().await;
    ///     let custom_release_radar = playlists.custom_release_radar().await;
    ///     println!("Custom Release Radar: {:?}", custom_release_radar);
    /// }
    /// ```
    ///
    /// # Dependencies
    /// - `tracing`: Used to log spans and events for debugging or monitoring.
    /// - `PlaylistId`: Used to parse and validate the playlist ID.
    /// - `self.client`: An instance of the client used to request playlists.
    /// - `FullPlaylist`: Represents the response format for a playlist.
    ///
    /// # Note
    /// Ensure that the client instance (`self.client`) is properly configured and authenticated
    /// to access the playlist data.
    pub async fn custom_release_radar(&self) -> FullPlaylist {
        let span = tracing::span!(Level::INFO, "UserPlaylists.custom_release_radar");
        let _enter = span.enter();
        let rr_id = match PlaylistId::from_id("46mIugmIiN2HYVwAwlaBAr") {
            Ok(id) => id,
            Err(err) => {
                event!(Level::ERROR, "Error: {:?}", err);
                panic!("Could not retrieve playlist");
            }
        };
        match self.client.playlist(rr_id.clone(), None, Some(Self::market())).await {
            Ok(release_radar_playlist) => release_radar_playlist,
            Err(err) => {
                event!(Level::ERROR, "Error: {:?}", err);
                panic!("Could not retrieve playlists");
            }
        }
    }

    /// Retrieves the playlists of the current user.
    ///
    /// This asynchronous function fetches the user's playlists using the Spotify client and
    /// returns a `HashMap` where the keys are playlist names (as `String`) and the values
    /// are their corresponding `PlaylistId`. The function utilizes retry logic in case of
    /// errors while fetching playlists, with a maximum of 3 attempts.
    ///
    /// # Returns
    ///
    /// A `HashMap<String, PlaylistId>` containing the playlist names as keys and their
    /// respective IDs as values. If all retry attempts fail, an empty `HashMap` is returned.
    ///
    /// # Behavior
    ///
    /// - The function enters a tracing span with level `INFO` for better observability.
    /// - If an error occurs during playlist retrieval, it logs the issue at `ERROR` level
    ///   and decrements the retry counter.
    /// - If the maximum number of retries is reached, it logs a final failure message at
    ///   `ERROR` level.
    /// - Successfully retrieved playlists are added to the `HashMap` which is returned.
    ///
    /// # Dependencies
    ///
    /// This function assumes the existence of a `client` within the implemented structure,
    /// which provides access to current user playlists via `current_user_playlists`.
    ///
    /// # Example Usage
    ///
    /// ```rust
    /// use spotify_assistant_core::actions::playlists::user::UserPlaylists;
    /// async fn main() {
    ///     let user_playlists = UserPlaylists::new().await;
    ///     let playlists = user_playlists.get_user_playlists().await;
    ///     for (name, id) in playlists {
    ///         println!("Playlist Name: {}, ID: {:?}", name, id);
    ///     }
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// - Logs an error if playlist retrieval fails for any reason.
    /// - If all retry attempts fail, an empty map is returned without propagating the specific error.
    pub async fn get_user_playlists(&self) -> HashMap<String, PlaylistId> {
        let span = tracing::span!(Level::INFO, "UserPlaylists.get_user_playlists");
        let _enter = span.enter();

        let mut user_playlists = self
            .client
            .current_user_playlists();
        let mut playlists = HashMap::new();
        let mut retries = 3;
        while retries > 0 {
            if let Some(pl) = user_playlists.next().await {
                match pl {
                    Ok(simp) => {
                        playlists.insert(simp.name, simp.id);
                    },
                    Err(err) => {
                        event!(Level::ERROR, "Error retrieving playlist: {:?}", err);
                        retries -= 1;
                    }
                }
            } else {
                break;
            }
        }
        if retries == 0 {
            event!(Level::ERROR, "Failed to retrieve playlists after multiple attempts.");
        }
        playlists.clone()
    }

    /// Fetches all playlists of the current user asynchronously.
    ///
    /// This function retrieves all playlists associated with the current user using
    /// paginated API calls to handle a potentially large amount of data. It fetches
    /// playlists in chunks (pages) with a maximum size of 50 items per page, iterating
    /// through all available pages until all playlists are retrieved.
    ///
    /// # Returns
    ///
    /// A `Vec<SimplifiedPlaylist>` containing all the playlists of the user.
    ///
    /// # Panics
    ///
    /// This function will panic if:
    /// - The API request fails during any of the user playlists retrieval.
    /// - An unexpected error occurs during data fetching.
    ///
    /// # Implementation Details
    ///
    /// - A span with a tracing log at the `INFO` level is used to help with debugging
    ///   and logging the lifecycle of this function.
    /// - The initial total number of playlists is fetched to determine the number of
    ///   pages required.
    /// - Playlists are fetched one page at a time, with an offset calculated to get
    ///   the correct subset of data.
    /// - Each playlist is inserted into a vector (`pl_vec`) based on its index computed
    ///   from the page and its relative position within that page.
    /// - For debugging, each page's progress and the complete list of fetched playlists
    ///   are logged.
    ///
    /// # Example
    ///
    /// ```rust
    /// use spotify_assistant_core::actions::playlists::user::UserPlaylists;
    /// async fn main() {
    ///     let user_playlists = UserPlaylists::new().await;
    ///     let playlists = user_playlists.get_user_playlists_old().await;
    ///     for playlist in playlists {
    ///         println!("Playlist: {} | Public: {}", playlist.name, playlist.public.unwrap_or(false));
    ///     }
    /// }
    /// ```
    ///
    /// # Logging Output
    ///
    /// Detailed logs include:
    /// - The total number of pages to be fetched (`pages with remainder` or `pages w/o remainder`).
    /// - Success information for each page as it's appended, e.g., `Page {current}/{total} appended`.
    /// - Details for each fetched playlist, including its name and public visibility status.
    /// - The total number of user playlists retrieved at the end.
    ///
    /// # Notes
    ///
    /// - This function uses the `current_user_playlists_manual` method to make paginated requests
    ///   to the API, assuming this method supports query parameters for page size and offset.
    /// - The function clones some data for processing (`pl_vec.clone()`), which might have performance
    ///   implications for very large datasets.
    pub async fn get_user_playlists_old(&self) -> Vec<SimplifiedPlaylist> {
        let span = tracing::span!(Level::INFO, "UserData.playlists");
        let _enter = span.enter();
        let playlists = match self.client.current_user_playlists_manual(Some(50), None).await {
            Ok(playlists) => playlists,
            Err(error) => panic!("Could not get playlists: {}", error),
        };

        let page_size = 50;
        let total_pl = playlists.total;
        let mut pl_vec = Vec::with_capacity(total_pl as usize);
        let pages_no_remainder = (total_pl / page_size) as i32;
        let pages = if total_pl % page_size > 0 {
            info!("pages with remainder: {}", pages_no_remainder + 1);
            pages_no_remainder + 1
        } else {
            info!("pages w/o remainder: {pages_no_remainder}");
            pages_no_remainder
        };

        for page in 0..pages {
            let offset = page_size * page as u32;
            let multiplier = page_size as usize * page as usize;
            let offset_playlists = match self
                .client
                .current_user_playlists_manual(Some(page_size), Some(offset))
                .await
            {
                Ok(page) => page.items.into_iter(),
                Err(error) => panic!("{:?}", error),
            };
            for (index, playlist) in offset_playlists.enumerate() {
                let playlist_number = index + multiplier;
                pl_vec.insert(playlist_number, playlist);
            }
            info!("Page {}/{} appended", page + 1, pages)
        }
        pl_vec
            .clone()
            .iter()
            .enumerate()
            .for_each(|(index, playlist)| {
                info!(
                    "{index}: Name: {:?} | Public: {:?}",
                    playlist.name, playlist.public
                );
            });
        info!("Total playlists: {}", playlists.total);
        pl_vec
    }
}
