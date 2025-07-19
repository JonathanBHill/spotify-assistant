use crate::enums::fs::ProjectDirectories;
use crate::traits::apis::Api;
use rspotify::model::SavedTrack;
use rspotify::prelude::{Id, OAuthClient};
use rspotify::AuthCodeSpotify;
use std::io::Write;
use std::path::PathBuf;
use std::{fs, io};
use tracing::event;

/// Represents the structure that handles a user's liked songs.
///
/// This structure is used to manage and interact with a user's liked songs
/// by communicating with the Spotify API and handling saved track data.
///
/// # Fields
///
/// * `client` - An instance of `AuthCodeSpotify` used to authenticate and
///   communicate with the Spotify API. This client is required for executing
///   API requests related to the user's liked songs.
///
/// * `tracks` - A `Vec<SavedTrack>` containing the user's liked songs. Each
///   `SavedTrack` represents metadata about an individual song that the user
///   has liked or saved.
///
/// * `saved_tracks_path` - A `PathBuf` representing the file system path
///   where saved tracks data will be stored or retrieved. This can be used
///   to persist the user's liked song data locally for offline access or
///   caching purposes.
pub struct UserLikedSongs {
    client: AuthCodeSpotify,
    tracks: Vec<SavedTrack>,
    saved_tracks_path: PathBuf,
}
impl Api for UserLikedSongs {
    fn select_scopes() -> std::collections::HashSet<String> {
        rspotify::scopes!(
            "user-library-read"
        )
    }
}
impl UserLikedSongs {
    /// Asynchronously constructs a new instance of the struct.
    ///
    /// This function performs the following operations:
    /// 1. Creates a client by calling `set_up_client`, with predefined settings.
    /// 2. Determines the directory for storing cached data.
    /// 3. Checks for a previously saved file (`liked_songs.json`) to load tracks.
    ///    - If the file is found and successfully read, the function loads the cached liked tracks and uses them to initialize the struct.
    ///    - Otherwise, it fetches all liked tracks using the client.
    ///      - If the fetch operation succeeds, the tracks are saved to a local cache file.
    ///      - If the fetch operation fails, it initializes the struct with an empty track list.
    ///
    /// Debugging and diagnostic information is logged at various stages, depending on success or failure of operations.
    ///
    /// # Returns
    /// An initialized instance of the struct, either with loaded or fetched liked tracks.
    ///
    /// # Errors
    /// Errors during the fetch or cache saving process are logged using the `tracing` crate,
    /// but the function ensures a valid instance is returned even in case of failures.
    ///
    /// # Example
    /// ```rust
    /// use spotify_assistant_core::actions::liked_songs::UserLikedSongs;
    /// async fn main() {
    ///     let instance = UserLikedSongs::new().await;
    /// }
    /// ```
    pub async fn new() -> Self {
        let client = Self::set_up_client(false, Some(Self::select_scopes())).await;
        let data_dir = ProjectDirectories::Data;
        let saved_tracks_path = data_dir.path().join("liked_songs.json");
        if let Ok(tracks) = Self::load_from_file() {
            println!("Loaded liked songs from cache.");
            Self { client, tracks, saved_tracks_path }
        } else {
            let tracks = match Self::fetch_all(&client).await {
                Ok(tracks) => {
                    event!(tracing::Level::INFO, "Successfully fetched liked songs.");
                    tracks
                },
                Err(err) => {
                    event!(tracing::Level::ERROR, "Failed to fetch liked songs: {:?}", err);
                    Vec::new()
                },
            };
            let self_obj = Self {
                client,
                tracks,
                saved_tracks_path,
            };
            match self_obj.save_to_file() {
                Ok(_) => event!(tracing::Level::INFO, "Liked songs saved to cache."),
                Err(err) => event!(tracing::Level::ERROR, "Failed to save liked songs to cache: {:?}", err),
            };
            // Self {client, tracks, saved_tracks_path}
            self_obj
        }
    }

