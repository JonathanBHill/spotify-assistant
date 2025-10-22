use std::collections::HashSet;

use dotenv::dotenv;
use rspotify::model::{FullPlaylist, FullTrack, PlayableItem};
use rspotify::{scopes, AuthCodeSpotify};

use crate::traits::apis::Api;

/// A struct that represents a comparison of playlists.
///
/// This struct is designed to hold information about a specific playlist
/// and its associated tracks, providing tools for comparing and analyzing
/// playlists. It contains the Spotify client, the playlist details, and
/// a collection of stored tracks.
///
/// # Fields
/// - `client`: An instance of `AuthCodeSpotify` representing the authenticated
///   Spotify client to interact with the Spotify Web API.
/// - `playlist`: A `FullPlaylist` object that represents the main playlist
///   being analyzed.
/// - `stored_tracks`: A vector of `FullTrack` objects that represents a
///   collection of stored tracks for playback or comparison purposes.
///
/// # Attributes
/// - `#[allow(dead_code)]`: This attribute prevents compiler warnings for unused
///   code, acknowledging that this struct or its fields might not currently
///   be in use but are intentionally kept for future functionality.
#[allow(dead_code)]
pub struct ComparePlaylists {
    client: AuthCodeSpotify,
    playlist: FullPlaylist,
    pub stored_tracks: Vec<FullTrack>,
}

impl PartialEq for ComparePlaylists {
    fn eq(&self, other: &Self) -> bool {
        self.playlist.id == other.playlist.id
    }
}
impl Api for ComparePlaylists {
    fn select_scopes() -> HashSet<String> {
        scopes!(
            "playlists-read-private",
            "playlists-read-collaborative",
            "playlists-modify-public",
            "playlists-modify-private"
        )
    }
}
#[allow(dead_code)]
impl ComparePlaylists {
    /// Creates a new `ComparePlaylists` instance with the specified playlist.
    ///
    /// This asynchronous function constructs a `ComparePlaylists` object by performing the following steps:
    /// 1. Loads environment variables using the `dotenv` crate.
    /// 2. Sets up an asynchronous Spotify client with the required scopes using the `set_up_client` method.
    /// 3. Extracts the tracks from the given playlist and processes them into a collection of `FullTrack` items.
    ///    - Each track is retrieved from the `playlist`'s `tracks.items` collection.
    ///    - If the item is a `PlayableItem` of type `Track`, it is processed and added to the collection.
    ///    - If the item is a `PlayableItem` of type `Episode`, or if the item is `None`, the function logs an error and terminates the process with a panic.
    ///
    /// # Arguments
    /// - `playlist`: A `FullPlaylist` object containing the complete playlist information, including its tracks.
    ///
    /// # Returns
    /// A new instance of `ComparePlaylists` containing:
    /// - A Spotify client object for API interactions.
    /// - The provided `playlist` object.
    /// - A vector of processed `FullTrack` objects extracted from the playlist.
    ///
    /// # Panics
    /// - The function panics if a track is missing (`None`) or is an unsupported `PlayableItem` type (e.g., `Episode`).
    ///
    /// # Example
    /// ```ignore
    /// let playlist = fetch_full_playlist(); // Assume this fetches a FullPlaylist object
    /// let compare_playlists = ComparePlaylists::new(playlist).await;
    /// ```
    ///
    /// Note: The function relies on environment variables, so ensure that the `.env` file is properly configured before invocation.
    pub async fn new(playlist: FullPlaylist) -> Self {
        dotenv().ok();
        let client = Self::set_up_client(false, Some(Self::select_scopes())).await;
        let tracks = playlist
            .tracks
            .items
            .iter()
            .map(|track| match track.track.clone() {
                Some(track) => match track {
                    PlayableItem::Track(track) => track,
                    PlayableItem::Episode(episode) => {
                        eprintln!(
                            "Error: Incorrect item returned. An episode was provided: {:?}",
                            episode.name
                        );
                        panic!("Could not get full track")
                    }
                    _ => {
                        panic!("Could not get full track")
                    }
                },
                None => panic!("Could not get track"),
            })
            .collect::<Vec<FullTrack>>();
        ComparePlaylists {
            client,
            playlist: playlist.clone(),
            stored_tracks: tracks,
        }
    }

