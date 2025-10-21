use std::collections::HashSet;

use rspotify::clients::BaseClient;
use rspotify::model::{
    AlbumId, ArtistId, FullAlbum, FullArtist, FullTrack, Id, SimplifiedArtist, SimplifiedTrack,
    TrackId,
};
use rspotify::{scopes, AuthCodeSpotify};
use tracing::{error, info, Level};

use crate::traits::apis::Api;

/// A struct representing an album explorer that interacts with the Spotify API.
///
/// The `AlbumXplr` struct provides a way to fetch and manage detailed information
/// about a specific album identified by its `album_id`. This struct is intended to be
/// used with an authenticated Spotify API client to perform operations related to the album.
///
/// # Fields
///
/// * `client` - An instance of `AuthCodeSpotify` which is the authenticated Spotify client
///   used to communicate with the Spotify Web API. This client holds the required
///   authorization details for API access.
///
/// * `album_id` - The unique identifier (`AlbumId`) for the album being explored. This helps
///   identify the album in the Spotify catalog.
///
/// * `full_album` - A `FullAlbum` struct that contains all the detailed information
///   about the album, such as its title, artist(s), tracks, release date,
///   and other metadata.
pub struct AlbumXplr {
    client: AuthCodeSpotify,
    album_id: AlbumId<'static>,
    full_album: FullAlbum,
}

impl Api for AlbumXplr {
    fn select_scopes() -> HashSet<String> {
        scopes!(
            "playlists-read-private",
            "playlists-read-collaborative",
            "playlists-modify-public",
            "playlists-modify-private"
        )
    }
}

impl AlbumXplr {
    /// Asynchronously creates a new instance of `AlbumXplr` by retrieving data for a specified album.
    ///
    /// # Arguments
    /// * `album_id` - An `AlbumId` representing the unique identifier of the album for which the data needs to be fetched.
    ///
    /// # Returns
    /// A new instance of `AlbumXplr` containing the album's data.
    ///
    /// # Workflow
    /// 1. Initializes a tracing span for logging purposes.
    /// 2. Sets up the client for fetching album data by calling the `set_up_client` method.
    /// 3. Uses the client to fetch data for the album specified by the `album_id`.
    ///     - On success, logs that the album data has been successfully retrieved.
    ///     - On failure, logs an error message and panics with the error details.
    /// 4. Returns a new instance of `AlbumXplr` containing the client, album ID, and the full album data.
    ///
    /// # Panics
    /// This method panics if there is any error while attempting to retrieve the album data.
    ///
    /// # Example
    /// ```no_run,ignore
    /// let album_id = AlbumId::new("some-album-id");
    /// let album_explorer = AlbumXplr::new(album_id).await;
    /// ```
    ///
    /// # Logging
    /// * Logs the start of the album creation process at the `INFO` level.
    /// * Logs success when data retrieval is complete.
    /// * Logs an error and panics if retrieval fails.
    ///
    /// # Prerequisites
    /// Ensure that the client setup and the `album` API call succeed to avoid panicking.
    pub async fn new(album_id: AlbumId<'static>) -> Self {
        let span = tracing::span!(Level::INFO, "AlbumXplr.new");
        let _enter = span.enter();

        let client = Self::set_up_client(false, Some(Self::select_scopes())).await;
        let full_album = match client.album(album_id.clone(), Some(Self::market())).await {
            Ok(album) => {
                info!(
                    "Data has been retrieved for the album, '{}'.",
                    album.name.clone()
                );
                album
            }
            Err(error) => {
                error!(album_id = ?album_id.clone(), "Was not able to get data for the requested album");
                panic!("Error: {:?}", error);
            }
        };
        info!(
            "Data has been retrieved for the album, '{}'.",
            full_album.name
        );
        AlbumXplr {
            client,
            album_id,
            full_album,
        }
    }

