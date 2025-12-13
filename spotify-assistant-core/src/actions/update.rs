use std::collections::HashSet;

use crate::actions::exploration::playlist::PlaylistXplr;
use crate::enums::pl::PlaylistType;
use crate::models::blacklist::{Blacklist, BlacklistArtist};
use crate::traits::apis::Api;
use rspotify::model::Id;
use rspotify::model::{AlbumId, FullPlaylist, FullTrack, PlayableItem, PlaylistId, TrackId};
use rspotify::prelude::*;
use rspotify::{AuthCodeSpotify, scopes};
use tracing::{Level, error, event};

/// The `Editor` struct is used to manage and handle Spotify playlists, serving as a utility
/// to reference and manipulate playlists using the Spotify API.
///
/// # Fields
///
/// - `client` (`AuthCodeSpotify`):
///   An authenticated Spotify client used to make API calls and manage playlist operations.
///
/// - `ref_id` (`PlaylistId<'static>`):
///   The ID of the reference playlist. Typically serves as the source or "template" playlist
///   for comparison or duplication purposes.
///
/// - `target_id` (`PlaylistId<'static>`):
///   The ID of the target playlist. Represents the playlist that will be updated, cloned, or manipulated
///   based on the operations applied.
///
/// - `ref_pl` (`FullPlaylist`):
///   Holds the full details of the reference playlist, including its metadata, tracks, and other information.
///   This structure provides a detailed snapshot of the source playlist for playlist operations.
///
/// - `target_pl` (`FullPlaylist`):
///   Contains the full details of the target playlist, including its metadata, tracks, and other information.
///   This allows for interacting with and updating the target playlist as needed.
///
/// # Purpose
///
/// The `Editor` struct is designed to facilitate complex playlist operations, such as:
/// - Comparing playlists (e.g., finding differences or similarities between two playlists).
/// - Duplicating playlists or replicating tracks from one playlist into another.
/// - Synchronizing or updating playlists based on another playlist's content.
///
/// The struct leverages Spotify's API capabilities through the provided `client` to ensure
/// efficient and authenticated interaction with playlists.
#[derive(Debug)]
pub struct Editor {
    client: AuthCodeSpotify,
    ref_id: PlaylistId<'static>,
    target_id: PlaylistId<'static>,
    ref_pl: FullPlaylist,
    target_pl: FullPlaylist,
}

impl Api for Editor {
    fn select_scopes() -> HashSet<String> {
        scopes!(
            "playlist-read-private",
            "playlist-read-collaborative",
            "playlist-modify-public",
            "playlist-modify-private"
        )
    }
}
impl Editor {
    /// Asynchronously creates a new instance of `Self` configured for the Release Radar feature.
    ///
    /// This function initializes an `Editor` with specific identifiers for the Release Radar playlists:
    /// - `PlaylistType::StockRR.get_id()` provides the ID for the stock (default) Release Radar playlist.
    /// - `PlaylistType::MyRR.get_id()` provides the ID for the personalized Release Radar playlist.
    ///
    /// # Returns
    /// A future that resolves to an instance of the calling type configured with the above playlist information.
    ///
    /// # Example
    /// ```no_run,ignore
    /// use spotify_assistant_core::actions::update::Editor;
    ///
    /// async fn main() {
    ///     let release_radar_instance = Editor::release_radar().await;
    /// }
    /// ```
    ///
    /// # Notes
    /// - This function is asynchronous and should be awaited.
    /// - Ensure that the `Editor` type being used is properly configured elsewhere in the codebase to handle the supplied IDs.
    pub async fn release_radar() -> Self {
        Editor::new(PlaylistType::StockRR.get_id(), PlaylistType::MyRR.get_id()).await
    }
    pub fn ref_pl_tracks(&self) -> Vec<FullTrack> {
        self.ref_pl
            .tracks
            .items
            .iter()
            .filter_map(|item| {
                match item.track.clone() {
                    Some(PlayableItem::Track(track)) => Some(track),
                    _ => None, // Skip if not a track
                }
            })
            .collect::<Vec<FullTrack>>()
    }

