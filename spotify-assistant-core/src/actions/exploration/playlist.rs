use crate::traits::apis::Api;
use futures::StreamExt;
use rspotify::clients::{BaseClient, OAuthClient};
use rspotify::model::{AlbumId, ArtistId, FullAlbum, FullPlaylist, FullTrack, PlayableId, PlayableItem, PlaylistId, SimplifiedAlbum, SimplifiedArtist, TrackId};
use rspotify::{scopes, AuthCodeSpotify};
use std::collections::{HashMap, HashSet};
use tracing::{event, Level};

/// `PlaylistXplr` is a struct that provides functionality for exploring and managing
/// Spotify playlists. It contains information about a specific playlist, its tracks,
/// and whether it contains duplicate tracks.
///
/// Fields:
///
/// * `client` (`AuthCodeSpotify`): An authenticated Spotify client used to access and
///   manage Spotify API resources. Requires proper authentication credentials for operation.
///
/// * `playlist_id` (`PlaylistId<'static>`): The unique identifier for the Spotify playlist
///   that the struct refers to. This ID is used to query and perform actions on the playlist
///   through the Spotify API.
///
/// * `full_playlist` (`FullPlaylist`): A complete representation of the playlist, including
///   metadata, owner details, and any other associated information fetched from Spotify.
///
/// * `tracks` (`Vec<FullTrack>`): A vector containing detailed information about each track
///   in the playlist. Each element provides metadata about a single track, such as its
///   name, artist, album, and duration.
///
/// * `duplicates` (`bool`): A private field (not accessible outside this struct) that
///   indicates whether the playlist contains duplicate tracks. This can be used internally
///   for operations such as filtering or alerting the user of duplicate entries.
pub struct PlaylistXplr {
    pub client: AuthCodeSpotify,
    pub playlist_id: PlaylistId<'static>,
    pub full_playlist: FullPlaylist,
    pub tracks: Vec<FullTrack>,
    duplicates: bool,
}

impl Api for PlaylistXplr {
    fn select_scopes() -> HashSet<String> {
        scopes!(
            "playlist-read-private",
            "playlist-read-collaborative",
            "playlist-modify-public",
            "playlist-modify-private"
        )
    }
}

impl PlaylistXplr {
    /// Asynchronously creates a new instance of `PlaylistXplr`.
    ///
    /// This function sets up a client, retrieves a full playlist metadata,
    /// and instantiates the playlist tracks based on the given `playlist_id`.
    /// Additionally, it establishes if duplicate tracks need to be considered.
    ///
    /// # Arguments
    ///
    /// * `playlist_id` - The unique identifier for the playlist.
    /// * `duplicates` - A boolean flag indicating whether duplicates in the playlist should be handled.
    ///
    /// # Returns
    ///
    /// Returns an instance of `PlaylistXplr` populated with the fetched playlist,
    /// tracks, and client.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let playlist_id = PlaylistId::new("sample_playlist_id");
    /// let playlist_explorer = PlaylistXplr::new(playlist_id, true).await;
    /// ```
    ///
    /// # Logging
    ///
    /// This function creates an `INFO` level tracing span named `"ExplorePlaylist.new"`.
    ///
    /// # Asynchronous Behavior
    ///
    /// This function is `async` and must be called within an asynchronous context.
    pub async fn new(playlist_id: PlaylistId<'static>, duplicates: bool) -> Self {
        let span = tracing::span!(Level::INFO, "ExplorePlaylist.new");
        let _enter = span.enter();

        let client = Self::set_up_client(false, Some(Self::select_scopes())).await;
        let full_playlist = Self::instantiate_playlist(client.clone(), playlist_id.clone()).await;
        let tracks = Self::instantiate_playlist_tracks(client.clone(), playlist_id.clone()).await;
        PlaylistXplr {
            client,
            playlist_id,
            full_playlist,
            tracks,
            duplicates,
        }
    }

