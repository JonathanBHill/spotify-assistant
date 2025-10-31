use std::collections::HashSet;

use chrono::{DateTime, Utc};
use rspotify::clients::OAuthClient;
use rspotify::model::{CursorBasedPage, PlayHistory, TimeLimits};
use rspotify::{scopes, AuthCodeSpotify};

use crate::traits::apis::Api;

/// Represents the listening history of a user obtained through the Spotify API.
///
/// The `UserListeningHistory` struct contains information related to a user's recently played tracks
/// and provides functionality to interact with Spotify's API for retrieving listening history.
///
/// # Fields
///
/// - `client`:
///   An authenticated instance of the `AuthCodeSpotify` client used to communicate with the Spotify Web API.
///
/// - `tracks`:
///   A vector of `PlayHistory` instances, where each record represents metadata about a track and its playback context.
///
/// - `next`:
///   A string containing the URL pointing to the next page of the user's listening history data.
///   This can be used for pagination to fetch additional listening history records.
///
/// Note: Proper authentication is required to access the user's listening history via the Spotify API.
/// Ensure that the `client` field of this struct is correctly configured.
pub struct UserListeningHistory {
    client: AuthCodeSpotify,
    tracks: Vec<PlayHistory>,
    next: String,
}

impl Api for UserListeningHistory {
    fn select_scopes() -> HashSet<String> {
        scopes!(
            "user-read-recently-played"
        )
    }
}

impl UserListeningHistory {
    /// Asynchronously initializes a new instance of `UserListeningHistory`.
    ///
    /// This function performs the following tasks:
    /// 1. Sets up an authenticated client using the `set_up_client` method.
    /// 2. Fetches the user's recently played tracks by invoking the `current_user_recently_played`
    ///    method on the client, with a limit of 50 items.
    /// 3. Extracts the next page's URL, if available, to enable pagination for further data retrieval.
    /// 4. Returns an instance of `UserListeningHistory` containing:
    ///    - The authenticated client.
    ///    - A vector of `PlayHistory` items representing the user's recent playback history,
    ///      ordered from oldest to newest.
    ///    - The URL for the next page of results (or an empty string if no further pages are available).
    ///
    /// # Panics
    /// This function will panic if:
    /// - The client's `current_user_recently_played` method encounters an error while fetching
    ///   the user's listening history.
    ///
    /// # Errors
    /// If there are no more pages of results to retrieve, a warning is printed to the standard error stream.
    ///
    /// # Returns
    /// A new `UserListeningHistory` object, structured with the fetched data.
    ///
    /// # Example
    /// ```no_run,ignore
    /// use spotify_assistant_core::actions::recently_played::UserListeningHistory;
    /// async fn main() {
    ///     let user_history = UserListeningHistory::new().await;
    ///     println!("{:?}", user_history.tracks());
    /// }
    /// ```
    ///
    /// # Dependencies
    /// - The client setup depends on the `set_up_client` method and a valid scope configuration
    ///   provided by `select_scopes()`.
    pub async fn new() -> Self {
        let client = Self::set_up_client(false, Some(Self::select_scopes())).await;
        let results = match client.current_user_recently_played(Some(50), None).await {
            Ok(results) => { results }
            Err(err) => { panic!("Could not retrieve your listening history: {:?}", err) }
        };
        let next = match results.next {
            Some(string) => { string }
            None => {
                eprintln!("Error: No more pages to retrieve.");
                "".to_string()
            }
        };
        UserListeningHistory {
            client,
            tracks: results.items.into_iter().rev().collect::<Vec<PlayHistory>>(),
            next,
        }
    }

    ///
    /// Retrieves a list of play history tracks.
    ///
    /// This method returns a clone of the `tracks` field, which is a vector
    /// containing instances of `PlayHistory`. Each `PlayHistory` represents
    /// a record of a track that has been played.
    ///
    /// # Returns
    ///
    /// A `Vec<PlayHistory>` that contains the play history tracks.
    ///
    /// # Examples
    /// ```no_run,ignore
    /// use spotify_assistant_core::actions::recently_played::UserListeningHistory;
    /// async fn main() {
    ///     let user_history = UserListeningHistory::new().await;
    ///     let history = user_history.tracks();
    ///     for track in history {
    ///         println!("{:?}", track);
    ///     }
    /// }
    /// ```
    pub fn tracks(&self) -> Vec<PlayHistory> {
        self.tracks.clone()
    }