    /// Retrieves a full playlist from Spotify using the provided client and playlist ID.
    ///
    /// # Arguments
    ///
    /// * `client` - A reference to an authenticated `AuthCodeSpotify` client instance
    ///   used to make requests to the Spotify API.
    /// * `pl_id` - A `PlaylistId` representing the unique identifier of the playlist to retrieve.
    ///
    /// # Returns
    ///
    /// Returns a `FullPlaylist` object representing the playlist metadata and content.
    ///
    /// # Errors
    ///
    /// This function will `panic!` if there is an error retrieving the playlist, logging
    /// the error details and the provided playlist ID.
    ///
    /// # Remarks
    ///
    /// The function uses the Spotify client to fetch the details of a playlist, and it assumes that
    /// the provided `AuthCodeSpotify` client is correctly authenticated and initialized.
    /// Ensure to call this function within an asynchronous runtime due to the `async` nature of the
    /// Spotify API client.
    async fn playlist_from_id(
        client: &AuthCodeSpotify,
        pl_id: PlaylistId<'static>,
    ) -> FullPlaylist {
        match client
            .playlist(pl_id.clone(), None, Some(Self::market()))
            .await
        {
            Ok(pl) => pl,
            Err(err) => {
                error!("Error: {err:?}");
                panic!("Could not retrieve playlist with ID, '{pl_id:?}'");
            }
        }
    }

    /// Creates a new instance of the `Editor` struct.
    ///
    /// This asynchronous function initializes the `Editor` by performing the following steps:
    /// 1. Sets up an HTTP client with predetermined configurations, including required API scopes.
    /// 2. Fetches detailed information about the reference playlist (`ref_id`) and the target playlist (`target_id`)
    ///    using their respective playlist IDs.
    /// 3. Constructs and returns an `Editor` instance populated with the fetched playlist data and the initialized client.
    ///
    /// # Arguments
    ///
    /// * `ref_id` - The ID of the reference playlist. This ID is used to fetch details about the playlist you are referencing.
    /// * `target_id` - The ID of the target playlist. This ID represents the playlist that the editor will work with.
    ///
    /// # Returns
    ///
    /// Returns an `Editor` instance containing:
    /// - A configured HTTP client.
    /// - The reference playlist ID (`ref_id`).
    /// - The target playlist ID (`target_id`).
    /// - Playlist details for the reference playlist (`ref_pl`).
    /// - Playlist details for the target playlist (`target_pl`).
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The HTTP client setup fails.
    /// - Playlist information cannot be fetched using the specified IDs.
    ///
    /// # Examples
    /// ```no_run,ignore
    /// use spotify_assistant_core::actions::update::Editor;
    /// use spotify_assistant_core::enums::pl::PlaylistType;
    ///
    /// async fn main() {
    ///     let editor = Editor::new(PlaylistType::StockRR.get_id(), PlaylistType::MyRR.get_id()).await;
    /// }
    /// // Perform operations on playlists using the editor
    /// ```
    pub async fn new(ref_id: PlaylistId<'static>, target_id: PlaylistId<'static>) -> Self {
        let client = Self::set_up_client(false, Some(Self::select_scopes())).await;
        let target_pl = Self::playlist_from_id(&client, target_id.clone()).await;
        let ref_pl = Self::playlist_from_id(&client, ref_id.clone()).await;
        Editor {
            client,
            ref_id,
            target_id,
            ref_pl,
            target_pl,
        }
    }

