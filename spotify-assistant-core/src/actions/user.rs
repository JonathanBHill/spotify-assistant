use crate::enums::fs::ProjectDirectories;
use crate::paginator::PaginatorRunner;
use crate::traits::apis::Api;
use rspotify::clients::OAuthClient;
use rspotify::model::{FullArtist, FullTrack, Id, PlayHistory, PrivateUser, SimplifiedPlaylist, SubscriptionLevel, TimeRange};
use rspotify::{scopes, AuthCodeSpotify};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use tracing::{event, info, Level};

/// `UserData` is a structure that holds information about a Spotify user and their authorized Spotify client.
///
/// # Fields
///
/// * `client` (`AuthCodeSpotify`):
///   The Spotify client authorized with OAuth2 for interacting with the Spotify API.
///   This field is private and used to manage authenticated requests on behalf of the user.
///
/// * `user` (`PrivateUser`):
///   A public field containing details about the authenticated Spotify user, such as
///   their profile information. This is fetched from the Spotify API and provides
///   user-specific data.
///
/// # Derives
///
/// * `Debug`:
///   Implements the `Debug` trait, which allows instances of `UserData` to be
///   formatted using the `{:?}` formatter for debugging purposes.
#[derive(Debug)]
pub struct UserData {
    client: AuthCodeSpotify,
    pub user: PrivateUser,
}

impl Api for UserData {
    fn select_scopes() -> HashSet<String> {
        scopes!(
            "user-read-private",
            "user-read-email",
            "user-read-recently-played",
            "user-top-read",
            "user-follow-read"
        )
    }
}

impl UserData {
    /// Asynchronously initializes a new instance of `UserData`.
    ///
    /// This function performs the following steps to create and return a new `UserData` instance:
    ///
    /// 1. Creates a tracing span to monitor and log the process of initializing user data.
    /// 2. Sets up an authentication client with predefined scopes using the `set_up_client` method.
    /// 3. Logs that the user has successfully authenticated with the client.
    /// 4. Retrieves the currently authenticated user's data using the client.
    /// 5. Logs that the user data has been successfully retrieved.
    /// 6. Initializes an instance of `UserData` with the authenticated client and user information.
    /// 7. Logs that the user data initialization is complete.
    ///
    /// # Returns
    /// An instance of `UserData` containing the authentication client and the current user's data.
    ///
    /// # Panics
    /// This function will panic if the current user's data cannot be retrieved. Ensure that the
    /// authentication credentials and setup are valid before calling this method.
    ///
    /// # Example
    /// ```rust
    /// use spotify_assistant_core::actions::user::UserData;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let user_data = UserData::new().await;
    ///     println!("User data initialized successfully.");
    /// }
    /// ```
    pub async fn new() -> Self {
        let span = tracing::span!(Level::INFO, "UserData.new");
        let _enter = span.enter();
        let client = UserData::set_up_client(false, Some(UserData::select_scopes())).await;
        event!(Level::INFO, "User has been authenticated with client.");
        let user = client.current_user().await.ok().unwrap();
        event!(Level::INFO, "User data has been retrieved.");
        let user_data = UserData { client, user };
        event!(Level::INFO, "User data has been initialized.");
        user_data
    }

    /// Returns the subscription level of the user.
    ///
    /// This method checks the `product` field of the `user` object.
    /// If the `product` field contains a value, it returns the corresponding
    /// `SubscriptionLevel`. If the `product` field is `None`, it defaults to
    /// returning `SubscriptionLevel::Free`.
    ///
    /// # Returns
    ///
    /// * `SubscriptionLevel` - The current subscription level of the user.
    ///   Defaults to `SubscriptionLevel::Free` if no subscription is specified.
    ///
    /// # Examples
    ///
    /// ```
    /// use spotify_assistant_core::actions::user::UserData;
    /// use rspotify::model::SubscriptionLevel;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let user_data = UserData::new().await;
    ///     let subscription_level = user_data.product();
    ///     assert_eq!(subscription_level, SubscriptionLevel::Premium);
    ///
    ///     let subscription_level = user_data.product();
    ///     assert_eq!(subscription_level, SubscriptionLevel::Free);
    /// }
    /// ```
    pub fn product(&self) -> SubscriptionLevel {
        self.user.product.unwrap_or(SubscriptionLevel::Free)
    }

