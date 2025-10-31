use std::collections::HashSet;
use std::ops::Index;

use rspotify::clients::BaseClient;
use rspotify::model::{FullPlaylist, PlaylistId, SearchResult, SearchType, SimplifiedPlaylist};
use rspotify::{scopes, AuthCodeSpotify, ClientError};

use crate::traits::apis::Api;

/// The `PlaylistQuery` struct is designed to handle operations related to querying playlists
/// by utilizing the `AuthCodeSpotify` client for authentication and interaction with the Spotify API.
///
/// # Fields
///
/// * `client` (`AuthCodeSpotify`):
///   An instance of the `AuthCodeSpotify` client used to authenticate and interact with Spotify's services.
///
/// // Use the `playlist_query` instance to interact with Spotify playlists.
/// ```
pub struct PlaylistQuery {
    pub client: AuthCodeSpotify,
}

impl Api for PlaylistQuery {
    fn select_scopes() -> HashSet<String> {
        scopes!(
            "playlist-read-private",
            "playlist-read-collaborative",
            "playlist-modify-public",
            "playlist-modify-private"
        )
    }
}
impl PlaylistQuery {
    /// Asynchronously creates a new instance of `PlaylistQuery` with an authenticated client configured for specific Spotify Web API scopes.
    ///
    /// # Scopes
    /// The following OAuth scopes are requested to interact with Spotify playlists:
    /// - `playlist-read-private` - Allows access to private playlists.
    /// - `playlist-read-collaborative` - Grants access to collaborative playlists.
    /// - `playlist-modify-public` - Allows modification of public playlists.
    /// - `playlist-modify-private` - Allows modification of private playlists.
    ///
    /// # Returns
    /// An initialized `PlaylistQuery` instance with an authenticated Spotify client.
    ///
    /// # Example
    /// ```no_run,ignore
    /// use spotify_assistant_core::actions::playlists::query::PlaylistQuery;
    /// async fn main() {
    ///     let playlist_query = PlaylistQuery::new().await;
    /// }
    /// ```
    ///
    /// # Notes
    /// - The `set_up_client` function is expected to handle the client setup including authentication.
    /// - The function is asynchronous and must be awaited.
    pub async fn new() -> Self {
        let scope = scopes!(
            "playlist-read-private",
            "playlist-read-collaborative",
            "playlist-modify-public",
            "playlist-modify-private"
        );
        PlaylistQuery {
            client: Self::set_up_client(false, Some(scope)).await,
        }
    }

    /// Asynchronously fetches a Spotify playlist using its ID.
    ///
    /// This function takes a playlist ID as a string, validates its format, and then retrieves
    /// the full playlist information using the Spotify client associated with the instance of the
    /// struct. If the playlist ID is invalid or the Spotify API call fails, it returns an error.
    ///
    /// # Arguments
    ///
    /// * `playlist_id_as_str` - A string slice containing the playlist ID.
    ///
    /// # Returns
    ///
    /// * `Ok(FullPlaylist)` - If the playlist is successfully retrieved, it returns a `FullPlaylist` object.
    /// * `Err(Box<dyn std::error::Error>)` - If an error occurs during parsing the ID or fetching the playlist, an `Error` is returned wrapped in a `Box`.
    ///
    /// # Errors
    ///
    /// This function can return an error in the following scenarios:
    /// * The provided `playlist_id_as_str` is not a valid Spotify playlist ID.
    /// * An error occurs during the API call to fetch the playlist.
    ///
    /// # Examples
    /// ```no_run,ignore
    /// use spotify_assistant_core::actions::playlists::query::PlaylistQuery;
    /// async fn main() {
    ///     let client = PlaylistQuery::new().await;
    ///     match client.get_playlist("37i9dQZF1DXcBWIGoYBM5M").await {
    ///         Ok(playlist) => println!("Playlist name: {}", playlist.name),
    ///         Err(err) => eprintln!("Error fetching playlist: {}", err),
    ///     }
    /// }
    /// ```
    ///
    /// # Note
    ///
    /// This function uses the Spotify Web API and expects the caller to ensure that the API client
    /// has access rights and a valid configuration.
    pub async fn get_playlist(&self, playlist_id_as_str: &str) -> Result<FullPlaylist, Box<dyn std::error::Error>> {
        let playlist_id = match PlaylistId::from_id(playlist_id_as_str) {
            Ok(id) => { id }
            Err(err) => { return Err(Box::new(err)) }
        };
        match self.client.playlist(playlist_id, None, None).await {
            Ok(pl) => { Ok(pl) }
            Err(err) => { Err(Box::new(err)) }
        }
    }