    /// Fetches all liked songs from the user's Spotify library.
    ///
    /// This asynchronous function retrieves saved (liked) songs for the current user in batches of 50
    /// until all songs are retrieved or an error occurs. It utilizes the Spotify API through `AuthCodeSpotify`
    /// client and handles pagination automatically. The function gathers all retrieved `SavedTrack`
    /// instances into a single `Vec` and returns them upon successful completion.
    ///
    /// ### Parameters
    /// - `client`: A reference to an initialized `AuthCodeSpotify` client, authenticated with proper
    ///   user credentials to fetch the user's liked songs.
    ///
    /// ### Returns
    /// - `Result<Vec<SavedTrack>, rspotify::ClientError>`:
    ///     - On success, returns a vector containing all the user's liked songs (`SavedTrack`s).
    ///     - On failure, returns a `ClientError` encapsulating details for why the operation failed.
    ///
    /// ### Behavior
    /// - Fetches liked songs in pages of 50 items, supports Spotify's offset-based pagination.
    /// - Logs events using the `tracing` crate at different levels:
    ///     - `INFO` on successful page fetch or when all songs are retrieved.
    ///     - `ERROR` if a request fails.
    /// - Stops fetching if:
    ///     - A page contains zero items (no more songs left to fetch).
    ///     - The `next` link in the response is `None`, indicating the last page.
    /// - Handles offset updates to ensure continuous fetching from where the last retrieval stopped.
    ///
    /// ### Notes
    /// - This function relies on the Spotify API endpoint for fetching saved tracks. Ensure appropriate
    ///   API permissions are granted for the user (scope: `user-library-read`).
    /// - Performance depends on network latency and the size of the user's library.
    /// - If no saved tracks are found, returns an empty vector.
    ///
    /// ### Errors
    /// - Returns `ClientError` when:
    ///     - The API client is not authenticated properly.
    ///     - Network issues occur.
    ///     - Spotify's API returns an error response.
    async fn fetch_all(client: &AuthCodeSpotify) -> Result<Vec<SavedTrack>, rspotify::ClientError> {
        let span = tracing::span!(tracing::Level::INFO, "UserLikedSongs.fetch_all");
        let _enter = span.enter();
        let mut tracks = Vec::new();
        let mut offset = 0;

        loop {
            let page = match client.current_user_saved_tracks_manual(
                Some(Self::market()), Some(50), Some(offset)
            ).await {
                Ok(page) => {
                    event!(tracing::Level::INFO, "Fetched page with {} items", page.items.len());
                    page
                },
                Err(err) => {
                    event!(tracing::Level::ERROR, "Failed to fetch liked songs: {:?}", err);
                    return Err(err)
                },
            };
            if page.items.is_empty() {
                event!(tracing::Level::INFO, "No more liked songs to fetch.");
                break;
            }
            offset += page.items.len() as u32;
            tracks.extend(page.items);
            if page.next.is_none() {
                event!(tracing::Level::INFO, "Reached the end of liked songs.");
                break;
            }
        }

        Ok(tracks)
    }

    /// Retrieves a list of saved tracks.
    ///
    /// This method returns a vector containing all the `SavedTrack` instances
    /// that are currently stored within the structure. The returned vector
    /// is a clone of the internal `tracks` field, ensuring that the original
    /// data remains unaffected by modifications to the returned vector.
    ///
    /// # Returns
    ///
    /// * `Vec<SavedTrack>` - A vector containing the saved tracks.
    ///
    /// # Example
    ///
    /// ```
    /// use spotify_assistant_core::actions::liked_songs::UserLikedSongs;
    /// async fn main() {
    ///     let my_instance = UserLikedSongs::new().await;
    ///     let saved_tracks = my_instance.tracks();
    ///     for track in saved_tracks {
    ///         println!("Track: {:?}", track);
    ///     }
    /// }
    /// ```
    pub fn tracks(&self) -> Vec<SavedTrack> {
        self.tracks.clone()
    }