    /// Returns a string representation of the user's subscription product level.
    ///
    /// This method checks the user's subscription level and converts it into a
    /// human-readable string format. If the user's product level is not set
    /// (i.e., `None`), it defaults to the `Free` subscription level.
    ///
    /// # Returns
    /// - `"Premium"` if the user's subscription level is `SubscriptionLevel::Premium`.
    /// - `"Free"` if the user's subscription level is `SubscriptionLevel::Free` or not set.
    ///
    /// # Examples
    /// ```
    /// use spotify_assistant_core::actions::user::UserData;
    /// #[tokio::main]
    /// async fn main() {
    ///     let instance = UserData::new().await;
    ///     // Assuming `self.user.product` is `Some(SubscriptionLevel::Premium)`
    ///     assert_eq!(instance.product_as_string(), "Premium");
    ///
    ///     // If `self.user.product` is `None`
    ///     assert_eq!(instance.product_as_string(), "Free");
    /// }
    /// ```
    pub fn product_as_string(&self) -> String {
        let product = self.user.product.unwrap_or(SubscriptionLevel::Free);
        match product {
            SubscriptionLevel::Premium => "Premium".to_string(),
            SubscriptionLevel::Free => "Free".to_string(),
        }
    }

    /// Generates a `HashMap` containing the external URLs associated with the user.
    ///
    /// This method creates a new `HashMap` that contains key-value pairs representing
    /// the external URLs associated with the current user. It iterates over the user's
    /// external URLs and adds each key-value pair from the `external_urls` field to the
    /// map. Additionally, it includes a special key `"href"` which maps to the user's
    /// `href` string.
    ///
    /// # Returns
    /// A `HashMap<String, String>` where:
    /// - Keys are strings representing the names of the external URLs or `"href"`.
    /// - Values are strings representing the corresponding external URL or the user's `href`.
    ///
    /// # Example
    /// ```
    /// use spotify_assistant_core::actions::user::UserData;
    /// #[tokio::main]
    /// async fn main() {
    ///     let instance = UserData::new().await;
    ///     let urls = instance.urls();
    ///     for (key, value) in urls {
    ///         println!("Key: {}, Value: {}", key, value);
    ///     }
    /// }
    /// ```
    ///
    /// # Assumptions
    /// - The `self.user.external_urls` is an iterable collection of key-value pairs.
    /// - The `self.user.href` is a valid `String` value.
    pub fn urls(&self) -> HashMap<String, String> {
        let mut urls = HashMap::new();
        for (key, value) in self.user.external_urls.iter() {
            urls.insert(key.to_string(), value.to_string());
        }
        urls.insert("href".to_string(), self.user.href.clone());
        urls
    }

    /// Retrieves the URL of the first image associated with the user.
    ///
    /// This method accesses the `images` field from the user's data,
    /// ensures it is not `None`, and clones it to avoid modifying the original data.
    /// It then retrieves and returns the URL of the first image in the list.
    ///
    /// # Returns
    ///
    /// A `String` containing the URL of the first image.
    ///
    /// # Panics
    ///
    /// This function will panic if the `images` field is `None` or if the list of images is empty.
    /// Make sure the user data includes a populated `images` field before calling this function.
    pub fn image(&self) -> String {
        let images = self
            .user
            .images
            .clone()
            .expect("Could not get the user's images");
        images[0].url.clone()
    }

    /// Converts the total number of followers of a user into a `String`.
    ///
    /// If the `followers` field of the user is present (`Some`), its `total` value (representing the number of followers)
    /// will be converted into a `String`. If the `followers` field is `None`, a default value of `0` will be used instead.
    ///
    /// # Returns
    ///
    /// A `String` representation of the total number of followers.
    ///
    /// # Example
    ///
    /// ```rust
    /// use spotify_assistant_core::actions::user::UserData;
    /// #[tokio::main]
    /// async fn main() {
    ///     let instance = UserData::new().await;
    ///     let followers_string = instance.followers_as_string();
    ///     println!("Followers: {followers_string}");
    /// }
    /// ```
    pub fn followers_as_string(&self) -> String {
        self.user
            .followers
            .clone()
            .unwrap_or_default()
            .total
            .to_string()
    }

