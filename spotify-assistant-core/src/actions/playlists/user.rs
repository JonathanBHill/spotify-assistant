use crate::paginators::PaginatorRunner;
use crate::traits::apis::Api;
use rspotify::clients::{BaseClient, OAuthClient};
use rspotify::model::{FullPlaylist, PlaylistId, SimplifiedPlaylist};
use rspotify::{scopes, AuthCodeSpotify};
use std::collections::{HashMap, HashSet};
use tracing::{event, Level};

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
    ///     let playlists = user_playlists.get_user_playlist_ids_as_hashmap().await;
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
    pub async fn get_user_playlist_ids_as_hashmap(&self) -> HashMap<String, PlaylistId<'_>> {
        let span = tracing::span!(Level::INFO, "UserPlaylists.get_user_playlists");
        let _enter = span.enter();

        let user_playlists = self.client.current_user_playlists();
        let mut playlists = HashMap::new();
        let paginator = PaginatorRunner::new(user_playlists, ());
        match paginator.run().await {
            Ok(vec) => {
                vec.into_iter().for_each(|playlist| {
                    playlists.insert(playlist.name, playlist.id);
                });
            },
            Err(err) => {
                event!(Level::ERROR, "Error retrieving playlists: {:?}", err);
                return HashMap::new();
            }
        };
        playlists
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
    ///     let playlists = user_playlists.get_user_playlists().await;
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
    pub async fn get_user_playlists(&self) -> Vec<SimplifiedPlaylist> {
        let span = tracing::span!(Level::INFO, "UserData.playlists");
        let _enter = span.enter();
        let user_playlists = self.client.current_user_playlists();
        let paginator = PaginatorRunner::new(user_playlists, ());
        paginator.run().await.unwrap_or_else(|err| {
            event!(Level::ERROR, "Error retrieving playlists: {:?}", err);
            Vec::new()
        })
    }
}