    /// Fetches the tracks of a specified Spotify playlist and returns them as a vector of `FullTrack`.
    ///
    /// This asynchronous function leverages the Spotify API client to retrieve items from a playlist, filters out
    /// playable tracks, and returns a list of `FullTrack` objects. The function is equipped with logging
    /// instrumentation using `tracing` to provide detailed information about its execution.
    ///
    /// # Arguments
    ///
    /// * `client` - An authenticated Spotify client (`AuthCodeSpotify`) used to interact with the Spotify API.
    /// * `playlist_id` - The unique identifier for the playlist (`PlaylistId`) whose tracks need to be fetched.
    ///
    /// # Returns
    ///
    /// Returns a `Vec<FullTrack>` containing the full details of each playable track in the playlist.
    ///
    /// # Behavior
    ///
    /// 1. A log span is created for tracing the execution of this function using the `tracing::span!`.
    /// 2. Logging is triggered to indicate the retrieval process of playlist tracks.
    /// 3. The playlist items are fetched via the Spotify client's `playlist_items` method, paginated if necessary.
    /// 4. For each item retrieved:
    ///    - If the item contains a playable track (`PlayableItem::Track`), the track is added to the resulting vector.
    ///    - If the item retrieval fails, an error message is logged, and the function continues with the remaining items.
    /// 5. Any item that is not a track causes a `panic!` to be raised.
    /// 6. A completion log message is recorded when all tracks are processed.
    /// 7. The collected tracks are returned as a `Vec<FullTrack>`.
    ///
    /// # Errors
    ///
    /// * Will panic if a `playlist_item` does not contain a playable track.
    /// * Logs an error but skips affected playlist items if an error occurs during retrieval.
    ///
    /// # Dependencies
    ///
    /// The function relies on the following crates:
    /// - `rspotify` for Spotify API integration.
    /// - `tracing` for structured logging and instrumentation.
    ///
    /// # Notes
    ///
    /// Ensure the Spotify client is properly authenticated and configured before calling this function to avoid
    /// any authentication errors or missing permissions.
    ///
    /// # See Also
    ///
    /// See the [`rspotify` crate](https://docs.rs/rspotify) documentation for more information on managing playlists
    /// and tracks using the Spotify API.
    async fn instantiate_playlist_tracks(client: AuthCodeSpotify, playlist_id: PlaylistId<'_>) -> Vec<FullTrack> {
        let span = tracing::span!(Level::INFO, "ExplorePlaylist.instantiate_playlist_tracks");
        let _enter = span.enter();
        event!(Level::TRACE, "Retrieving playlist tracks");
        let mut playlist_items = client.playlist_items(playlist_id, None, Some(Self::market()));
        let mut tracks = Vec::new();
        while let Some(items) = playlist_items.next().await {
            match items {
                Ok(item) => {
                    match item.track {
                        Some(PlayableItem::Track(track)) => tracks.push(track),
                        _ => { panic!("Could not get track from playlists") }
                    };
                }
                Err(err) => {
                    event!(Level::ERROR, "Could not retrieve playlist items: {:?}", err);
                    continue;
                }
            }
        }
        event!(Level::TRACE, "Playlist tracks have been retrieved.");
        tracks
    }

    /// Retrieves and instantiates a Spotify playlist using the provided credentials and playlist ID.
    ///
    /// This asynchronous function fetches the playlist data from the Spotify API using the
    /// `AuthCodeSpotify` client and the given `PlaylistId`. It logs the progress and any issues
    /// encountered during the process. If successful, it returns the full playlist data as a
    /// `FullPlaylist`. In case of failure, it will log the error and terminate the program with a panic.
    ///
    /// # Arguments
    ///
    /// * `client` - An authenticated instance of `AuthCodeSpotify` used to interact with Spotify's API.
    /// * `playlist_id` - The unique identifier of the playlist to be retrieved. Expected to be of type `PlaylistId`.
    ///
    /// # Returns
    ///
    /// Returns an instance of `FullPlaylist` containing the detailed data of the requested playlist.
    ///
    /// # Panics
    ///
    /// This function will panic if the playlist could not be retrieved due to any error. The error
    /// details are logged with the `Level::ERROR` log level.
    ///
    /// # Tracing
    ///
    /// The function utilizes the `tracing` crate to log the following information:
    /// - INFO level spans for the start of the playlist instantiation process.
    /// - TRACE level events to indicate the success of playlist data retrieval.
    /// - ERROR level events to log any errors encountered during the retrieval process.
    async fn instantiate_playlist(client: AuthCodeSpotify, playlist_id: PlaylistId<'_>) -> FullPlaylist {
        let span = tracing::span!(Level::INFO, "ExplorePlaylist.instantiate_playlist");
        let _enter = span.enter();
        event!(Level::TRACE, "Retrieving playlist data");
        match client.playlist(playlist_id, None, Some(Self::market())).await {
            Ok(pl) => {
                event!(Level::TRACE, "Playlist data has been retrieved.");
                pl
            },
            Err(err) => {
                event!(Level::ERROR, "Could not retrieve playlist: {:?}", err);
                panic!("Could not retrieve playlist: {err:?}");
            }
        }
    }