    /// Returns the total number of followers for the user.
    ///
    /// This function retrieves the `followers` value from the associated user object.
    /// If the `followers` field is `None`, it will return a default value of `0`.
    /// Otherwise, it returns the total followers count.
    ///
    /// # Returns
    /// * `u32` - The total number of followers, or `0` if the value is not set.
    ///
    /// # Examples
    /// ```
    /// use spotify_assistant_core::actions::user::UserData;
    /// #[tokio::main]
    /// async fn main() {
    ///     let instance = UserData::new().await;
    ///     let followers_as_num = instance.followers();
    ///     println!("Followers: {followers_as_num}");
    /// }
    /// ```
    pub fn followers(&self) -> u32 {
        self.user.followers.clone().unwrap_or_default().total
    }

    /// Retrieves the user identifier based on the provided `id_type`.
    ///
    /// # Parameters
    /// - `id_type`: An `Option<String>` specifying the type of identifier to retrieve.
    ///    - If `Some`, can be one of the following:
    ///        - `"display_name"`: Returns the user's display name.
    ///        - `"email"`: Returns the user's email address.
    ///        - Any other value will return the user ID.
    ///    - If `None`, the user's display name is returned.
    ///
    /// # Returns
    /// A `String` representing the requested user identifier.
    ///
    /// # Panics
    /// This function will panic in the following scenarios:
    /// - If `id_type` is `"display_name"` or `None` but the display name is `None`.
    /// - If `id_type` is `"email"` but the email is `None`.
    ///
    /// # Examples
    /// ```
    /// use spotify_assistant_core::actions::user::UserData;
    /// use rspotify::model::Id;
    /// #[tokio::main]
    /// async fn main() {
    ///     let instance = UserData::new().await;
    ///     let id = instance.user.id;
    ///     println!("User Display Name: {}", id.id());
    /// }
    /// ```
    pub fn user_id(&self, id_type: Option<String>) -> String {
        match id_type {
            Some(data_unwrapped) => match data_unwrapped.as_str() {
                "display_name" => self
                    .user
                    .display_name
                    .clone()
                    .expect("Could not get display name")
                    .to_string(),
                "email" => self
                    .user
                    .email
                    .clone()
                    .expect("Could not get email")
                    .to_string(),
                _ => self.user.id.id().to_string(),
            },
            None => self
                .user
                .display_name
                .clone()
                .expect("Could not get display name")
                .to_string(),
        }
    }

    /// Retrieves the explicit content settings for the user.
    ///
    /// This method returns a `HashMap` containing information about the explicit content
    /// filtering configuration for the user.
    ///
    /// # Returns
    ///
    /// A `HashMap<&str, bool>` with the following keys:
    /// - `"filter_enabled"`: Indicates whether the explicit content filter is enabled.
    /// - `"filter_locked"`: Indicates whether the explicit content filter is locked.
    ///
    /// # Panics
    ///
    /// This method will panic with an `unreachable!` error if the explicit content settings
    /// for the user are not found (`None` in the `self.user.explicit_content`).
    ///
    /// # Example
    ///
    /// ```
    /// use spotify_assistant_core::actions::user::UserData;
    /// #[tokio::main]
    /// async fn main() {
    ///     let instance = UserData::new().await;
    ///     let settings = instance.explicit_content();
    ///     assert_eq!(settings.get("filter_enabled"), Some(&true));
    ///     assert_eq!(settings.get("filter_locked"), Some(&false));
    /// }
    /// ```
    ///
    /// Note: Ensure the user has explicit content settings defined to avoid runtime panics.
    pub fn explicit_content(&self) -> HashMap<&str, bool> {
        let mut explicit_settings = HashMap::new();
        match self.user.explicit_content.clone() {
            None => {
                unreachable!("Explicit content settings not found")
            }
            Some(explicit) => {
                explicit_settings.insert("filter_enabled", explicit.filter_enabled);
                explicit_settings.insert("filter_locked", explicit.filter_locked);
                explicit_settings
            }
        }
    }