    /// Saves the current state of the `tracks` field to a file in JSON format.
    ///
    /// This method serializes the `tracks` field using `serde_json` into
    /// a pretty-printed JSON string and writes the content to a file
    /// specified by the `saved_tracks_path` property. If the file
    /// does not exist, it will create a new file. If the file already exists,
    /// it will overwrite its contents.
    ///
    /// # Errors
    ///
    /// This function will return an `io::Result` error in the following cases:
    /// - If the serialization of the `tracks` field fails.
    /// - If the file cannot be created or written to.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the save operation completes successfully.
    fn save_to_file(&self) -> io::Result<()> {
        let json = serde_json::to_string_pretty(&self.tracks)?;
        let mut file = fs::File::create(self.saved_tracks_path.clone())?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    /// Returns a clone of the path to the saved tracks.
    ///
    /// # Returns
    /// * `PathBuf` - A cloned `PathBuf` representing the path to the saved tracks.
    ///
    /// # Example
    /// ```
    /// use spotify_assistant_core::actions::liked_songs::UserLikedSongs;
    /// use std::path::PathBuf;
    /// use spotify_assistant_core::enums::fs::ProjectDirectories;
    /// async fn main() {
    ///     let data_dir = ProjectDirectories::Data;
    ///     let saved_tracks_path = data_dir.path().join("liked_songs.json");
    ///     let configuration = UserLikedSongs::new().await; 
    ///     let path = configuration.saved_tracks_path();
    ///     assert_eq!(path, PathBuf::from("/music/saved_tracks"));
    /// }
    /// ```
    ///
    /// This method ensures that the original path stored in the struct remains unaltered
    /// while providing a copy to the caller.
    pub fn saved_tracks_path(&self) -> PathBuf {
        self.saved_tracks_path.clone()
    }

    /// Returns the number of tracks in the current collection.
    ///
    /// # Example
    /// ```
    /// use spotify_assistant_core::actions::liked_songs::UserLikedSongs;
    /// async fn main() {
    /// let collection = UserLikedSongs::new().await;
    /// assert_eq!(collection.number_of_tracks(), 3);
    /// }
    /// ```
    ///
    /// # Returns
    /// * `usize` - The total count of tracks in the collection.
    pub fn number_of_tracks(&self) -> usize {
        self.tracks.len()
    }

    /// Retrieves a list of track IDs from the current object.
    ///
    /// This function iterates over the `tracks` collection and extracts the ID of each track.
    /// The ID is cloned, unwrapped, and converted to a `String` before being collected into a `Vec<String>`.
    ///
    /// # Returns
    /// - `Vec<String>`: A vector containing the string representation of track IDs.
    ///
    /// # Panics
    /// - This function will panic if any of the following occurs:
    ///   - The `track.id` is `None` during the unwrap operation.
    ///
    /// # Example
    /// ```rust
    /// use spotify_assistant_core::actions::liked_songs::UserLikedSongs;
    /// async fn main() {
    ///     let my_object = UserLikedSongs::new().await;
    ///     let track_ids = my_object.track_ids();
    ///     println!("{:?}", track_ids);
    /// }
    /// ```
    pub fn track_ids(&self) -> Vec<String> {
        self.tracks.iter().map(|track| track.track.id.clone().unwrap().id().to_string()).collect()
    }

    /// Loads a list of saved tracks from a JSON file located in the application data directory.
    ///
    /// This function retrieves the path to the data directory using the `ProjectDirectories` library,
    /// appends the file name `liked_songs.json` to it, and attempts to read the file's contents.
    /// The contents are then deserialized into a vector of `SavedTrack` objects using the `serde_json` library.
    ///
    /// # Returns
    ///
    /// - `Ok(Vec<SavedTrack>)`: A vector containing the deserialized `SavedTrack` objects if the file
    ///   is read and parsed successfully.
    /// - `Err(io::Error)`: An error if there is an issue reading the file or parsing the JSON content.
    ///
    /// # Errors
    ///
    /// This function will propagate errors if:
    /// - The file `liked_songs.json` does not exist or cannot be accessed.
    /// - The file contents cannot be read as a string.
    /// - The JSON within the file cannot be deserialized into `Vec<SavedTrack>`.
    ///
    /// # Dependencies
    ///
    /// - This function uses the `ProjectDirectories` crate to determine the application's data directory.
    /// - The `serde_json` crate is required for JSON parsing.
    /// ```
    fn load_from_file() -> io::Result<Vec<SavedTrack>> {
        let data_dir = ProjectDirectories::Data;
        let liked_songs_path = data_dir.path().join("liked_songs.json");
        let contents = fs::read_to_string(liked_songs_path)?;
        let tracks: Vec<SavedTrack> = serde_json::from_str(&contents)?;
        Ok(tracks)
    }
}