    /// Returns a vector of `FullTrack` instances cloned from the `tracks` field.
    ///
    /// # Returns
    /// A `Vec<FullTrack>` containing all tracks stored in the `tracks` field.
    ///
    /// # Examples
    /// ```no_run,ignore
    /// let cloned_tracks = explorer.tracks();
    /// assert_eq!(cloned_tracks.len(), 2);
    /// ```
    ///
    /// Note: The returned vector is a clone of the original, ensuring the original
    /// `tracks` field remains unmodified.
    pub fn tracks(&self) -> Vec<FullTrack> {
        self.tracks.clone()
    }

    /// Retrieves a collection of albums associated with the tracks.
    ///
    /// This method extracts all the albums from the tracks and returns them as a vector of `SimplifiedAlbum`.
    /// Each album is cloned from its corresponding track.
    ///
    /// # Returns
    ///
    /// A `Vec<SimplifiedAlbum>` containing the albums associated with the tracks. If the tracks list is empty,
    /// an empty vector is returned.
    ///
    /// # Example
    ///
    /// ```no_run,ignore
    /// let albums = explorer.albums();
    /// for album in albums {
    ///     println!("{:?}", album.name);
    /// }
    /// ```
    ///
    /// # Notes
    ///
    /// - This method assumes that each track contains an associated `SimplifiedAlbum`.
    /// - The resulting vector may contain duplicate albums if multiple tracks belong to the same album.
    ///
    /// # Performance
    ///
    /// - The method iterates over all tracks and applies a mapping operation, which may impact performance
    ///   on large collections. The cloning operation may also introduce additional overhead when working with a large number of albums.
    pub fn albums(&self) -> Vec<SimplifiedAlbum> {
        self.tracks()
            .iter()
            .map(|track| track.album.clone())
            .collect::<Vec<SimplifiedAlbum>>()
    }

    /// Retrieves a list of all the artists from the tracks in the playlist.
    ///
    /// This method iterates over all the tracks in a playlist, extracts the artists associated with each track,
    /// and compiles them into a `Vec<SimplifiedArtist>`. Useful for obtaining a consolidated list of all artists
    /// featured in the playlist.
    ///
    /// The function uses tracing instrumentation to log the operation process:
    /// - Logs at the `INFO` level when the function is called, specifying the playlist name.
    /// - Logs at the `DEBUG` level for each track and its associated artist(s) during iteration.
    ///
    /// Returns:
    /// - A `Vec<SimplifiedArtist>` containing all the artists in the playlist, where duplicates are not removed.
    ///
    /// # Example
    /// ```no_run,ignore
    /// let playlist = ExplorePlaylist::new(...); // Assuming an appropriately initialized playlist.
    /// let artists = playlist.artists();
    /// for artist in artists {
    ///     println!("Artist: {}", artist.name);
    /// }
    /// ```
    ///
    /// Notes:
    /// - This function performs a deep clone of artist and track data, which could be costly for large playlists.
    /// - Consider further filtering or deduplication outside this function if required.
    ///
    /// Tracing Levels:
    /// - **INFO** - Indicates the playlist being processed.
    /// - **DEBUG** - Provides detailed insights into each track and artist during extraction.
    pub fn artists(&self) -> Vec<SimplifiedArtist> {
        let span = tracing::span!(Level::INFO, "ExplorePlaylist.artists");
        let _enter = span.enter();
        let pl_name = self.full_playlist.name.clone();
        event!(
            Level::INFO,
            "Retrieving artists from the '{pl_name}' playlists"
        );

        let mut artists_vec = Vec::new();
        self.tracks().iter().for_each(|track| {
            event!(Level::DEBUG, "Track: {:?}", track.clone().name);
            track.artists.iter().for_each(|artist| {
                event!(Level::DEBUG, "\tArtist: {:?}", artist.clone().name);
                artists_vec.push(artist.clone());
            });
        });
        artists_vec
    }