    /// Asynchronously removes liked songs from the target playlist.
    ///
    /// This function identifies and removes songs marked as "liked" from the
    /// target playlist specified by `self.target_id`. The removal process handles
    /// the tracks in batches to improve efficiency when dealing with large playlists.
    ///
    /// ## Process Overview:
    /// 1. Creates a `PlaylistXplr` instance and identifies "liked" songs within the target playlist.
    /// 2. Transforms the liked songs into a list of `PlayableId` instances.
    /// 3. Iteratively removes the liked songs from the playlist in chunks of 100 items per batch,
    ///    updating the playlist snapshot after each successful batch process.
    ///
    /// ## Logging:
    /// - DEBUG: Logs detailed information, such as the count of liked songs and real-time progress status.
    /// - INFO: Provides updates when liked songs are successfully removed and when playlist data is refreshed.
    /// - ERROR: Captures any errors encountered during the removal or playlist fetching operations.
    ///
    /// ## Error Handling:
    /// - Panics if a track lacks a unique ID or if the target playlist cannot be refreshed.
    /// - Panics if the API call to remove songs fails for any reason.
    ///
    /// ## Snapshot:
    /// Keeps track of the playlist's snapshot ID to ensure that changes are applied consistently
    /// and to avoid potential conflicts from concurrent playlist updates.
    ///
    /// ## Batching:
    /// - Processes liked songs in chunks of 100 to comply with API restrictions and manage network resources.
    ///
    /// ## Example Usage:
    /// ```no_run,ignore
    /// use spotify_assistant_core::actions::update::Editor;
    /// use spotify_assistant_core::enums::pl::PlaylistType;
    ///
    /// async fn main() {
    ///     let mut editor = Editor::new(PlaylistType::StockRR.get_id(), PlaylistType::MyRR.get_id()).await;
    ///     editor.remove_liked_songs().await;
    /// }
    /// ```
    ///
    /// ## Prerequisites:
    /// Ensure that `self.client` is a valid Spotify client and that `self.target_id`
    /// refers to an existing playlist with valid permissions for modification.
    ///
    /// ## Panics:
    /// - If a track lacks an ID.
    /// - If there is a failure in refreshing the target playlist.
    /// - If any batch removal API call fails.
    ///
    /// ## Dependencies:
    /// - `playlist_remove_all_occurrences_of_items`: API endpoint to remove tracks.
    /// - `playlist`: API call to refresh the playlist metadata.
    ///
    /// ## Notes:
    /// - Make sure that large playlist modifications are logged appropriately to trace actions during execution.
    /// - This function assumes that the user has adequate permission to modify the specified playlist.
    pub async fn remove_liked_songs(&mut self) {
        let span = tracing::span!(Level::DEBUG, "remove_liked_songs");
        let _enter = span.enter();

        let xplr = PlaylistXplr::new(self.target_id.clone(), false).await;
        let is_liked_hashmap = xplr.find_liked_songs().await;
        let liked = is_liked_hashmap.get("liked").unwrap();
        let liked_song_ids = liked
            .iter()
            .map(|track| match PlayableItem::Track(track.clone()).id() {
                None => {
                    panic!("Track does not have an ID.")
                }
                Some(id) => id.into_static(),
            })
            .collect::<Vec<PlayableId>>();
        event!(
            Level::INFO,
            "Removing liked songs from {:?}. Current track number: {:?} | Snapshot ID: {:?}",
            self.target_pl.name,
            self.target_pl.tracks.total,
            self.target_snapshot()
        );
        event!(
            Level::DEBUG,
            "Liked songs count: {:?}",
            liked_song_ids.len()
        );
        for batch in liked_song_ids.chunks(100) {
            match self
                .client
                .playlist_remove_all_occurrences_of_items(
                    self.target_id.clone(),
                    batch.to_vec(),
                    Some(self.target_snapshot().as_str()),
                )
                .await
            {
                Ok(snapshot_id) => {
                    self.target_pl = match self
                        .client
                        .playlist(self.target_id.clone(), None, Some(Self::market()))
                        .await
                    {
                        Ok(pl) => pl,
                        Err(err) => {
                            error!("Error: {:?}", err);
                            panic!("Could not retrieve target playlist");
                        }
                    };
                    event!(
                        Level::INFO,
                        "Removed liked songs from {:?}. Updated track number: {:?} | Snapshot ID: {:?}",
                        self.target_pl.name,
                        self.target_pl.tracks.total,
                        snapshot_id
                    );
                }
                Err(err) => {
                    error!("Error: {:?}", err);
                    panic!("Could not remove liked songs");
                }
            };
        }
    }

