use crate::enums::fs::ProjectDirectories;
use crate::paginator::PaginatorRunner;
use crate::traits::apis::Api;
use rspotify::model::{FullTrack, SavedTrack};
use rspotify::prelude::{Id, OAuthClient};
use rspotify::AuthCodeSpotify;
use std::io::Write;
use std::path::PathBuf;
use std::{fs, io};
use tracing::{event, span, Level};

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
    /// ```no_run,ignore
    /// use spotify_assistant_core::actions::liked_songs::UserLibrary;
    /// async fn main() {
    ///     let instance = UserLibrary::new().await;
    /// }
    /// ```
    pub async fn new() -> Self {
        let client = Self::set_up_client(false, Some(Self::select_scopes())).await;
        let data_dir = ProjectDirectories::Data;
        let saved_tracks_path = data_dir.path().join("liked_songs.json");
        if let Ok(saved_tracks) = Self::load_from_file() {
            println!("Loaded liked songs from cache.");
            Self {
                client,
                saved_tracks,
                saved_tracks_path,
            }
        } else {
            let saved_tracks = match Self::update_library(&client).await {
                Ok(tracks) => {
                    event!(Level::INFO, "Successfully fetched liked songs.");
                    tracks
                }
                Err(err) => {
                    event!(Level::ERROR, "Failed to fetch liked songs: {:?}", err);
                    Vec::new()
                }
            };
            let self_obj = Self {
                client,
                saved_tracks,
                saved_tracks_path,
            };
            match self_obj.save_to_file() {
                Ok(_) => event!(Level::INFO, "Liked songs saved to cache."),
                Err(err) => event!(
                    Level::ERROR,
                    "Failed to save liked songs to cache: {:?}",
                    err
                ),
            };
            self_obj
        }
    }

    fn load_from_file() -> io::Result<Vec<SavedTrack>> {
        let data_dir = ProjectDirectories::Data;
        let liked_songs_path = data_dir.path().join("liked_songs.json");
        let contents = fs::read_to_string(liked_songs_path)?;
        let tracks: Vec<SavedTrack> = serde_json::from_str(&contents)?;
        Ok(tracks)
    }

    pub fn full_tracks(&self) -> Vec<FullTrack> {
        self.saved_tracks
            .iter()
            .map(|saved_track| saved_track.track.clone())
            .collect::<Vec<FullTrack>>()
    }

    async fn update_library(
        client: &AuthCodeSpotify,
    ) -> Result<Vec<SavedTrack>, rspotify::ClientError> {
        let span = span!(Level::INFO, "UserLibrary.library");
        let _enter = span.enter();

        let liked_songs = client.current_user_saved_tracks(Some(Self::market()));
        let paginator = PaginatorRunner::new(liked_songs, ());
        let library = paginator.run().await.unwrap_or_else(|err| {
            event!(
                Level::ERROR,
                "Could not retrieve your saved tracks: {:?}",
                err
            );
            Vec::new()
        });
        Ok(library)
    }

    pub fn total_tracks(&self) -> usize {
        self.saved_tracks.len()
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
    /// ```no_run,ignore
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
    fn save_to_file(&self) -> io::Result<()> {
        let json = serde_json::to_string_pretty(&self.saved_tracks)?;
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
    /// ```no_run,ignore
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
    /// ```no_run,ignore
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
    /// ```no_run,ignore
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::offline::OfflineObjects;
    use std::sync::{Mutex, OnceLock};
    use tempfile::tempdir;

    fn sample_saved_track(label: &str) -> SavedTrack {
        OfflineObjects::sample_saved_track(label)
    }

    fn sample_tracks() -> Vec<SavedTrack> {
        OfflineObjects::sample_saved_tracks()
    }

    fn env_mutex() -> &'static Mutex<()> {
        static ENV_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();
        ENV_MUTEX.get_or_init(|| Mutex::new(()))
    }

    #[test]
    fn tracks_returns_clone_of_internal_state() {
        let temp_dir = tempdir().expect("temporary directory");
        let file_path = temp_dir.path().join("liked_songs.json");
        let tracks = sample_tracks();
        let liked_songs = UserLibrary {
            client: AuthCodeSpotify::default(),
            saved_tracks: tracks.clone(),
            saved_tracks_path: file_path,
        };

        let mut cloned_tracks = liked_songs.tracks();
        cloned_tracks.clear();

        assert_eq!(liked_songs.saved_tracks.len(), tracks.len());
        assert_ne!(liked_songs.saved_tracks.len(), cloned_tracks.len());
    }

    #[test]
    fn number_of_tracks_matches_len() {
        let temp_dir = tempdir().expect("temporary directory");
        let file_path = temp_dir.path().join("liked_songs.json");
        let tracks = sample_tracks();
        let liked_songs = UserLibrary {
            client: AuthCodeSpotify::default(),
            saved_tracks: tracks.clone(),
            saved_tracks_path: file_path,
        };

        assert_eq!(liked_songs.number_of_tracks(), tracks.len());
    }

    #[test]
    fn track_ids_extracts_ids() {
        let temp_dir = tempdir().expect("temporary directory");
        let file_path = temp_dir.path().join("liked_songs.json");
        let saved_tracks = sample_tracks();
        let liked_songs = UserLibrary {
            client: AuthCodeSpotify::default(),
            saved_tracks,
            saved_tracks_path: file_path,
        };

        assert_eq!(
            liked_songs.track_ids(),
            vec![
                "AAAAAAAAAAAAAAAAAAAAAA".to_string(),
                "DDDDDDDDDDDDDDDDDDDDDD".to_string()
            ]
        );
    }

    #[test]
    fn saved_tracks_path_returns_clone() {
        let temp_dir = tempdir().expect("temporary directory");
        let file_path = temp_dir.path().join("liked_songs.json");
        let saved_tracks = sample_tracks();
        let liked_songs = UserLibrary {
            client: AuthCodeSpotify::default(),
            saved_tracks,
            saved_tracks_path: file_path.clone(),
        };

        let mut returned_path = liked_songs.saved_tracks_path();
        assert_eq!(returned_path, file_path);
        returned_path.push("extra");
        assert_ne!(returned_path, liked_songs.saved_tracks_path);
    }

    #[test]
    fn save_to_file_persists_tracks() {
        let temp_dir = tempdir().expect("temporary directory");
        let file_path = temp_dir.path().join("liked_songs.json");
        let tracks = sample_tracks();
        let liked_songs = UserLibrary {
            client: AuthCodeSpotify::default(),
            saved_tracks: tracks.clone(),
            saved_tracks_path: file_path.clone(),
        };

        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).expect("create parent directory");
        }

        liked_songs.save_to_file().expect("save liked songs");

        let persisted = fs::read_to_string(&file_path).expect("read persisted file");
        let parsed_tracks: Vec<SavedTrack> =
            serde_json::from_str(&persisted).expect("parse persisted tracks");
        assert_eq!(parsed_tracks, tracks);
    }

    struct EnvVarGuard {
        key: &'static str,
        original: Option<String>,
    }

    impl EnvVarGuard {
        unsafe fn set(key: &'static str, value: &str) -> Self {
            unsafe {
                let original = std::env::var(key).ok();
                std::env::set_var(key, value);
                Self { key, original }
            }
        }
    }

    impl Drop for EnvVarGuard {
        fn drop(&mut self) {
            if let Some(ref value) = self.original {
                unsafe {
                    std::env::set_var(self.key, value);
                }
            } else {
                unsafe {
                    std::env::remove_var(self.key);
                }
            }
        }
    }

    #[test]
    fn load_from_file_reads_tracks_from_cache() {
        let _guard = env_mutex().lock().expect("lock environment mutex");
        let temp_dir = tempdir().expect("temporary directory");
        let env_guard = unsafe {
            EnvVarGuard::set(
                "XDG_DATA_HOME",
                temp_dir.path().to_str().expect("utf-8 path"),
            )
        };
        let data_dir = ProjectDirectories::Data.path();
        fs::create_dir_all(&data_dir).expect("create data directory");
        let file_path = data_dir.join("liked_songs.json");
        let tracks = sample_tracks();
        fs::write(
            &file_path,
            serde_json::to_string(&tracks).expect("serialize tracks"),
        )
            .expect("write tracks to cache");

        let loaded = UserLibrary::load_from_file().expect("load tracks from cache");
        assert_eq!(loaded, tracks);

        drop(env_guard);
    }
}