    /// Generates a mapping of track names to their corresponding artists for the playlist.
    ///
    /// This function iterates over all the tracks in the playlist, retrieving the artists associated
    /// with each track and organizing them into a `HashMap` where the key is the track name (as a `String`)
    /// and the value is a vector (`Vec<SimplifiedArtist>`) of associated artists.
    ///
    /// Logging is implemented at various levels to track the function's execution:
    /// - `INFO`: Logs the start of the operation with the playlist name.
    /// - `TRACE`: Logs the number of artists added for each specific track.
    /// - `DEBUG`: Logs the updated state of the `HashMap` after each entry is added, including
    ///   its length, the current key (track name), and the length of the associated list of artists.
    ///
    /// # Returns
    /// A `HashMap` associating every track name in the playlist with a `Vec` of its respective `SimplifiedArtist` objects.
    ///
    /// # Example
    /// ```no_run,ignore
    /// let artists_map = explorer.artists_by_track();
    /// for (track_name, artists) in artists_map {
    ///     println!("Track: {}", track_name);
    ///     for artist in artists {
    ///         println!(" - {}", artist.name);
    ///     }
    /// }
    /// ```
    ///
    /// # Notes
    /// - `self.tracks()` is expected to return a collection of tracks within the playlist, where
    ///   each track provides a `.name` property and an `.artists` property.
    /// - Associated logging spans can be useful for debugging and tracing this function when it's run
    ///   in environments with enabled `tracing`.
    pub fn artists_by_track(&self) -> HashMap<String, Vec<SimplifiedArtist>> {
        let span = tracing::span!(Level::INFO, "ExplorePlaylist.artists_by_track");
        let _enter = span.enter();
        let pl_name = self.full_playlist.name.clone();
        event!(
            Level::INFO,
            "Retrieving artists from the '{pl_name}' playlists"
        );
        let mut artists_map = HashMap::new();
        self.tracks().iter().for_each(|track| {
            let track_name = track.name.clone();
            let artists = track.artists.clone();
            event!(Level::TRACE, "Adding {:?} artists from track {:?}", artists.len(), track_name);
            artists_map.insert(track_name.clone(), artists.clone());
            event!(Level::DEBUG,
                "Hashmap length: {:?} | Key: {:?} | Length of value {:?}",
                artists_map.len(), track_name, artists.len()
            );
        });
        artists_map
    }