    /// Returns a clone of the `AlbumId` associated with the current instance.
    ///
    /// # Returns
    /// A cloned `AlbumId` which represents the unique identifier for the album.
    ///
    /// # Examples
    /// ```no_run,ignore
    /// let album = AlbumXplr::new(AlbumId::new("12345")).await;
    /// let id = album.album_id();
    /// assert_eq!(id, AlbumId::new("12345"));
    /// ```
    ///
    /// # Note
    /// The returned `AlbumId` is a clone of the original to ensure the integrity
    /// of the data in the current instance remains unchanged.
    pub fn album_id(&self) -> AlbumId<'_> {
        self.album_id.clone()
    }

    /// Returns a `Vec` of `SimplifiedArtist` instances representing the artists associated with the album.
    ///
    /// The method retrieves all artists from the `full_album` field of the struct and returns them
    /// as a vector. Each artist in the returned vector is a simplified representation.
    ///
    /// # Returns
    /// * `Vec<SimplifiedArtist>` - A vector containing the simplified artist information.
    ///
    /// # Example
    /// ```no_run,ignore
    /// let album = AlbumXplr::new(album_id_obj).await;
    /// let artists = album.simple_artists();
    /// println!("Artists: {:?}", artists);
    /// ```
    pub fn simple_artists(&self) -> Vec<SimplifiedArtist> {
        self.full_album.artists.to_vec()
    }

    /// Retrieves a list of unique artist IDs associated with the current instance.
    ///
    /// # Parameters
    /// - `for_tracks` (`bool`): A boolean flag indicating whether to fetch artist IDs associated with tracks or directly with artists.
    ///   - If `true`, the method will gather artist IDs from the tracks retrieved by `self.simple_tracks()`.
    ///   - If `false`, the method will gather artist IDs from the artists retrieved by `self.simple_artists()`.
    ///
    /// # Returns
    /// A `Vec<ArtistId>` containing the unique artist IDs. The result is constructed using the following logic:
    /// - If `for_tracks` is `true`:
    ///     - Iterates through all tracks from `self.simple_tracks()`.
    ///     - Gathers all unique artist IDs from the track's associated artists.
    /// - If `for_tracks` is `false`:
    ///     - Directly maps and collects IDs from `self.simple_artists()`.
    ///
    /// # Panics
    /// This function will panic if:
    /// - An artist in either `self.simple_tracks()` or `self.simple_artists()` does not have a valid `id`.
    /// - The `.expect("Could not get artist ID")` statement is triggered when attempting to fetch an artist ID.
    ///
    /// # Example
    /// ```no_run,ignore
    /// let explorer = AlbumXplr::new(album_id_obj).await;
    /// let artist_ids_from_tracks = explorer.artist_ids(true); // Fetch artist IDs from tracks
    /// let artist_ids_from_artists = explorer.artist_ids(false); // Fetch artist IDs from artists
    /// ```
    ///
    /// # Note
    /// The method ensures the uniqueness of artist IDs when `for_tracks` is `true` by checking for duplicates before adding them to the resulting vector.
    pub fn artist_ids(&self, for_tracks: bool) -> Vec<ArtistId<'_>> {
        match for_tracks {
            true => {
                let tracks = self.simple_tracks();
                let mut all_ids = Vec::new();
                tracks.iter().for_each(|track| {
                    track.artists.iter().for_each(|artist| {
                        if !all_ids.contains(&artist.id.clone().expect("Could not get artist ID")) {
                            all_ids.push(artist.id.clone().expect("Could not get artist ID"));
                        }
                    });
                });
                all_ids
            }
            false => self
                .simple_artists()
                .iter()
                .map(|artist| artist.id.clone().expect("Could not get artist ID"))
                .collect(),
        }
    }

    /// Asynchronously retrieves detailed information about artists either for the tracks or the album.
    /// This function fetches artist data in chunks and logs relevant information during the process.
    ///
    /// # Arguments
    /// - `for_tracks` - A boolean flag to determine whether to retrieve artists for tracks or for the album in general.
    ///
    /// # Returns
    /// - A `Vec<FullArtist>` containing full details of each artist retrieved.
    ///
    /// # Behavior
    /// - The function gathers all artist IDs related to the current context (either tracks or album) by calling `self.artist_ids(for_tracks)`.
    /// - It processes artist IDs in chunks of 50 to optimize network calls. For each chunk:
    ///   - Successfully fetched artists are logged with their names and added to the result vector.
    ///   - If any error occurs during data retrieval for a chunk, it logs the error and panics with the error details.
    ///
    /// # Logging
    /// - Creates a tracing span with level `INFO` to track the execution of the function.
    /// - Logs successful data retrieval for each artist with their name.
    /// - Logs an error with the chunk of artist IDs that failed to be retrieved.
    ///
    /// # Errors
    /// - If an error occurs while fetching data for a batch of artist IDs, it logs the error and panics.
    ///
    /// # Notes
    /// - The client call to fetch artists (`self.client.artists`) is expected to be an asynchronous function that accepts an iterable of artist IDs.
    /// - This function follows a "fail-fast" approach by panicking if there is an issue with retrieving artist data.
    ///
    /// # Example
    /// ```no_run,ignore
    /// let album_explorer = AlbumXplr::new(album_id_obj).await;
    /// let artists = album_explorer.full_artists(true).await;
    /// for artist in artists {
    ///     println!("Artist Name: {}", artist.name);
    /// }
    /// ```
    pub async fn full_artists(&self, for_tracks: bool) -> Vec<FullArtist> {
        let span = tracing::span!(Level::INFO, "AlbumXplr.full_artists");
        let _enter = span.enter();

        let all_ids = self.artist_ids(for_tracks);
        let mut full_artists = vec![];
        let id_chunks = all_ids.chunks(50);
        for id_chunk in id_chunks {
            match self.client.artists(id_chunk.iter().cloned()).await {
                Ok(artist_batch) => artist_batch.iter().for_each(|full_artist| {
                    info!(
                        "Data has been retrieved for the artist, '{}'.",
                        full_artist.name.clone()
                    );
                    full_artists.push(full_artist.clone());
                }),
                Err(error) => {
                    error!(artist_id = ?id_chunk, "Was not able to get data for the requested artist");
                    panic!("Error: {:?}", error);
                }
            };
        }
        full_artists
    }

    /// Returns a vector of `SimplifiedTrack` objects extracted from the album's track list.
    ///
    /// # Description
    /// This method retrieves the track information from the associated `full_album` object.
    /// Specifically, it accesses the tracks collection (`tracks.items`) stored in the album
    /// and converts it into a `Vec<SimplifiedTrack>`. This provides a simplified representation
    /// of the album's track list.
    ///
    /// # Returns
    /// A vector containing `SimplifiedTrack` instances that represent the individual tracks
    /// in the album.
    ///
    /// # Examples
    /// ```no_run,ignore
    /// let album = AlbumXplr::new(album_id_obj).await;
    /// let simplified_tracks = album.simple_tracks();
    ///
    /// for track in simplified_tracks {
    ///     println!("Track name: {}", track.name);
    /// }
    /// ```
    pub fn simple_tracks(&self) -> Vec<SimplifiedTrack> {
        self.full_album.tracks.items.to_vec()
    }

    /// Retrieves a list of track IDs from the collection of simple tracks.
    ///
    /// This method iterates through all tracks from the `simple_tracks` method, extracts
    /// the `id` from each track, and collects these IDs into a `Vec<TrackId>`. The `expect`
    /// is used to handle cases where a track ID is unexpectedly `None`, causing the program
    /// to panic with an error message.
    ///
    /// # Returns
    ///
    /// A vector containing the `TrackId` of each simple track.
    ///
    /// # Panics
    ///
    /// This method will panic if a track does not have an ID (i.e., if the `id` field is `None`).
    ///
    /// # Example
    ///
    /// ```no_run,ignore
    /// let explorer = AlbumXplr::new(album_id_obj).await;
    /// let track_ids = explorer.track_ids();
    /// println!("{:?}", track_ids);
    /// ```
    pub fn track_ids(&self) -> Vec<TrackId<'_>> {
        self.simple_tracks()
            .iter()
            .map(|track| track.id.clone().expect("Could not get track ID"))
            .collect()
    }

    /// Retrieves the full details of all tracks associated with an album asynchronously.
    ///
    /// This function iterates through a collection of simplified tracks and attempts to
    /// retrieve full track information for each one. The track details are fetched using
    /// the Spotify client and are based on their unique `id`. If a track does not have an
    /// `id`, or if there is an error during the data retrieval for a specific track, the process
    /// will log an appropriate error and terminate with a panic.
    ///
    /// # Returns
    /// A `Vec<FullTrack>` containing the full track details for each track successfully
    /// retrieved.
    ///
    /// # Errors
    /// This function will panic in the cases below:
    ///
    /// - If a track's `id` is `None` and details cannot be retrieved.
    /// - If there is an error during the fetching of a track's full details from the Spotify
    ///   client. The error will be logged, and the process will terminate.
    ///
    /// Log statements in the function inform the user of the progress and errors,
    /// including which tracks had their details successfully fetched.
    ///
    /// # Example
    /// ```no_run,ignore
    /// let album_explorer = AlbumXplr::new(client);
    /// let full_tracks = album_explorer.full_tracks().await;
    /// println!("Retrieved {} full tracks", full_tracks.len());
    /// ```
    ///
    /// # Tracing
    /// A tracing span with `INFO` level is used to track the lifecycle of this function:
    /// - This includes logging success or errors related to individual tracks.
    ///
    /// NOTE: This function assumes the presence of a properly configured tracing setup
    /// and a valid Spotify client instance capable of fetching track information.
    ///
    /// # Panics
    /// - Panics if a track ID is missing or cannot be resolved.
    /// - Panics if the data retrieval for a specific track fails and throws an error.
    pub async fn full_tracks(&self) -> Vec<FullTrack> {
        let span = tracing::span!(Level::INFO, "AlbumXplr.full_tracks");
        let _enter = span.enter();

        let simplified = self.simple_tracks();
        let mut full_tracks = vec![];
        for track in simplified {
            match track.id {
                Some(id) => {
                    let full_track = match self.client.track(id.clone(), Some(Self::market())).await
                    {
                        Ok(full_track) => {
                            info!(
                                "Data has been retrieved for the track, '{}'.",
                                full_track.name.clone()
                            );
                            full_track
                        }
                        Err(error) => {
                            error!(
                                track_id = id.id(),
                                "Was not able to get data for the requested track"
                            );
                            panic!("Error: {:?}", error);
                        }
                    };
                    full_tracks.push(full_track);
                }
                None => {
                    error!(
                        track_id = "None",
                        "Was not able to get data for the requested track"
                    );
                    panic!("Could not get track ID from the simplified track vector.")
                }
            };
        }
        full_tracks
    }

    /// Returns a vector of genres associated with the album.
    ///
    /// This method retrieves a list of genres tied to the album by cloning the
    /// genres data from the `full_album` struct within the instance.
    ///
    /// # Returns
    ///
    /// A `Vec<String>` containing the genres.
    ///
    /// # Example
    ///
    /// ```no_run,ignore
    /// let album = AlbumXplr::new(); // Assuming an Album struct and constructor exist
    /// let genres = album.genres();
    /// println!("{:?}", genres); // Outputs a Vec<String> of genres for the album
    /// ```
    pub fn genres(&self) -> Vec<String> {
        self.full_album.genres.clone()
    }
}

