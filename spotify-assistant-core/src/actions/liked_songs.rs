use crate::enums::fs::ProjectDirectories;
use crate::paginators::PaginatorRunner;
use crate::traits::apis::Api;
use rspotify::model::{FullTrack, SavedTrack};
use rspotify::prelude::{Id, OAuthClient};
use rspotify::AuthCodeSpotify;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use tracing::{event, Level};

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
pub struct UserLibrary {
    client: AuthCodeSpotify,
    saved_tracks: Vec<SavedTrack>,
    saved_tracks_path: PathBuf,
}
impl Api for UserLibrary {
    fn select_scopes() -> std::collections::HashSet<String> {
        rspotify::scopes!("user-library-read", "user-library-modify")
    }
}
impl Default for UserLibrary {
    fn default() -> Self {
        Self {
            client: AuthCodeSpotify::default(),
            saved_tracks: Vec::new(),
            saved_tracks_path: ProjectDirectories::Data.path().join("liked_songs.json"),
        }
    }
}
impl UserLibrary {
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
    /// use spotify_assistant_core::actions::liked_songs::UserLibrary;
    /// async fn main() {
    ///     let instance = UserLibrary::new().await;
    /// }
    /// ```
    pub async fn new() -> Self {
        let mut self_obj = Self::default();

        if self_obj.does_file_exist() {
            event!(
                Level::TRACE,
                "Liked songs file found locally, attempting to load tracks."
            );
            self_obj.populate_tracks_from_file();
            let first_length = self_obj.total_tracks();
            event!(
                Level::INFO,
                "Loaded {} liked songs from file.",
                first_length
            );
            self_obj.update_library().await;
            let second_length = self_obj.total_tracks();
            if first_length < second_length {
                event!(Level::INFO, "Updating liked songs file with new tracks because the cached file is outdated.");
                self_obj.save_to_file();
            }
        } else {
            event!(
                Level::INFO,
                "No cached liked songs file found, fetching from Spotify."
            );
            self_obj.update_library().await;
            self_obj.save_to_file();
        }

        self_obj
    }

    pub fn full_tracks(&self) -> Vec<FullTrack> {
        self.saved_tracks
            .iter()
            .map(|saved_track| saved_track.track.clone())
            .collect::<Vec<FullTrack>>()
    }

    async fn update_library(&mut self) {
        let span = tracing::span!(Level::INFO, "UserLikedSongs.library");
        let _enter = span.enter();

        let liked_songs = self.client.current_user_saved_tracks(Some(Self::market()));
        let paginator = PaginatorRunner::new(liked_songs, ());
        match paginator.run().await {
            Ok(library) => self.saved_tracks = library,
            Err(err) => event!(
                Level::ERROR,
                "Could not retrieve your saved tracks: {:?}",
                err
            ),
        };
    }

    pub fn total_tracks(&self) -> usize {
        self.saved_tracks.len()
    }

    fn does_file_exist(&self) -> bool {
        self.saved_tracks_path.exists()
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
    /// use spotify_assistant_core::actions::liked_songs::UserLibrary;
    /// async fn main() {
    ///     let my_instance = UserLibrary::new().await;
    ///     let saved_tracks = my_instance.tracks();
    ///     for track in saved_tracks {
    ///         println!("Track: {:?}", track);
    ///     }
    /// }
    /// ```
    pub fn tracks(&self) -> Vec<SavedTrack> {
        self.saved_tracks.clone()
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
    fn save_to_file(&self) {
        let json = match serde_json::to_string_pretty(&self.saved_tracks) {
            Ok(json) => json,
            Err(err) => {
                event!(Level::ERROR, "Failed to serialize liked songs: {err:?}");
                return;
            }
        };
        let mut file = match fs::File::create(self.saved_tracks_path.clone()) {
            Ok(file) => file,
            Err(err) => {
                event!(Level::ERROR, "Failed to create liked songs file: {err:?}");
                return;
            }
        };
        match file.write_all(json.as_bytes()) {
            Ok(_) => {
                event!(Level::INFO, "Successfully saved liked songs to file.");
            }
            Err(err) => {
                event!(Level::ERROR, "Failed to write liked songs to file: {err:?}");
            }
        };
    }

    /// Returns a clone of the path to the saved tracks.
    ///
    /// # Returns
    /// * `PathBuf` - A cloned `PathBuf` representing the path to the saved tracks.
    ///
    /// # Example
    /// ```
    /// use spotify_assistant_core::actions::liked_songs::UserLibrary;
    /// use std::path::PathBuf;
    /// use spotify_assistant_core::enums::fs::ProjectDirectories;
    /// async fn main() {
    ///     let data_dir = ProjectDirectories::Data;
    ///     let saved_tracks_path = data_dir.path().join("liked_songs.json");
    ///     let configuration = UserLibrary::new().await;
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
    /// use spotify_assistant_core::actions::liked_songs::UserLibrary;
    /// async fn main() {
    /// let collection = UserLibrary::new().await;
    /// assert_eq!(collection.number_of_tracks(), 3);
    /// }
    /// ```
    ///
    /// # Returns
    /// * `usize` - The total count of tracks in the collection.
    pub fn number_of_tracks(&self) -> usize {
        self.saved_tracks.len()
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
    /// use spotify_assistant_core::actions::liked_songs::UserLibrary;
    /// async fn main() {
    ///     let my_object = UserLibrary::new().await;
    ///     let track_ids = my_object.track_ids();
    ///     println!("{:?}", track_ids);
    /// }
    /// ```
    pub fn track_ids(&self) -> Vec<String> {
        self.saved_tracks
            .iter()
            .map(|track| track.track.id.clone().unwrap().id().to_string())
            .collect()
    }

    fn populate_tracks_from_file(&mut self) {
        let contents = match fs::read_to_string(&self.saved_tracks_path) {
            Ok(contents) => contents,
            Err(err) => {
                event!(Level::ERROR, "Failed to read liked songs file: {:?}", err);
                return;
            }
        };
        self.saved_tracks = match serde_json::from_str(&contents) {
            Ok(tracks) => {
                event!(Level::INFO, "Successfully loaded liked songs from file.");
                tracks
            }
            Err(err) => {
                event!(Level::ERROR, "Failed to deserialize liked songs: {:?}", err);
                Vec::new()
            }
        };
    }
}