    /// Retrieves a clone of the playlist's reference ID.
    ///
    /// # Returns
    ///
    /// * `PlaylistId` - A clone of the `ref_id` associated with the playlist.
    ///
    /// This method allows access to the `ref_id` field of a playlist,
    /// returning it as a new cloned instance. Useful when the reference ID
    /// needs to be accessed or reused without modifying the original value.
    ///
    /// # Example
    /// ```no_run,ignore
    /// use spotify_assistant_core::actions::update::Editor;
    /// use spotify_assistant_core::enums::pl::PlaylistType;
    ///
    /// async fn main() {
    ///     let editor = Editor::new(PlaylistType::StockRR.get_id(), PlaylistType::MyRR.get_id()).await;
    ///     let ref_id = editor.reference_id();
    ///     println!("Reference ID: {:?}", ref_id);
    /// }
    /// ```
    ///
    /// Here, `reference_id` provides access to a cloned version of the
    /// playlist's `ref_id`.
    pub fn reference_id(&self) -> PlaylistId<'_> {
        self.ref_id.clone()
    }

    /// Returns the `PlaylistId` associated with the target.
    ///
    /// # Description
    /// This method provides a way to access the `target_id` of the current instance.
    /// The `target_id` represents the unique identifier of a playlist.
    ///
    /// # Returns
    /// A `PlaylistId` that is a clone of the `target_id`.
    ///
    /// # Examples
    /// ```no_run,ignore
    /// use spotify_assistant_core::actions::update::Editor;
    /// use spotify_assistant_core::enums::pl::PlaylistType;
    ///
    /// async fn main() {
    ///     let editor = Editor::new(PlaylistType::StockRR.get_id(), PlaylistType::MyRR.get_id()).await;
    ///     let id = editor.target_id();
    ///     println!("Target ID: {:?}", id);
    /// }
    /// ```
    pub fn target_id(&self) -> PlaylistId<'_> {
        self.target_id.clone()
    }

    /// Returns a cloned version of the `FullPlaylist` referenced by the `ref_pl` field.
    ///
    /// This method provides a way to retrieve a full copy of the internal playlist object
    /// (`ref_pl`) contained within the current instance. As the playlist is cloned, any modifications
    /// to the returned value will not affect the original object's state.
    ///
    /// # Returns
    ///
    /// A `FullPlaylist` object that is a clone of the `ref_pl` field.
    ///
    /// # Examples
    /// ```no_run,ignore
    /// use spotify_assistant_core::actions::update::Editor;
    /// use spotify_assistant_core::enums::pl::PlaylistType;
    ///
    /// async fn main() {
    ///     let editor = Editor::new(PlaylistType::StockRR.get_id(), PlaylistType::MyRR.get_id()).await;
    ///     let playlist = editor.reference_playlist();
    ///     println!("Playlist ID: {:?}", playlist.id);
    /// }
    /// // Use `playlist` as needed, without impacting the original `ref_pl`.
    /// ```
    pub fn reference_playlist(&self) -> FullPlaylist {
        self.ref_pl.clone()
    }

    /// Retrieves the target playlist associated with the current instance.
    ///
    /// This method returns a clone of the `FullPlaylist` object stored
    /// in the `target_pl` field of the instance. Cloning ensures that
    /// the original playlist remains unmodified while providing access
    /// to its data.
    ///
    /// # Returns
    ///
    /// * `FullPlaylist` - A cloned instance of the target playlist.
    ///
    /// # Example
    /// ```no_run,ignore
    /// use spotify_assistant_core::actions::update::Editor;
    /// use spotify_assistant_core::enums::pl::PlaylistType;
    ///
    /// async fn main() {
    ///     let editor = Editor::new(PlaylistType::StockRR.get_id(), PlaylistType::MyRR.get_id()).await;
    ///     let playlist = editor.target_playlist();
    ///     println!("Playlist ID: {:?}", playlist.id);
    /// }
    /// // Use `playlist` as needed, without impacting the original `ref_pl`.
    /// ```
    ///
    /// # Note
    ///
    /// Since the method clones the playlist, ensure that the clone
    /// operation is necessary to prevent unnecessary memory usage
    /// for large playlists.
    pub fn target_playlist(&self) -> FullPlaylist {
        self.target_pl.clone()
    }

    /// Retrieves the snapshot ID from the `target_pl` field and returns it as a `String`.
    ///
    /// This function accesses the `snapshot_id` of the `target_pl` field, clones it, and
    /// prints the obtained snapshot ID to the console for debugging purposes.
    ///
    /// # Returns
    ///
    /// * `String` - The cloned snapshot ID.
    ///
    /// # Example
    /// ```no_run,ignore
    /// use spotify_assistant_core::actions::update::Editor;
    /// use spotify_assistant_core::enums::pl::PlaylistType;
    ///
    /// async fn main() {
    ///     let editor = Editor::new(PlaylistType::StockRR.get_id(), PlaylistType::MyRR.get_id()).await;
    ///     let snapshot_id = editor.target_snapshot();
    ///     println!("Snapshot ID: {:?}", snapshot_id);
    /// }
    /// ```
    ///
    /// # Debug Output
    /// Prints the snapshot ID to the console in a debug formatting:
    /// ```text
    /// Snapshot ID: "your_snapshot_id_here"
    /// ```
    pub fn target_snapshot(&self) -> String {
        let snapshot = self.target_pl.snapshot_id.clone();
        println!("Snapshot ID: {snapshot:?}");
        snapshot
    }

    /// Retrieves the album ID associated with a given track.
    ///
    /// This function takes a reference to a `FullTrack` object and attempts to extract the `album`'s ID from it.
    /// If the album does not have an ID (i.e., the `id` field in the album is `None`),
    /// the function will panic with an error message.
    ///
    /// # Arguments
    ///
    /// * `full_track` - A reference to the `FullTrack` object from which the album ID is to be retrieved.
    ///
    /// # Returns
    ///
    /// * An `AlbumId` representing the identifier of the album associated with the provided track.
    ///
    /// # Panics
    ///
    /// This function will panic if the `album` does not have an ID (i.e., `album.id` is `None`).
    ///
    /// In the above example, the function successfully retrieves the album ID.
    /// If the `album.id` was `None`, the function would have panicked.
    fn get_track_album_id(&self, full_track: &FullTrack) -> AlbumId<'_> {
        match full_track.album.id.clone() {
            None => {
                panic!("Track does not have an album ID.")
            }
            Some(album_id) => album_id,
        }
    }

    /// Asynchronously retrieves a filtered list of album IDs associated with the reference
    /// playlist's tracks, excluding tracks by blacklisted artists.
    ///
    /// # Details
    /// This function inspects the tracks in the `ref_pl` playlist, excluding tracks where
    /// the lead artist is included in the current blacklist. The filtering process logs the
    /// blacklist and decisions related to whether an artist or track is excluded. The album
    /// IDs from the non-blacklisted tracks are then returned as a `Vec<AlbumId>`.
    ///
    /// # Logging
    /// - Logs the current blacklist at `DEBUG` level.
    /// - Logs when a blacklisted artist is encountered at `INFO` level and skips processing their tracks.
    ///
    /// # Return
    /// Returns a `Vec<AlbumId>` containing album IDs of tracks for non-blacklisted artists.
    ///
    /// # Panics
    /// - The function panics if it fails to clone the ID of the lead artist of a track.
    ///   Ensure that all artist records in the playlist contain valid and clonable IDs.
    ///
    /// # Example
    /// ```no_run,ignore
    /// use spotify_assistant_core::actions::update::Editor;
    /// use spotify_assistant_core::enums::pl::PlaylistType;
    ///
    /// async fn main() {
    ///     let editor = Editor::new(PlaylistType::StockRR.get_id(), PlaylistType::MyRR.get_id()).await;
    ///     let album_ids = editor.get_reference_track_album_ids_filtered().await;
    ///     for album_id in album_ids {
    ///         println!("Album ID: {:?}", album_id);
    ///     }
    /// }
    /// ```
    ///
    /// # Errors
    /// This function doesn't explicitly return errors but will omit tracks that don't match
    /// the `PlayableItem::Track` variant or have missing album IDs.
    ///
    /// # Notes
    ///
    pub async fn get_reference_track_album_ids_filtered(&self) -> Vec<AlbumId<'_>> {
        let span = tracing::span!(
            Level::DEBUG,
            "Editor.get_reference_track_album_ids_filtered"
        );
        let _enter = span.enter();

        let blacklist = Blacklist::default().artists();
        event!(Level::DEBUG, "Current blacklist: {:?}", blacklist);
        self.ref_pl
            .tracks
            .items
            .iter()
            .filter_map(|track| match track.track {
                Some(PlayableItem::Track(ref track)) => {
                    let lead_artist_id = track
                        .artists
                        .first()
                        .unwrap()
                        .id
                        .clone()
                        .expect("Could not clone artist ID")
                        .to_string();
                    let lead_artist_name = track.artists.first().unwrap().name.clone();
                    let hypothetical_blacklist_artist =
                        BlacklistArtist::new(lead_artist_name.clone(), lead_artist_id.clone());
                    if blacklist.contains(&hypothetical_blacklist_artist) {
                        event!(
                            Level::INFO,
                            "Artist {:?} is blacklisted. Skipping album ID retrieval.",
                            lead_artist_name
                        );
                        None
                    } else {
                        let album_id = self.get_track_album_id(track);
                        Some(album_id)
                    }
                }
                _ => None,
            })
            .collect()
    }

    /// Asynchronously retrieves a list of unique track IDs from album references.
    ///
    /// # Description
    /// This method fetches track IDs associated with the albums referenced within the instance.
    /// It performs the following workflow:
    /// 1. Fetches album IDs by calling an internal asynchronous method `get_reference_track_album_ids_filtered`.
    /// 2. Splits the album IDs into manageable chunks (of size 20) for processing to comply with potential API constraints.
    /// 3. For each chunk of album IDs, retrieves the album details (including track information) using the client.
    /// 4. Extracts track IDs from the retrieved albums, ensuring all IDs are unique by maintaining a deduplicated vector.
    /// 5. Prints the size of the return vector and the cleaned track ID vector for debugging purposes.
    ///
    /// The function ultimately returns a deduplicated vector of track IDs.
    ///
    /// # Returns
    /// - `Vec<TrackId>`: A vector containing unique track IDs associated with the referenced albums.
    ///
    /// # Panics
    /// - The function will panic if:
    ///   - Retrieving albums from album IDs fails.
    ///   - Cloning track IDs fails.
    ///
    /// # Notes
    /// - This function uses an internal helper method `Self::append_uniques` to append unique track IDs to the result vector.
    /// - Another internal helper method `Self::clean_duplicate_id_vector` is used to remove any duplicate track IDs inadvertently introduced.
    ///
    /// # Examples
    /// ```no_run,ignore
    /// use spotify_assistant_core::actions::update::Editor;
    /// use spotify_assistant_core::enums::pl::PlaylistType;
    ///
    /// async fn main() {
    ///     let editor = Editor::new(PlaylistType::StockRR.get_id(), PlaylistType::MyRR.get_id()).await;
    ///     let unique_track_ids = editor.get_album_tracks_from_reference().await;
    ///     println!("Unique Track IDs: {:?}", unique_track_ids);
    /// }
    /// ```
    ///
    /// # Debugging
    /// - A debug statement is printed to the console showing the sizes of the return vector and the deduplicated track ID vector for verification.
    pub async fn get_album_tracks_from_reference(&self) -> Vec<TrackId<'_>> {
        let album_ids = self.get_reference_track_album_ids_filtered().await;
        let mut return_vector = Vec::new();
        let mut album_track_ids = Vec::new();
        for chunk in album_ids.chunks(20) {
            let albums = self
                .client
                .albums(chunk.to_vec(), Some(Self::market()))
                .await
                .expect("Could not retrieve albums from album IDs");

            albums.iter().for_each(|album| {
                let album_track_ids_vec = album
                    .tracks
                    .items
                    .iter()
                    .map(|track| track.id.clone().expect("Could not clone track ID"))
                    .collect::<Vec<TrackId>>();

                return_vector = Self::append_uniques(&return_vector, &album_track_ids_vec);
                album_track_ids.extend(album_track_ids_vec);
            });
        }
        album_track_ids = Self::clean_duplicate_id_vector(album_track_ids);
        println!(
            "Return length: {:?} | ID length {:?}",
            return_vector.len(),
            album_track_ids.len()
        );
        return_vector
    }

    /// Asynchronously wipes all tracks from the reference playlist tied to the current `Editor` instance.
    ///
    /// This function utilizes tracing instrumentation for debugging. It performs the following:
    /// 1. Initializes a span for logging purposes.
    /// 2. Creates an instance of `PlaylistXplr` to explore the current reference playlist.
    /// 3. Extracts the track identifiers (IDs) from the playlist tracks. If a track does not have
    ///    an ID, the function panics with an appropriate message.
    /// 4. Iterates over the track IDs in chunks of up to 100 and sends removal requests to
    ///    the remote client API.
    /// 5. For each chunk removal request:
    ///     - Logs an informational event if successful.
    ///     - Logs an error and panics if the removal request fails.
    ///
    /// # Panics
    /// This function will panic in the following situations:
    /// - A track in the playlist does not have an ID.
    /// - Any batch of tracks fails to be removed from the playlist due to an error in the API call.
    ///
    /// # Errors and Logging
    /// - If an error occurs during the removal API call, an appropriate error message will be logged,
    ///   and the program will terminate via `panic!`.
    ///
    /// # Dependencies
    /// - `crate::tracing`: Used for logging and tracing the flow of execution for debugging purposes.
    /// - `PlaylistXplr`: A utility to explore playlists and retrieve track data.
    /// - `PlayableItem`, `PlayableId`: Abstractions for dealing with playable entities in the playlist.
    /// - `self.client`: The client for interacting with the playlist API.
    /// - Each API request batch handles a maximum of 100 tracks at a time.
    ///
    /// # Example Usage
    /// ```no_run,ignore
    /// use spotify_assistant_core::actions::update::Editor;
    /// use spotify_assistant_core::enums::pl::PlaylistType;
    ///
    /// async fn main() {
    ///     let editor = Editor::new(PlaylistType::StockRR.get_id(), PlaylistType::MyRR.get_id()).await;
    ///     editor.wipe_reference_playlist().await;
    ///     assert!(editor.reference_playlist().tracks.items.is_empty(), "Reference playlist should be empty after wipe.");
    /// }
    /// ```
    ///
    /// This will remove all tracks from the reference playlist associated with the `Editor` instance.
    pub async fn wipe_reference_playlist(&self) {
        let span = tracing::span!(Level::DEBUG, "Editor.wipe_reference_playlist");
        let _enter = span.enter();
        let xplorer = PlaylistXplr::new(self.ref_id.clone(), false).await;
        let track_ids = xplorer.playable_ids();

        for batch in track_ids.chunks(100) {
            match self
                .client
                .playlist_remove_all_occurrences_of_items(self.ref_id.clone(), batch.to_vec(), None)
                .await
            {
                Ok(_) => {
                    event!(Level::INFO, "Removed tracks from reference playlist.");
                }
                Err(err) => {
                    error!("Error: {:?}", err);
                    panic!("Could not remove tracks from reference playlist");
                }
            }
        }
    }

    /// This method checks if the Stock Release Radar ID was used and logs an error or proceeds to log
    /// an informational message depending on the scenario.
    ///
    /// # Arguments
    ///
    /// * `number_of_ids` - The number of song IDs being updated in the Full Release Radar.
    ///
    /// # Behavior
    ///
    /// - If `self.target_id` corresponds to the `Stock Release Radar` ID (as retrieved by `PlaylistType::StockRR.get_id()`):
    ///   - Logs an error message with the stock playlist ID.
    ///   - Panics with a message instructing the caller to use the full version Release Radar ID instead of the stock version.
    /// - Otherwise:
    ///   - Logs an informational message indicating that the Full Release Radar playlists will be updated with the specified number of songs.
    ///
    /// # Panics
    ///
    /// Panics with a descriptive message if the Stock Release Radar ID is incorrectly used instead of the full version ID.
    ///
    /// # Notes
    ///
    /// - This method is useful for ensuring correct playlist updates and preventing unintended updates to stock playlists.
    ///
    /// # Logging
    ///
    /// The method uses structured logging with either `Level::ERROR` or `Level::INFO`.
    ///
    fn check_if_stock_release_radar_id_was_used(&self, number_of_ids: usize) {
        if self.target_id.clone() == PlaylistType::StockRR.get_id() {
            event!(
                Level::ERROR,
                "Your Stock Release Radar ID was used: {playlist_id}",
                playlist_id = self.target_id.id()
            );
            panic!(
                "You must ensure that you are calling the update method with your full version release radar ID instead of your stock version's."
            )
        } else {
            event!(
                Level::INFO,
                "Your Full Release Radar playlists will be updated with {number_of_ids} songs",
            );
        }
    }

    /// Generates a description string for the "Release Radar" playlist.
    ///
    /// The function retrieves the current local date using the `chrono` crate, formats it into
    /// a `MM/DD/YYYY` string representation, and incorporates it into a template string indicating
    /// when the playlist description was last updated.
    ///
    /// # Returns
    /// * `String` - A formatted string that describes the "Release Radar" playlist, including
    ///   the creation date (hardcoded as "11/02/2023") and the dynamically updated current date.
    fn generate_release_radar_description(&self) -> String {
        let local_time = chrono::Local::now();
        let local_time_string = local_time.format("%m/%d/%Y").to_string();
        format!(
            "Release Radar playlists with songs from albums included. Created on 11/02/2023. Updated on {local_time_string}."
        )
    }

    /// Updates the target playlist with tracks obtained from an album reference.
    ///
    /// This asynchronous function performs the following tasks:
    /// 1. Retrieves album track IDs from a reference.
    /// 2. Checks if the stock release radar ID has been used.
    /// 3. Updates the playlist in chunks of 20 tracks, taking care to:
    ///    - Set the playlist's description for the first chunk.
    ///    - Replace the playlist's existing items with the first chunk of tracks.
    ///    - Append subsequent chunks of tracks to the playlist.
    /// 4. Clears the reference playlist upon completion.
    ///
    /// # Implementation Details
    /// - A tracing span is created (with debug-level logging) to monitor the execution of this function.
    /// - The playlist description is generated only once (for the first chunk) to avoid overwriting details
    ///   for subsequent chunks.
    /// - Any errors occurring during updates, replacements, or additions to the target playlist will cause
    ///   a panic, as these operations expect to succeed.
    ///
    /// # Panics
    /// - If the playlist description update fails.
    /// - If replacing playlist items with the initial chunk fails.
    /// - If adding items to the playlist from subsequent chunks fails.
    ///
    /// # Example
    /// ```no_run,ignore
    /// use spotify_assistant_core::actions::update::Editor;
    /// use spotify_assistant_core::enums::pl::PlaylistType;
    ///
    /// async fn main() {
    ///     let editor = Editor::new(PlaylistType::StockRR.get_id(), PlaylistType::MyRR.get_id()).await;
    ///     editor.update_playlist().await;
    /// }
    /// ```
    ///
    /// # Notes
    /// - This function assumes that the `target_id` is valid and properly configured.
    /// - The `wipe_reference_playlist` method is called at the end to clean up the reference playlist.
    pub async fn update_playlist(&self) {
        let span = tracing::span!(Level::DEBUG, "Editor.update_playlist");
        let _enter = span.enter();
        let ids = self.get_album_tracks_from_reference().await;
        self.check_if_stock_release_radar_id_was_used(ids.len());

        let mut first_chunk = true;
        for chunk in ids.chunks(20) {
            let chunk_iterated = chunk
                .iter()
                .map(|track| PlayableId::Track(track.as_ref()))
                .collect();

            first_chunk = self
                .update_playlist_from_chunk(chunk_iterated, first_chunk)
                .await;
        }
        self.wipe_reference_playlist().await;
    }
    pub async fn update_rr_from_xplorer(&self) {
        let span = tracing::span!(Level::DEBUG, "Editor.update_playlist_from_xplorer");
        let _enter = span.enter();
        let mut xplorer = PlaylistXplr::new(self.ref_id.clone(), false).await;
        xplorer.set_tracks_to_unique_from_expanded();
        // xplorer.tracks = xplorer.unique_tracks();
        let track_ids = xplorer.playable_ids();
        self.check_if_stock_release_radar_id_was_used(track_ids.len());

        let mut first_chunk = true;
        for chunk in track_ids.chunks(20) {
            first_chunk = self
                .update_playlist_from_chunk(chunk.to_vec(), first_chunk)
                .await;
        }
    }

    pub async fn update_playlist_from_chunk(
        &self,
        chunk: Vec<PlayableId<'_>>,
        is_first: bool,
    ) -> bool {
        let span = tracing::span!(Level::DEBUG, "Editor.update_playlist_from_chunk");
        let _enter = span.enter();

        if is_first {
            let description = self.generate_release_radar_description();
            self.client
                .playlist_change_detail(
                    self.target_id.clone(),
                    None,
                    None,
                    Some(description.as_str()),
                    None,
                )
                .await
                .expect("Couldn't update description");
            event!(Level::DEBUG, "Replacing playlist items.");
            self.client
                .playlist_replace_items(self.target_id.clone(), chunk.to_vec())
                .await
                .expect("Track IDs should be assigned to chunk_iterated as type TrackID");
        } else {
            event!(Level::DEBUG, "Adding {} tracks to playlist.", chunk.len());
            self.client
                .playlist_add_items(self.target_id.clone(), chunk.to_vec(), None)
                .await
                .expect("Track IDs should be assigned to chunk_iterated as type TrackID");
        }
        false
    }

    /// Appends unique elements from a new collection of `TrackId` to an existing vector of `TrackId`
    /// and returns an extended vector containing all unique elements.
    ///
    /// This function compares elements in the `new` slice against those already present in the `existing`
    /// vector. Elements that are not already in the `existing` vector are appended to it, ensuring that
    /// no duplicates are introduced into the resulting vector.
    ///
    /// # Type Parameters
    /// - `'a`: A lifetime associated with the `TrackId`.
    ///
    /// # Arguments
    /// - `existing`: A reference to a vector of `TrackId<'a>` representing previously stored track IDs.
    /// - `new`: A slice of `TrackId<'a>` containing potentially new track IDs to be added.
    ///
    /// # Returns
    /// A new `Vec<TrackId<'a>>` containing all elements from the `existing` vector, as well as any new,
    /// non-duplicated elements from the `new` slice.
    ///
    /// # Performance
    /// - This function involves filtering and cloning, which may have a performance cost for large inputs.
    ///   Use cautiously with very large datasets.
    ///
    /// # Notes
    /// - The function preserves the order of elements:
    ///   - Elements in the `existing` vector retain their original order.
    ///   - Unique elements from the `new` slice are appended in the order they appear.
    fn append_uniques<'a>(existing: &Vec<TrackId<'a>>, new: &Vec<TrackId<'a>>) -> Vec<TrackId<'a>> {
        let mut extended = existing.to_owned();
        let intersection: Vec<TrackId> = existing
            .iter()
            .filter(|x| new.contains(x))
            .cloned()
            .collect();
        extended.extend(new.iter().filter(|x| !intersection.contains(x)).cloned());
        extended
    }
}