    /// Retrieves a list of unique album IDs associated with the tracks in the collection.
    ///
    /// This method iterates through all the tracks, extracts their associated album IDs,
    /// and returns them as a vector. If a track does not contain an album ID, the method
    /// will panic with the error message: "Could not get album id from track".
    ///
    /// # Returns
    ///
    /// A `Vec<AlbumId>` containing the album IDs of all tracks. Note that the IDs may
    /// contain duplicates if multiple tracks share the same album.
    ///
    /// # Panics
    ///
    /// The method will panic if the `id` field of an album is `None`. This indicates
    /// there is a missing or invalid album ID within a track.
    ///
    /// # Example
    ///
    /// ```no_run,ignore
    /// let album_ids = explorer.album_ids();
    /// for id in album_ids {
    ///     println!("{:?}", id);
    /// }
    /// ```
    pub fn album_ids(&self) -> Vec<AlbumId<'_>> {
        self.tracks()
            .iter()
            .map(|track| {
                track
                    .album
                    .id
                    .clone()
                    .expect("Could not get album id from track")
            })
            .collect::<Vec<AlbumId>>()
    }

    /// Asynchronously retrieves and expands a list of artist IDs based on the albums associated
    /// with the current playlist.
    ///
    /// # Details
    /// This method iterates through chunks of album IDs associated with the playlist and retrieves
    /// detailed album information using the `client`. For each album, the method extracts the
    /// artist IDs of all associated artists. Optionally, it removes duplicate artist IDs if
    /// the `duplicates` field in the struct is set to `false`.
    ///
    /// The method uses tracing spans and events for logging at both `INFO` and `DEBUG` levels,
    /// providing insights into processing steps and intermediate album chunks.
    ///
    /// # Returns
    /// * `Vec<ArtistId>` - A vector of unique artist IDs extracted from all relevant albums.
    ///
    /// # Panics
    /// * Panics if:
    ///   - Any of the asynchronous calls to the `client.albums` method fails.
    ///   - An album does not contain a valid artist ID.
    ///
    /// # Logging
    /// - An `INFO` level tracing span is created to track the entire operation.
    /// - A `DEBUG` level event is logged with details of the current album chunk being processed.
    ///
    /// # Notes
    /// - The albums are processed in chunks of 20 for optimized API calls.
    /// - Removes duplicate artist IDs from the resulting vector using `clean_duplicate_id_vector`
    ///   if the `duplicates` field is set to `false`.
    ///
    /// # Example
    /// ```no_run,ignore
    /// let expanded_artist_ids = playlist.artist_ids_from_track_albums().await;
    /// println!("Expanded artist IDs: {:?}", expanded_artist_ids);
    /// ```
    pub async fn artist_ids_from_track_albums(&self) -> Vec<ArtistId<'_>> {
        let span = tracing::span!(Level::INFO, "ExplorePlaylist.artist_ids_from_track_albums");
        let _enter = span.enter();

        let mut artist_ids = Vec::new();
        for album_chunk in self.album_ids().chunks(20) {
            event!(
                Level::DEBUG,
                "Current album chunk: {:?}",
                album_chunk.to_vec()
            );
            let albums = self
                .client
                .albums(album_chunk.to_vec(), Some(Self::market()))
                .await
                .expect("Could not retrieve albums");
            albums.iter().for_each(|album| {
                artist_ids.extend(
                    album
                        .artists
                        .iter()
                        .map(|artist| {
                            artist
                                .id
                                .clone()
                                .expect("Could not get artist id from album")
                        })
                        .collect::<Vec<ArtistId>>(),
                );
            });
        }
        if !self.duplicates {
            artist_ids = Self::clean_duplicate_id_vector(artist_ids);
        }
        artist_ids
    }

    /// Collects and returns a vector containing the IDs of all artists associated
    /// with the tracks in the playlist.
    ///
    /// This method iterates over the tracks in the playlist, retrieves the list of
    /// artists for each track, and extracts the `id` field of each artist. If an
    /// artist does not have an ID, it will panic with the message:
    /// "Could not get artist id from track".
    ///
    /// A tracing span is created for logging purposes with the span name
    /// "ExplorePlaylist.artist_ids_from_tracks".
    ///
    /// # Panics
    /// This function will panic if any artist in the tracks does not have an `id`.
    ///
    /// # Returns
    /// A `Vec<ArtistId>` containing all the artist IDs from the playlist's tracks.
    ///
    /// # Example
    /// ```no_run,ignore
    /// // Assuming `playlist` is an instance of ExplorePlaylist.
    /// let artist_ids = playlist.artist_ids_from_tracks();
    /// for artist_id in artist_ids {
    ///     println!("{}", artist_id);
    /// }
    /// ```
    pub fn artist_ids_from_tracks(&self) -> Vec<ArtistId<'_>> {
        let span = tracing::span!(Level::INFO, "ExplorePlaylist.artist_ids_from_tracks");
        let _enter = span.enter();
        let mut artist_ids = self.tracks()
                                 .iter()
                                 .flat_map(|track| {
                track
                    .artists
                    .iter()
                    .flat_map(|artist| {
                        Some(
                            artist
                                .id
                                .clone()
                                .expect("Could not get artist id from track"),
                        )
                    })
                    .collect::<Vec<ArtistId>>()
            })
                                 .collect::<Vec<ArtistId>>();

        if !self.duplicates {
            artist_ids = Self::clean_duplicate_id_vector(artist_ids);
        };
        artist_ids
    }

    /// Retrieves a collection of expanded track IDs by fetching tracks from albums.
    ///
    /// This asynchronous method fetches track IDs from multiple albums associated with the playlist.
    /// It retrieves album data in chunks (to limit the number of requests being sent) and extracts
    /// the track IDs from the album tracks. Duplicate track IDs can be optionally removed based on
    /// the `duplicates` property of the struct.
    ///
    /// # Returns
    /// A `Vec<TrackId>` containing the expanded list of track IDs.
    ///
    /// # Behavior
    /// - The method processes albums in chunks to avoid overloading the API.
    /// - Uses the async `client.albums` method to fetch album details for each chunk.
    /// - The `duplicates` flag controls whether duplicate track IDs are removed or not.
    /// - Utilizes tracing for logging events during execution.
    ///
    /// # Logging
    /// - Creates a log span with the name `ExplorePlaylist.track_ids_expanded`.
    /// - Logs the current batch of album chunks being processed at the debug level.
    /// - Ensures errors such as failure to retrieve album data or track IDs are addressed with appropriate panics.
    ///
    /// # Errors
    /// - Panics if album fetch fails using the `client.albums` method.
    /// - Panics if a track ID cannot be retrieved from a track within an album.
    ///
    /// # Examples
    /// ```no_run,ignore
    /// // Assuming an instance of ExplorePlaylist - `playlist`
    /// let track_ids = playlist.track_ids_expanded().await;
    /// ```
    ///
    /// # Notes
    /// - This method relies on the `self.album_ids()` method to retrieve the list of album IDs.
    /// - The `clean_duplicate_id_vector` method is used to remove duplicate IDs if the `duplicates` flag is `false`.
    pub async fn track_ids_expanded(&self) -> Vec<TrackId<'_>> {
        let span = tracing::span!(Level::INFO, "ExplorePlaylist.track_ids_expanded");
        let _enter = span.enter();

        let mut track_ids: Vec<TrackId> = Vec::new();
        for album_chunk in self.album_ids().chunks(20) {
            event!(
                Level::DEBUG,
                "Current album chunk: {:?}",
                album_chunk.to_vec()
            );
            let albums: Vec<FullAlbum> = self
                .client
                .albums(album_chunk.to_vec(), Some(Self::market()))
                .await
                .expect("Could not retrieve albums");
            albums.iter().for_each(|album| {
                track_ids.extend(
                    album
                        .tracks
                        .items
                        .iter()
                        .map(|track| track.id.clone().expect("Could not get track id from album"))
                        .collect::<Vec<TrackId>>(),
                );
            });
        }
        if !self.duplicates {
            track_ids = Self::clean_duplicate_id_vector(track_ids);
        }
        track_ids
    }

    /// Asynchronously finds and categorizes the user's liked and not liked songs.
    ///
    /// This function interacts with the Spotify API to determine which songs from the user's
    /// playlist are marked as "liked" (saved to the user's Spotify library).
    ///
    /// # Functionality
    /// - The function processes the user's tracks in batches to determine if each track
    ///   is liked by the user.
    /// - Tracks are categorized into two groups: `liked` and `not_liked`.
    /// - The categorized tracks are returned in a `HashMap` with keys `"liked"` and
    ///   `"not_liked"` containing corresponding song lists.
    ///
    /// # Implementation Details
    /// - Uses [tracing](https://docs.rs/tracing) for structured logging at various log levels:
    ///   - `INFO`: Marks the entry into the function.
    ///   - `DEBUG`: Logs details about batch processing and sizes of track collections.
    ///   - `ERROR`: Logs any errors encountered during API calls.
    /// - Processes tracks in chunks using a configurable `batch_size` to limit the number of
    ///   requests sent to the Spotify API at one time.
    /// - On encountering an error during API interaction, the function logs the error
    ///   and panics.
    ///
    /// # Returns
    /// A `HashMap` with the following keys:
    /// - `"liked"`: A `Vec` of `FullTrack` objects representing liked songs.
    /// - `"not_liked"`: A `Vec` of `FullTrack` objects representing not liked songs.
    ///
    /// # Panics
    /// This function will panic if it encounters an error while fetching track metadata
    /// from the Spotify API. Ensure proper error handling mechanisms are implemented
    /// in higher-level logic to handle such cases resiliently.
    ///
    /// # Usage Example
    /// ```no_run ignore
    /// use spotify_api_library::FullTrack; // Replace with actual FullTrack import
    /// use std::collections::HashMap;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let playlist = PlaylistXplr::new(...); // Initialize your `ExplorePlaylist` instance
    ///
    ///     let liked_songs_map: HashMap<&str, Vec<FullTrack>> = playlist.find_liked_songs().await;
    ///
    ///     if let Some(liked_tracks) = liked_songs_map.get("liked") {
    ///         println!("Liked songs: {:?}", liked_tracks);
    ///     }
    ///
    ///     if let Some(not_liked_tracks) = liked_songs_map.get("not_liked") {
    ///         println!("Not liked songs: {:?}", not_liked_tracks);
    ///     }
    /// }
    /// ```
    ///
    /// # Dependencies
    /// - `HashMap` from the standard library is used to categorize tracks.
    /// - External dependencies include Spotify's API client and the `tracing` logging crate.
    ///
    /// # Notes
    /// - This function assumes that `track_ids()` and `tracks()` are implemented and return
    ///   the necessary identifiers and data for the playlist.
    /// - Ensure the Spotify client is properly authenticated and initialized before calling
    ///   this function.
    ///
    /// # Parameters
    /// - `self`: This function is defined as a method and is invoked on an instance of
    ///   the containing struct, which presumably provides access to the `self.client`,
    ///   `self.track_ids()`, and `self.tracks()` methods.
    pub async fn find_liked_songs(&self) -> HashMap<&str, Vec<FullTrack>> {
        let span = tracing::span!(Level::INFO, "ExplorePlaylist.find_liked_songs");
        let _enter = span.enter();
        let mut liked = Vec::new();
        let mut not_liked = Vec::new();
        let batch_size = 50;

        let mut liked_songz = Vec::new();
        for chunk in self.track_ids().chunks(batch_size) {
            let chunk_iter = chunk.iter().cloned();
            match self.client.current_user_saved_tracks_contains(chunk_iter).await {
                Ok(liked) => {
                    event!(Level::DEBUG, "batch size: {:?}", liked.len());
                    liked.iter().for_each(|liked| { liked_songz.push(liked.to_owned()) });
                }
                Err(err) => {
                    event!(Level::ERROR, "Could not retrieve liked songs: {:?}", err);
                    panic!();
                }
            }
        }
        event!(Level::DEBUG, "Liked songs length: {:?}", liked_songz.len());
        event!(Level::DEBUG, "Track ids length: {:?}", self.track_ids().len());
        let zipped = self.tracks().into_iter().zip(liked_songz.into_iter()).collect::<Vec<(FullTrack, bool)>>();
        zipped.into_iter().for_each(|(track, is_liked)| {
            if is_liked {
                liked.push(track.to_owned());
            } else {
                not_liked.push(track.to_owned());
            }
        });
        HashMap::from([("liked", liked), ("not_liked", not_liked)])
    }

    /// Asynchronously retrieves and maps artists associated with albums in the context of the `ExplorePlaylist` object.
    ///
    /// This method performs the following steps:
    /// 1. Iterates over chunks of album IDs and retrieves detailed album data from the client API.
    /// 2. Extracts artist information for each album and filters out any duplicate artists if the `duplicates` flag is set to `false`.
    /// 3. Aggregates the list of unique artists for each album into a `HashMap` where the key is the album name
    ///    (a `String`), and the value is a `Vec` of the artists (represented as `SimplifiedArtist`).
    ///
    /// The method includes logging at various trace levels to track progress and potential issues:
    /// - Logs the current album chunk being processed at the `DEBUG` level.
    /// - Logs any errors encountered when retrieving albums at the `ERROR` level.
    /// - Logs detailed debugging information related to the lengths of raw and cleaned artist lists.
    ///
    /// # Returns
    /// A `HashMap` where:
    /// - The keys are album names (`String`).
    /// - The values are vectors of `SimplifiedArtist` objects representing the unique artists associated with the album.
    ///
    /// # Panics
    /// This function may panic under the following conditions:
    /// - If an album retrieval request fails (e.g., due to a network error), a panic is triggered with the error message.
    /// - If a `SimplifiedArtist` instance lacks an `id`, a panic is triggered as the artist ID is critical for filtering duplicates.
    ///
    /// # Considerations
    /// - The method processes album chunks in sizes of 20 to comply with API limitations or optimize batch processing.
    /// - Filtering for duplicate artists is controlled by the `duplicates` flag in the `ExplorePlaylist` struct.
    ///   When this flag is `false`, duplicate artist IDs are cleaned out.
    ///
    /// # Errors
    /// Although not returned explicitly as a `Result`, errors related to API calls and data retrieval are logged
    /// and would result in a panic, halting further execution.
    ///
    /// # Example Usage
    /// ```no_run ignore
    /// async fn example(playlist: &PlaylistXplr) {
    ///     let artists_map = playlist.artists_by_album().await;
    ///     for (album, artists) in artists_map {
    ///         println!("Album: {}", album);
    ///         for artist in artists {
    ///             println!(" - Artist: {}", artist.name);
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// # Dependencies
    /// The method depends on the following:
    /// - `tokio` or another async runtime for asynchronous execution.
    /// - A `self.client` field that provides an API interface for fetching album details.
    /// - A `SimplifiedArtist` and `FullAlbum` type representing artist and album information retrieved from the API.
    /// - A logging framework compatible with `tracing` for structured logging and instrumentation.
    ///
    /// # Notes
    /// - Ensure the `self.album_ids()` method exists and correctly provides the IDs of albums that need processing.
    /// - Verify that the client used in `self.client.albums` is configured properly, including its market and proper handling of API rate limits.
    pub async fn artists_by_album(&self) -> HashMap<String, Vec<SimplifiedArtist>> {
        let span = tracing::span!(Level::INFO, "ExplorePlaylist.artists_by_album");
        let _enter = span.enter();

        let mut artists_map = HashMap::new();
        for album_chunk in self.album_ids().chunks(20) {
            event!(
                Level::DEBUG,
                "Current album chunk: {:?}",
                album_chunk.to_vec()
            );
            let albums: Vec<FullAlbum> = match self.client.albums(album_chunk.to_vec(), Some(Self::market())).await {
                Ok(albums) => albums,
                Err(err) => {
                    event!(Level::ERROR, "Could not retrieve albums: {:?}", err);
                    panic!("Could not retrieve albums: {err:?}");
                }
            };
            albums.iter().for_each(|album| {
                let album_artists = album.tracks.items.iter().flat_map(|track| {
                    track.artists.to_vec()
                }).collect::<Vec<SimplifiedArtist>>();
                let artist_ids = album_artists.iter().map(|artist| {
                    match artist.id.clone() {
                        Some(id) => id,
                        None => panic!("Could not get artist id from album")
                    }
                }).collect::<Vec<ArtistId>>();
                let cleaned_artist_ids = if !self.duplicates {
                    Self::clean_duplicate_id_vector(artist_ids.clone())
                } else {
                    artist_ids.clone()
                };
                let mut pushed: Vec<ArtistId> = Vec::new();
                let cleaned_artists = album_artists.iter().filter_map(|artist| {
                    let artist_id_to_filter = match artist.id.clone() {
                        Some(id) => id,
                        None => panic!("Could not get artist id from album")
                    };
                    let contains = cleaned_artist_ids.contains(&artist_id_to_filter);
                    let is_duplicate = pushed.contains(&artist_id_to_filter);
                    if contains && !is_duplicate {
                        pushed.push(artist_id_to_filter);
                        Some(artist.to_owned())
                    } else {
                        None
                    }
                }).collect::<Vec<SimplifiedArtist>>();
                event!(
                    Level::DEBUG,
                    "raw_id_length: {:?} | cleaned_length: {:?} | artist_raw_length: {:?} | artist_cleaned_length: {:?}",
                    artist_ids.len(),
                    cleaned_artist_ids.len(),
                    album_artists.clone().len(),
                    cleaned_artists.len()
                );
                artists_map.insert(album.name.clone(), cleaned_artists);
            });
        }
        artists_map
    }

    /// Retrieves a list of track IDs from the playlist.
    ///
    /// # Description
    /// This function extracts the unique identifiers (`TrackId`) of all tracks
    /// present in the playlist. It logs the operation's progress and the number
    /// of tracks in the playlist for tracing and debugging purposes. If any track
    /// in the playlist does not have an associated `TrackId`, the function will
    /// panic.
    ///
    /// # Returns
    /// A `Vec<TrackId>` containing the IDs of all tracks in the playlist.
    ///
    /// # Panics
    /// This function will panic if any track in the playlist does not contain
    /// an associated `TrackId`.
    ///
    /// # Logging
    /// - Logs an `INFO`-level message that the track IDs are being retrieved
    ///   along with the count of tracks in the playlist.
    /// - Uses a tracing span named `"ExplorePlaylist.track_ids_original"` to encapsulate the operation.
    ///
    /// # Examples
    /// ```no_run ignore
    /// // Assuming `playlist` is an instance of a struct implementing this function:
    /// let track_ids = playlist.track_ids();
    /// println!("Retrieved track IDs: {:?}", track_ids);
    /// ```
    ///
    /// # Notes
    /// Ensure that all tracks in the playlist have a valid ID before calling this function,
    /// as the presence of a `None` value for a track's ID will cause a panic.
    pub fn track_ids(&self) -> Vec<TrackId<'_>> {
        let span = tracing::span!(Level::INFO, "ExplorePlaylist.track_ids_original");
        let _enter = span.enter();
        event!(Level::INFO, "Retrieving track ids from the playlist. Track count: {:?}", self.tracks().len());
        self.tracks()
            .iter()
            .map(|track| match track.id.clone() {
                Some(id) => id,
                None => panic!("Could not get track id from track")
            })
            .collect::<Vec<TrackId>>()
    }
    pub fn playable_ids(&self) -> Vec<PlayableId<'_>> {
        let span = tracing::span!(Level::INFO, "ExplorePlaylist.playable_ids");
        let _enter = span.enter();

        let owned_tracks = self.tracks.iter()
                               .filter_map(|track| track.id.clone())
                               .map(PlayableId::Track)
                               .collect::<Vec<_>>();

        if owned_tracks.is_empty() {
            event!(Level::ERROR, "Could not collect track IDs from reference playlist");
            panic!("Could not collect track IDs from reference playlist");
        } else {
            owned_tracks
        }
    }
}