    /// Asynchronously retrieves the next page of the user's recently played tracks using a cursor-based pagination mechanism.
    ///
    /// # Returns
    ///
    /// A `CursorBasedPage` containing a list of `PlayHistory` items representing the user's listening history.
    ///
    /// # Errors
    ///
    /// This function will panic if it fails to retrieve the user's listening history from the Spotify API.
    /// The error message will contain details about the failure.
    ///
    /// # Implementation Details
    ///
    /// - The method first calculates the time limit for retrieving the next set of tracks by calling `self.get_time_limit()`.
    /// - The Spotify client (`self.client`) is then used to fetch up to 50 items filtered by the calculated time limit.
    /// - If the API call succeeds, the resulting `CursorBasedPage<PlayHistory>` is returned.
    /// - If the API call fails, the function will `panic!` with an error message containing the details of the error.
    ///
    /// # Example
    /// ```no_run,ignore
    /// use spotify_assistant_core::actions::recently_played::UserListeningHistory;
    /// async fn main() {
    ///     let user_history = UserListeningHistory::new().await;
    ///     let page = user_history.next().await;
    ///     for history in page.items {
    ///         println!("Played track: {}", history.track.name);
    ///     }
    /// }
    /// ```
    pub async fn next(&self) -> CursorBasedPage<PlayHistory> {
        let next = self.get_time_limit();
        let results = match self.client.current_user_recently_played(Some(50), Some(next)).await {
            Ok(results) => { results }
            Err(err) => { panic!("Could not retrieve your listening history: {:?}", err) }
        };
        results
    }

    /// Retrieves the time limit specified in the `next` field of the struct and returns it as a `TimeLimits` enum.
    ///
    /// This function assumes that the `next` field contains specific query parameters in the format:
    /// `before=<timestamp>&limit=<value>`. The function extracts the `before` timestamp from this string,
    /// parses it into an integer, and then converts it to a `DateTime<Utc>` object.
    ///
    /// # Returns
    /// * `TimeLimits::Before(datetime)` - A variant of the `TimeLimits` enum containing the parsed `DateTime<Utc>`
    ///   object.
    ///
    /// # Panics
    /// The function will panic in the following scenarios:
    /// 1. If the `next` field does not contain a `before=<timestamp>` parameter or its format is invalid.
    /// 2. If the timestamp value cannot be parsed into an integer.
    /// 3. If the parsed timestamp cannot be converted into a valid `DateTime<Utc>` object.
    ///
    /// Note: Ensure the `next` string field has a valid structure before invoking this method.
    fn get_time_limit(&self) -> TimeLimits {
        let timestamp = self.next
                            .split("before=").collect::<Vec<&str>>()[1]
            .split("&limit=").collect::<Vec<&str>>()[0]
            .to_string();
        let timestamp_parsed = match timestamp.parse() {
            Ok(timestamp) => { timestamp }
            Err(_) => { panic!("Could not parse timestamp") }
        };
        let datetime: DateTime<Utc> = match DateTime::from_timestamp(timestamp_parsed, 0) {
            Some(datetime) => { datetime }
            None => { panic!("Could not convert to DateTime") }
        };
        TimeLimits::Before(datetime)
    }

    /// Extends the play history by retrieving additional pages of play history and appending the retrieved tracks
    /// to the current history.
    ///
    /// # Parameters
    /// - `number_of_loops`: The number of iterations (or pages) to fetch and extend the play history.
    ///
    /// # Behavior
    /// - This asynchronous function fetches additional pages of play history based on the specified `number_of_loops`.
    /// - For each iteration:
    ///     - It uses the `self.next()` method to fetch the next page of play history data.
    ///     - The `items` from the fetched `next_page` are reversed and converted to a `Vec<PlayHistory>`.
    ///     - The reversed items are then extended into `self.tracks`.
    ///     - The `self.next` field is updated to point to the next page URL if provided, or an empty string if no further page exists.
    ///
    /// # Requirements
    /// - The `self.next()` method must return future-like objects that resolve into a structure containing the `items` and `next` URL for the next page.
    ///
    /// # Caveats
    /// - If `next_page.next` is `None` (indicating the end of the history), `self.next` is updated to an empty string, effectively terminating further loops early.
    ///
    /// # Usage
    /// Call this method when you want to extend the current play history with more records.
    ///
    /// # Example
    /// ```no_run,ignore
    /// use spotify_assistant_core::actions::recently_played::UserListeningHistory;
    /// async fn main() {
    ///     let mut user_history = UserListeningHistory::new().await;
    ///     user_history.extend_history(3).await; // Extends the history with 3 additional pages of play history.
    /// }
    /// ```
    pub async fn extend_history(&mut self, number_of_loops: u32) {
        for _ in 0..number_of_loops {
            let next_page = self.next().await;
            let tracks = next_page.items.into_iter().rev().collect::<Vec<PlayHistory>>();
            self.tracks.extend(tracks);
            self.next = match next_page.next {
                Some(next) => { next }
                None => { "".to_string() }
            };
        }
    }
}