    /// Retrieves the list of recently played tracks for the current user.
    ///
    /// This asynchronous function fetches the user's 50 most recently played tracks and reverses the order,
    /// so that the oldest track appears first in the returned list.
    /// It also retrieves the next URL for fetching more recently played tracks if available.
    ///
    /// # Returns
    /// A tuple consisting of:
    /// - `Vec<PlayHistory>`: A vector of `PlayHistory` items representing the user's reverse-ordered recent play history.
    /// - `String`: The next URL to fetch more play history (empty string if no next URL is available).
    ///
    /// # Panics
    /// This function will panic if it fails to retrieve the user's listening history from the Spotify API.
    ///
    /// # Example
    /// ```rust
    /// use spotify_assistant_core::actions::user::UserData;
    /// #[tokio::main]
    /// async fn main() {
    ///     let instance = UserData::new().await;
    ///     let (recently_played, next_url) = instance.get_recently_played().await;
    ///     println!("Oldest track: {:?}", recently_played.first());
    ///     println!("Next URL: {}", next_url);
    /// }
    /// ```
    pub async fn get_recently_played(&self) -> (Vec<PlayHistory>, String) {
        let results = match self.client.current_user_recently_played(Some(50), None).await {
            Ok(results) => {
                results
            }
            Err(err) => {
                panic!("Could not retrieve your listening history: {err:?}");
            }
        };
        let tracks = results.items;
        println!("{:?}", results.next);
        let reverse_tracks = tracks.into_iter().rev().collect::<Vec<PlayHistory>>();
        (reverse_tracks, results.next.unwrap_or_default())
    }

    /// Fetches the user's top tracks over a short-term time range and returns them as a `Vec<FullTrack>`.
    ///
    /// # Details
    /// - This function queries the current user's top tracks using pagination to handle a large number
    ///   of tracks efficiently.
    /// - The total number of tracks is determined first, and then multiple pages of tracks are fetched
    ///   from the Spotify API. Each page is appended to a single vector (`top_vec`), which is then returned.
    ///
    /// # Process
    /// - Calls the `current_user_top_tracks_manual` API to get the total number of top tracks.
    /// - Calculates the required number of pages based on the total tracks and the page size (50 items per page).
    /// - Iteratively requests each page from the API, appending results to the `top_vec`.
    /// - Tracks, along with debugging/logging information, are aggregated into the final vector.
    ///
    /// # Errors
    /// - If any API call to fetch the user's top tracks fails, the function panics with detailed error information.
    ///
    /// # Returns
    /// A vector of `FullTrack` objects containing information about the user's top tracks.
    ///
    /// # Dependencies
    /// - Uses the `tracing` crate to log information about the execution process.
    /// - Leverages Spotify's API client to fetch the user's top tracks.
    ///
    /// # Example
    /// ```rust
    /// use spotify_assistant_core::actions::user::UserData;
    /// #[tokio::main]
    /// async fn main() {
    ///     let instance = UserData::new().await;
    ///     let top_tracks = instance.top_tracks().await; // Fetches top tracks asynchronously
    ///     for track in top_tracks {
    ///         println!("Track Name: {}, Artist: {}", track.name, track.artists[0].name);
    ///     }
    /// }
    /// ```
    ///
    /// # Notes
    /// - The function is scoped with a logging span (`UserData.top-tracks`) for consistent logging output.
    /// - Currently, commented-out functionality (`save_to_file`) exists for serialization of top tracks to a file.
    /// - Tracks are inserted into the vector at the correct positions based on their index to ensure order preservation.
    pub async fn top_tracks(&self, term: &str) -> Vec<FullTrack> {
        let span = tracing::span!(Level::INFO, "UserData.top-tracks");
        let _enter = span.enter();

        let time_range = match term {
            "short" => TimeRange::ShortTerm,
            "medium" => TimeRange::MediumTerm,
            "long" => TimeRange::LongTerm,
            _ => TimeRange::ShortTerm,
        };
        let top_tracks = self.client.current_user_top_tracks(Some(time_range));
        let paginator = PaginatorRunner::new(top_tracks, ());
        paginator.run().await.unwrap_or_else(|err| {
            event!(Level::ERROR, "Error retrieving top tracks: {:?}", err);
            Vec::new()
        })
    }