    /// Compares the total number of tracks in two playlists and determines if their lengths are equal.
    ///
    /// # Parameters
    /// - `&self`: A reference to the current instance of the object containing the playlist.
    /// - `other`: A reference to another instance of the same object type, which also contains a playlist.
    ///
    /// # Returns
    /// - `bool`: Returns `true` if the total number of tracks in both playlists are equal; otherwise, `false`.
    ///
    /// # Side Effects
    /// - Prints whether the playlist lengths are equal to the standard output.
    /// - Prints the total number of tracks in both playlists to the standard output.
    ///
    /// # Example
    /// ```ignore
    /// let playlist1: ComparePlaylists = fetch_playist(); // Assume this fetches a FullPlaylist object
    /// let playlist2: ComparePlaylists = fetch_playist(); // Assume this fetches another FullPlaylist object
    ///
    /// if playlist1.eq_len(&playlist2) {
    ///     println!("Playlists have the same number of tracks.");
    /// } else {
    ///     println!("Playlists have different lengths.");
    /// }
    /// ```
    ///
    /// # Note
    /// This function assumes that the `playlist.tracks.total` field provides the accurate total number of tracks for both playlist objects.
    pub fn eq_len(&self, other: &Self) -> bool {
        println!(
            "Playlist lengths are equal: {:?}",
            self.playlist.tracks.total == other.playlist.tracks.total
        );
        println!("Playlist 1 length: {:?}", self.playlist.tracks.total);
        println!("Playlist 2 length: {:?}", other.playlist.tracks.total);
        self.playlist.tracks.total == other.playlist.tracks.total
    }

    /// Compares the metadata of two playlists and checks whether they are equal.
    ///
    /// This method evaluates the equality of the `playlist.id` from the current
    /// object and the other playlist object. Depending on the result, it prints
    /// details of both playlists for diagnostic purposes, including the playlist
    /// IDs, names, and owner IDs.
    ///
    /// # Arguments
    /// - `other` - A reference to another instance of the same struct type that
    ///   contains playlist metadata to be compared.
    ///
    /// # Returns
    /// - `true` if the playlists' `id` fields are equal.
    /// - `false` if the playlists' `id` fields are not equal.
    ///
    /// # Behavior
    /// - If the playlists are equal (same `id`), it logs the equality details and other
    ///   metadata about the playlist (ID, name, and owner ID).
    /// - If the playlists are not equal, it logs the inequality details and displays the
    ///   metadata differences between the two playlists (ID, name, and owner ID).
    ///
    /// # Example
    /// ```ignore
    /// let playlist1: ComparePlaylists = fetch_playist(); // Assume this fetches a FullPlaylist object
    /// let playlist2: ComparePlaylists = fetch_playist(); // Assume this fetches another FullPlaylist object
    ///
    /// if playlist1.comp_metadata(&playlist2) {
    ///     println!("The playlists match!");
    /// } else {
    ///     println!("The playlists do not match.");
    /// }
    /// ```
    ///
    /// # Prints
    /// - Prints diagnostic logs to the console detailing the comparison results.
    pub fn comp_metadata(&self, other: &Self) -> bool {
        let eq_id = self.playlist.id == other.playlist.id;
        if eq_id {
            println!("Playlists are equal.");
            println!("Playlist ID: {:?}", self.playlist.id);
            println!("Playlist name: {:?}", self.playlist.name);
            println!("Playlist owner ID: {:?}", self.playlist.owner.id);
        } else {
            println!("Playlists are not equal.");
            println!("Playlist 1 ID: {:?}", self.playlist.id);
            println!("Playlist 2 ID: {:?}", other.playlist.id);
            println!("Playlist 1 name: {:?}", self.playlist.name);
            println!("Playlist 2 name: {:?}", other.playlist.name);
            println!("Playlist 1 owner ID: {:?}", self.playlist.owner.id);
            println!("Playlist 2 owner ID: {:?}", other.playlist.owner.id);
        }
        eq_id
    }

    /// Combines three input vectors into a single vector of tuples, where each tuple contains one element
    /// from each vector. If the vectors are of different lengths, the shorter vectors are padded with a
    /// default value.
    ///
    /// # Type Parameters
    /// - `T`: The type of the elements in the vectors. Must implement the `Clone` trait.
    ///
    /// # Parameters
    /// - `v1`: The first vector of elements.
    /// - `v2`: The second vector of elements.
    /// - `v3`: The third vector of elements.
    /// - `headers`: A tuple of three elements to be used as the header row in the resulting vector.
    /// - `default`: A default value to be used when one of the vectors is shorter than the others. This value
    ///   will be cloned as needed.
    ///
    /// # Returns
    /// A vector of tuples `(T, T, T)`, where each tuple contains one element from each of the input vectors
    /// (or the default value if the corresponding input vector is shorter). The first element in the resulting
    /// vector is always the provided `headers`.
    fn combine_vectors<T: Clone>(
        v1: Vec<T>,
        v2: Vec<T>,
        v3: Vec<T>,
        headers: (T, T, T),
        default: T,
    ) -> Vec<(T, T, T)> {
        let first_len = std::cmp::max(v1.len(), v2.len());
        let len = std::cmp::max(first_len, v3.len());
        let mut combined = Vec::with_capacity(len);
        combined.push(headers);

        for i in 0..len {
            let elem1 = v1.get(i).cloned().unwrap_or_else(|| default.clone());
            let elem2 = v2.get(i).cloned().unwrap_or_else(|| default.clone());
            let elem3 = v3.get(i).cloned().unwrap_or_else(|| default.clone());
            combined.push((elem1, elem2, elem3));
        }
        combined
    }
}