#[cfg(test)]
mod tests {
    use rspotify::model::AlbumId;

    use super::*;

    #[tokio::test]
    async fn test_new() {
        let album_id = AlbumId::from_id("24AElprtNKLkp18RzoddPb").unwrap();
        let album_xplr = AlbumXplr::new(album_id.clone()).await;
        assert_eq!(album_xplr.album_id, album_id);
    }

    #[tokio::test]
    async fn test_genres() {
        let album_id = AlbumId::from_id("24AElprtNKLkp18RzoddPb").unwrap();
        let album_xplr = AlbumXplr::new(album_id.clone()).await;
        let genres = album_xplr.genres();
        println!("{:?}", genres);
        assert_eq!(genres.len(), 0);
    }

    #[tokio::test]
    async fn test_track_methods() {
        let album_id = AlbumId::from_id("24AElprtNKLkp18RzoddPb").unwrap();
        let album_xplr = AlbumXplr::new(album_id.clone()).await;

        let track_ids = album_xplr.track_ids();
        let simple_tracks = album_xplr.simple_tracks();
        let full_tracks = album_xplr.full_tracks().await;
        assert_eq!(track_ids.len(), 13);
        assert_eq!(simple_tracks.len(), 13);
        assert_eq!(full_tracks.len(), 13);
    }

    #[tokio::test]
    async fn test_artist_methods() {
        let album_id = AlbumId::from_id("24AElprtNKLkp18RzoddPb").unwrap();
        let album_xplr = AlbumXplr::new(album_id.clone()).await;

        let simple_artists = album_xplr.simple_artists();
        assert_eq!(simple_artists.len(), 1);
        assert_eq!(simple_artists[0].name, "Kraddy".to_string());

        println!("Working on method, 'artist_ids'");
        let album_collaborators = album_xplr.artist_ids(false);
        let track_collaborators = album_xplr.artist_ids(true);
        assert_eq!(album_collaborators.len(), 1);
        assert_eq!(track_collaborators.len(), 14);

        println!("Working on method, 'full_artists'");
        let full_artists = album_xplr.full_artists(false).await;
        let full_collaborators = album_xplr.full_artists(true).await;
        assert_eq!(full_artists.len(), 1);
        assert_eq!(full_collaborators.len(), 14);
    }
}