    /// Asynchronously retrieves and logs the current user's playlists.
    ///
    /// This function fetches the playlists of the current user using the
    /// `current_user_playlists_manual` method of the Spotify client. It logs
    /// each playlist's name and the total number of playlists available.
    ///
    /// # Behavior
    /// - A tracing span is entered to track the function's execution flow.
    /// - The function fetches the playlists with a page limit of 1.
    /// - Each playlist's name is logged using the `info!` macro.
    /// - The total number of playlists is logged at the end.
    ///
    /// # Panics
    /// This function will panic if an error occurs while attempting to fetch the playlists,
    /// providing a debug representation of the error.
    ///
    /// # Examples
    /// ```no_run
    /// use spotify_assistant_core::actions::user::UserData;
    /// #[tokio::main]
    /// async fn main() {
    ///     let instance = UserData::new().await;
    ///     instance.playlists().await;
    /// }
    /// ```
    ///
    /// # Dependencies
    /// - This function assumes the use of the `tracing` library for logging and spans.
    /// - The `client` field in `self` must provide the `current_user_playlists_manual`
    ///   method for fetching playlists.
    ///
    /// # Logging
    /// - Logs the name of each playlist.
    /// - Logs the total number of playlists fetched.
    ///
    /// # Notes
    /// - Adjust the pagination parameters (e.g., limit, offset) in the
    ///   `current_user_playlists_manual` call as needed to fetch more playlists.
    ///
    /// # Errors
    /// - All errors will result in an immediate `panic!`, so error handling may
    ///   need to be revised for more robust applications.
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