    /// Constructs a case-insensitive regex pattern matching the provided list of words in sequence
    /// with boundaries for each word.
    ///
    /// The resulting pattern ensures:
    /// - The pattern matches text containing the specified words in the given order.
    /// - Each word is surrounded by word boundaries (`\b`) to ensure exact word matching.
    /// - Any number of characters can appear between words, due to the use of `.*`.
    /// - Case-insensitivity is achieved by setting the `(?i)` flag at the beginning of the pattern.
    ///
    /// # Parameters
    /// - `words`: A vector of string slices (`Vec<&str>`) representing the words to be included
    ///   in the pattern. The words are matched in the order they appear in this vector.
    ///
    /// # Returns
    /// - A `String` containing the constructed regex pattern.
    ///
    /// In the above example, the returned pattern will match text that contains:
    /// - The word "rust" (case-insensitively),
    /// - Followed by the word "regex" (case-insensitively),
    /// - Followed by the word "pattern" (case-insensitively),
    /// - With any characters allowed between those words.
    fn construct_pattern(&self, words: Vec<&str>) -> String {
        // Create the base pattern with case-insensitive flag
        let mut pattern = String::from("(?i).*");

        // Iterate over the words and construct the pattern
        for (i, word) in words.iter().enumerate() {
            if i > 0 {
                pattern.push_str(".*"); // Match any characters between words
            }
            pattern.push_str(r"\b");
            pattern.push_str(word);
            pattern.push_str(r"\b");
        }
        pattern.push_str(".*");

        pattern
    }

    /// Queries a public playlist that matches the provided `playlist_name`.
    ///
    /// This function uses a Spotify client to search for playlists, filters the results using a regex
    /// matching the input playlist name, and prompts the user to select a specific playlist from the filtered results.
    /// Upon user selection, the function retrieves and returns the details of the chosen playlist.
    ///
    /// # Arguments
    ///
    /// * `playlist_name` - A `String` representing the name or partial name of the playlist to search for.
    ///
    /// # Returns
    ///
    /// This function returns a `Result<FullPlaylist, ClientError>`, where:
    /// - `FullPlaylist` contains the details of the selected public playlist.
    /// - `ClientError` represents any error that might occur during the query or API interaction.
    ///
    /// # Behavior
    ///
    /// 1. The function builds a regex pattern from the `playlist_name` to match potential playlists.
    /// 2. Searches for playlists using the Spotify API with a maximum of 50 results.
    /// 3. Filters the search results to find playlists with names matching the regex pattern.
    /// 4. Uses `dialoguer` to present a selectable list of filtered playlists to the user.
    /// 5. Fetches and returns the full details of the selected playlist.
    /// 6. Prints information about the selected playlist, such as its name and owner.
    ///
    /// # Error Handling
    ///
    /// - If the search result doesn't match the expected `SearchResult::Playlists` variant, a default playlist is returned.
    /// - Any unwrapping failures for Spotify API interactions will result in a runtime panic.
    /// - If regex creation fails, it will also result in a runtime panic.
    /// - The code currently does not gracefully handle user interaction errors with the playlist selection.
    ///
    /// # Examples
    /// ```no_run,ignore
    /// use spotify_assistant_core::actions::playlists::query::PlaylistQuery;
    /// async fn main() {
    ///     let client = PlaylistQuery::new().await;
    ///     let playlist_name = "Chill Vibes".to_string();
    ///     match client.query_public_playlist(playlist_name).await {
    ///         Ok(full_playlist) => {
    ///             println!("Playlist ID: {:?}", full_playlist.id);
    ///         },
    ///         Err(err) => {
    ///             eprintln!("Error querying playlist: {:?}", err);
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// # Dependencies
    ///
    /// - `regex` crate is used for building and matching the playlist name.
    /// - `dialoguer` crate is used for user interaction to select the desired playlist.
    /// - Spotify API client must be properly configured to use this functionality.
    ///
    /// # Notes
    ///
    /// - Ensure the Spotify API credentials are configured and have the appropriate permissions to search public playlists.
    /// - The function currently uses `unwrap()` extensively, which can lead to panics in case of errors. Consider improving error handling.
    ///
    /// # Panics
    ///
    /// The function may panic under the following conditions:
    /// - Failure to create the regex pattern.
    /// - Any `unwrap()` failure when handling results from the Spotify client API, interaction dialogs, or regex matching.
    pub async fn query_public_playlist(&self, playlist_name: String) -> Result<FullPlaylist, ClientError> {
        let market = Self::market();
        let results = self.client.search(&playlist_name, SearchType::Playlist, Some(market), None, Some(50), None).await.unwrap();
        let pl_name_vec = playlist_name.split(" ").collect::<Vec<&str>>();
        let regex_pattern = self.construct_pattern(pl_name_vec);
        let regex_match = regex::Regex::new(regex_pattern.as_str()).unwrap();
        match results {
            SearchResult::Playlists(paginator) => {
                let oop = paginator.clone().items.into_iter().filter(|pl| {
                    regex_match.is_match(pl.name.as_str())
                }).collect::<Vec<SimplifiedPlaylist>>();

                let sel =
                    dialoguer::Select::new()
                        .items(&oop.clone().iter().map(|pl| {
                            if let Some(displayname) = &pl.owner.display_name {
                                format!("{} - {:?}", pl.name.as_str(), displayname)
                            } else {
                                format!("{}", pl.name.as_str())
                            }
                        }).collect::<Vec<String>>())
                        .interact().unwrap();

                let selected = oop.index(sel);
                if let Some(displayname) = &selected.owner.display_name {
                    println!("Selection: {:?} by {:?}", selected.name, displayname);
                    println!("Selection ID: {:?}", selected.id);
                } else {
                    println!("Selection: {:?}", selected.name);
                };
                Ok(self.client.playlist(oop.index(sel).id.clone(), None, Some(market)).await?)
            }
            _ => {
                println!("Error: {:?}", results);
                Ok(self.client.playlist(PlaylistId::from_id("37i9dQZEVXbdINACbjb1qu").unwrap(), None, Some(market)).await?)
            }
        }
    }
}