    /// Retrieves a list of artists followed by the current user.
    ///
    /// This asynchronous function fetches the list of artists followed by the user using the Spotify client.
    /// It processes paginated data to handle cases where the total number of followed artists exceeds
    /// the limit of artists retrievable in a single API call.
    ///
    /// # Details
    /// - The function begins by initializing a tracing span for debugging and logging purposes.
    /// - It sets a limit of 50 artists to be retrieved per API call.
    /// - If the total number of artists exceeds the limit, it iteratively retrieves data for multiple pages
    ///   and appends the results to a vector of artists.
    ///
    /// For each page:
    /// - Logs the current page and its index.
    /// - Fetches subsequent artists, keeping track of the last artist ID as a starting point for the next page.
    /// - Logs each artist and its position in the overall list during the retrieval process.
    ///
    /// # Returns
    /// - A `Vec<FullArtist>`: A vector containing all artists followed by the user.
    ///
    /// # Panics
    /// - The function will panic if it encounters an error while fetching the user's followed artists using
    ///   the Spotify API.
    ///
    /// # Logging
    /// - Logged information includes:
    ///   - Total number of pages and artists.
    ///   - Each page's progress and the respective artist data.
    ///
    /// # Example Usage
    /// ```rust
    /// use spotify_assistant_core::actions::user::UserData;
    /// #[tokio::main]
    /// async fn main() {
    ///     let instance = UserData::new().await;
    ///     let followed_artists = instance.artists().await;
    ///     for artist in followed_artists {
    ///         println!("Artist: {}", artist.name);
    ///     }
    /// }
    /// ```
    ///
    /// # Dependencies
    /// - The function relies on the implementation of a Spotify API client (`self.client`) that contains
    ///   methods such as `current_user_followed_artists` for data retrieval.
    ///
    /// # Notes
    /// - Errors are not gracefully handled and will cause a panic. Consider adding proper error handling
    ///   if required for production use.
    pub async fn artists(&self) -> Vec<FullArtist> {
        let span = tracing::span!(Level::INFO, "UserData.artists");
        let _enter = span.enter();

        let limit = 50;
        let artists = self
            .client
            .current_user_followed_artists(None, Some(1))
            .await;
        let mut followed_artists = Vec::new();
        match artists {
            Ok(artists) => {
                let total = artists.total.unwrap_or(limit);
                let mut last_artist_id = artists.items.last().unwrap().id.clone();
                let repetitions = total / limit;
                let remainder = total % limit;
                let pages = if remainder > 0 {
                    repetitions + 1
                } else {
                    repetitions
                };
                info!("Total pages: {:?}", pages);
                info!("Total artists: {:?}", total);
                for page in 0..pages {
                    info!("Page: {:?} of {}", page + 1, pages);
                    let artists = self
                        .client
                        .current_user_followed_artists(Some(last_artist_id.id()), Some(limit))
                        .await;
                    match artists {
                        Ok(artists) => {
                            last_artist_id = artists.items.last().unwrap().id.clone();
                            artists
                                .items
                                .iter()
                                .enumerate()
                                .for_each(|(index, artist)| {
                                    let true_index = index as u32 + (page * limit);
                                    info!("{}: {:?}", true_index, artist.name);
                                    followed_artists.push(artist.clone());
                                });
                        }
                        Err(error) => panic!("Could not get artists: {error:?}"),
                    }
                }
                info!("Total artists: {:?}", artists.total);
            }
            Err(error) => panic!("Could not get artists: {error:?}"),
        }
        followed_artists
    }
    pub async fn update_followed_artists(&self) {
        let span = tracing::span!(Level::INFO, "UserData.update-artists");
        let _enter = span.enter();
        let artists = self.artists().await;
        let follower_length = artists.len();
        let local_time = chrono::Local::now();
        let local_time_string = local_time.format("%m-%d-%Y").to_string();
        let file_name = format!("{}_followers-{}.json", follower_length, local_time_string);
        let file_dir = Self::follower_file_directory();
        let file_path = file_dir.join(file_name);
        let artists_json = serde_json::to_string_pretty(&artists).unwrap();
        std::fs::write(file_path.clone(), artists_json).unwrap();
        info!("Stored {} artists to file path: {:?}", follower_length, file_path);
    }
    fn follower_file_directory() -> PathBuf {
        let data_path = ProjectDirectories::Data.path();
        data_path.join("followers")
    }
    pub async fn save_artists_locally_manual(&self, artists: Vec<FullArtist>) -> () {
        let span = tracing::span!(Level::INFO, "UserData.store-artists");
        let _enter = span.enter();
        let follower_length = artists.len();
        let local_time = chrono::Local::now();
        let local_time_string = local_time.format("%m-%d-%Y").to_string();
        let file_name = format!("{}_followers-{}.json", follower_length, local_time_string);
        let file_dir = Self::follower_file_directory();
        let file_path = file_dir.join(file_name);
        let json = serde_json::to_string_pretty(&artists).unwrap();
        std::fs::write(file_path.clone(), json).unwrap();
        info!("Stored {} artists to file path: {:?}", follower_length, file_path);
    }
    pub fn read_artists_from_file(&self, file_name: &str) -> Vec<FullArtist> {
        let span = tracing::span!(Level::INFO, "UserData.read-artists");
        let _enter = span.enter();
        let file_dir = Self::follower_file_directory();
        let file_name = format!("{}.json", file_name);
        let file_path = file_dir.join(file_name);
        let json = std::fs::read_to_string(file_path.clone()).unwrap();
        let artists: Vec<FullArtist> = serde_json::from_str(&json).unwrap();
        info!("Read {} artists from file path: {:?}", artists.len(), file_path);
        artists
    }
    fn artists_not_followed_from_playlist_directory() -> PathBuf {
        let data_path = ProjectDirectories::Data.path();
        data_path.join("playlist-artists")
    }
    pub fn save_unfollowed_artists_locally(&self, artists: Vec<FullArtist>, playlist_name: String) {
        let span = tracing::span!(Level::INFO, "UserData.store-unfollowed-artists");
        let _enter = span.enter();
        let local_time = chrono::Local::now();
        let local_time_string = local_time.format("%m-%d-%Y").to_string();
        let file_name = format!("{}_artists_not_followed-{}.json", playlist_name, local_time_string);
        let file_dir = Self::artists_not_followed_from_playlist_directory();
        let file_path = file_dir.join(file_name);
        let json = serde_json::to_string_pretty(&artists).unwrap();
        std::fs::write(file_path.clone(), json).unwrap();
    }
}
