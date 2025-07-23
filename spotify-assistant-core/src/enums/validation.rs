/// Represents batch limits for various Spotify API endpoints.
///
/// Each variant in the `BatchLimits` enum corresponds to a specific type of API call,
/// along with its associated limit on the number of items that can be processed
/// in a single request. The numbers listed in the comments next to each variant
/// indicate the maximum allowed batch size.
///
/// # Variants
///
/// - `Albums`
///   Maximum batch size: 20.
///
/// - `AlbumTracks`
///   Maximum batch size: 50.
///
/// - `GetSavedAlbums`
///   Maximum batch size: 50.
///
/// - `ModifyCurrentUserAlbums`
///   Maximum batch size: 20.
///   Applies to the following API actions:
///     - Save albums for the current user.
///     - Remove the current user's saved albums.
///     - Check the current user's saved albums.
///
/// - `NewReleases`
///   Maximum batch size: 50.
///
/// - `Artists`
///   Maximum batch size: 50.
///
/// - `ArtistAlbums`
///   Maximum batch size: 50.
///
/// - `Audiobooks`
///   Maximum batch size: 50.
///
/// - `AudiobookChapters`
///   Maximum batch size: 50.
///
/// - `SavedAudiobooks`
///   Maximum batch size: 50.
///
/// - `ModifyCurrentUserAudiobook`
///   Maximum batch size: 50.
///   Applies to the following API actions:
///     - Save audiobooks for the current user.
///     - Remove the current user's saved audiobooks.
///     - Check the current user's saved audiobooks.
///
/// - `BrowseCategories`
///   Maximum batch size: 50.
///
/// - `AudiobooksChapters`
///   Maximum batch size: 50.
///
/// - `Episodes`
///   Maximum batch size: 50.
///
/// - `GetSavedEpisodes`
///   Maximum batch size: 50.
///
/// - `ModifyCurrentUserEpisodes`
///   Maximum batch size: 50.
///   Applies to the following API actions:
///     - Save episodes for the current user.
///     - Remove the current user's saved episodes.
///     - Check the current user's saved episodes.
///
/// - `RecentlyPlayed`
///   Maximum batch size: 50.
///
/// - `PlaylistItems`
///   Maximum batch size: 50.
///
/// - `ModifyPlaylistItems`
///   Maximum batch size: 100.
///   Applies to the following API actions:
///     - Add playlist items.
///     - Remove playlist items.
///     - Update playlist items.
///
/// - `UserPlaylists`
///   Maximum batch size: 50.
///   Applies to public user playlists and private/current user playlists.
///
/// - `GetPlaylists`
///   Maximum batch size: 50.
///   Applies to category and featured playlists.
///
/// - `SearchItem`
///   Maximum batch size: 50.
///
/// - `GetShows`
///   Maximum batch size: 50.
///
/// - `GetShowEpisodes`
///   Maximum batch size: 50.
///
/// - `GetUserSavedShows`
///   Maximum batch size: 50.
///
/// - `ModifyCurrentUserShows`
///   Maximum batch size: 50.
///   Applies to the following API actions:
///     - Save shows for the current user.
///     - Remove the current user's saved shows.
///     - Check the current user's saved shows.
///
/// - `Tracks`
///   Maximum batch size: 50.
///
/// - `GetSavedTracks`
///   Maximum batch size: 50.
///
/// - `ModifyCurrentUserTracks`
///   Maximum batch size: 50.
///   Applies to the following API actions:
///     - Save tracks for the current user.
///     - Remove the current user's saved tracks.
///     - Check the current user's saved tracks.
///
/// - `TracksAudioFeatures`
///   Maximum batch size: 100.
///
/// - `Recommendations`
///   Maximum batch size: 100.
///
/// - `CurrentUserTopItems`
///   Maximum batch size: 50.
///
/// - `CurrentUserFollowedArtists`
///   Maximum batch size: 50.
///
/// - `ModifyWhoCurrentUserFollows`
///   Maximum batch size: 50.
///   Applies to the following API actions:
///     - Follow artists or users.
///     - Unfollow artists or users.
///     - Check if the current user follows artists or users.
///
/// - `CurrentUserPlaylists`
///   Maximum batch size: 50.
pub enum BatchLimits {
    Albums,                      // 20
    AlbumTracks,                 // 50
    GetSavedAlbums,              // 50
    ModifyCurrentUserAlbums, // 20 This applies to 'save albums for current user', 'remove users' saved albums', & 'check users saved albums'
    NewReleases,             // 50
    Artists,                 // 50
    ArtistAlbums,            //50
    Audiobooks,              //50
    AudiobookChapters,       //50
    SavedAudiobooks,         //50
    ModifyCurrentUserAudiobook, //50 This applies to 'save audiobook for current user', 'remove users' saved audiobook', & 'check users saved audiobook'
    BrowseCategories,           //50
    AudiobooksChapters,         //50
    Episodes,                   //50
    GetSavedEpisodes,           //50
    ModifyCurrentUserEpisodes, //50 This applies to 'save episode for current user', 'remove users' saved episode', & 'check users saved episode'
    RecentlyPlayed,            //50
    PlaylistItems,             //50
    ModifyPlaylistItems, //100 This applies to 'add playlists items', 'remove playlists items', & 'update playlists items'
    UserPlaylists,       //50 This applies to a public user and private/current user
    GetPlaylists,        //50 This applies to category and featured playlists
    SearchItem,          //50
    GetShows,            //50
    GetShowEpisodes,     //50
    GetUserSavedShows,   //50
    ModifyCurrentUserShows, //50 This applies to 'save show for current user', 'remove user's saved show', & 'check user's saved show'
    Tracks,                 //50
    GetSavedTracks,         //50
    ModifyCurrentUserTracks, //50 This applies to 'save track for current user', 'remove user's saved track', & 'check user's saved track'
    TracksAudioFeatures,     //100
    Recommendations,         //100
    CurrentUserTopItems,     //50
    CurrentUserFollowedArtists, //50
    ModifyWhoCurrentUserFollows, //50 This applies to 'follow artists or users', 'unfollow artists or users', & 'check if current user follows artists or users'
    CurrentUserPlaylists,        //50
}

impl BatchLimits {
    /// Retrieves the batch limit for an API operation corresponding to a specific `BatchLimits` variant.
    ///
    /// This function returns the maximum allowed number of items that can be processed in a single batch
    /// for the given operation represented by the `BatchLimits` enum. Each variant of the enum corresponds
    /// to a specific API operation, and the batch size is predefined as per the operation's specifications.
    ///
    /// # Returns
    /// - A `usize` value representing the batch limit for the corresponding `BatchLimits` variant.
    ///
    /// # Example
    /// ```
    /// use spotify_assistant_core::enums::validation::BatchLimits;
    /// match BatchLimits::Albums.get_limit() {
    ///     20 => println!("The batch limit for Albums is 20."),
    ///     _ => println!("Other batch limit."),
    /// }
    /// ```
    ///
    /// # Supported Variants and Limits
    /// - `BatchLimits::Albums` -> 20
    /// - `BatchLimits::AlbumTracks` -> 50
    /// - `BatchLimits::GetSavedAlbums` -> 50
    /// - `BatchLimits::ModifyCurrentUserAlbums` -> 20
    /// - `BatchLimits::NewReleases` -> 50
    /// - `BatchLimits::Artists` -> 50
    /// - `BatchLimits::ArtistAlbums` -> 50
    /// - `BatchLimits::Audiobooks` -> 50
    /// - `BatchLimits::AudiobookChapters` -> 50
    /// - `BatchLimits::SavedAudiobooks` -> 50
    /// - `BatchLimits::ModifyCurrentUserAudiobook` -> 50
    /// - `BatchLimits::BrowseCategories` -> 50
    /// - `BatchLimits::AudiobooksChapters` -> 50
    /// - `BatchLimits::Episodes` -> 50
    /// - `BatchLimits::GetSavedEpisodes` -> 50
    /// - `BatchLimits::ModifyCurrentUserEpisodes` -> 50
    /// - `BatchLimits::RecentlyPlayed` -> 50
    /// - `BatchLimits::PlaylistItems` -> 50
    /// - `BatchLimits::ModifyPlaylistItems` -> 100
    /// - `BatchLimits::UserPlaylists` -> 50
    /// - `BatchLimits::GetPlaylists` -> 50
    /// - `BatchLimits::SearchItem` -> 50
    /// - `BatchLimits::GetShows` -> 50
    /// - `BatchLimits::GetShowEpisodes` -> 50
    /// - `BatchLimits::GetUserSavedShows` -> 50
    /// - `BatchLimits::ModifyCurrentUserShows` -> 50
    /// - `BatchLimits::Tracks` -> 50
    /// - `BatchLimits::GetSavedTracks` -> 50
    /// - `BatchLimits::ModifyCurrentUserTracks` -> 50
    /// - `BatchLimits::TracksAudioFeatures` -> 100
    /// - `BatchLimits::Recommendations` -> 100
    /// - `BatchLimits::CurrentUserTopItems` -> 50
    /// - `BatchLimits::CurrentUserFollowedArtists` -> 50
    /// - `BatchLimits::ModifyWhoCurrentUserFollows` -> 50
    /// - `BatchLimits::CurrentUserPlaylists` -> 50
    ///
    pub fn get_limit(&self) -> usize {
        match self {
            BatchLimits::Albums => 20usize,
            BatchLimits::AlbumTracks => 50usize,
            BatchLimits::GetSavedAlbums => 50usize,
            BatchLimits::ModifyCurrentUserAlbums => 20usize,
            BatchLimits::NewReleases => 50usize,
            BatchLimits::Artists => 50usize,
            BatchLimits::ArtistAlbums => 50usize,
            BatchLimits::Audiobooks => 50usize,
            BatchLimits::AudiobookChapters => 50usize,
            BatchLimits::SavedAudiobooks => 50usize,
            BatchLimits::ModifyCurrentUserAudiobook => 50usize,
            BatchLimits::BrowseCategories => 50usize,
            BatchLimits::AudiobooksChapters => 50usize,
            BatchLimits::Episodes => 50usize,
            BatchLimits::GetSavedEpisodes => 50usize,
            BatchLimits::ModifyCurrentUserEpisodes => 50usize,
            BatchLimits::RecentlyPlayed => 50usize,
            BatchLimits::PlaylistItems => 50usize,
            BatchLimits::ModifyPlaylistItems => 100usize,
            BatchLimits::UserPlaylists => 50usize,
            BatchLimits::GetPlaylists => 50usize,
            BatchLimits::SearchItem => 50usize,
            BatchLimits::GetShows => 50usize,
            BatchLimits::GetShowEpisodes => 50usize,
            BatchLimits::GetUserSavedShows => 50usize,
            BatchLimits::ModifyCurrentUserShows => 50usize,
            BatchLimits::Tracks => 50usize,
            BatchLimits::GetSavedTracks => 50usize,
            BatchLimits::ModifyCurrentUserTracks => 50usize,
            BatchLimits::TracksAudioFeatures => 100usize,
            BatchLimits::Recommendations => 100usize,
            BatchLimits::CurrentUserTopItems => 50usize,
            BatchLimits::CurrentUserFollowedArtists => 50usize,
            BatchLimits::ModifyWhoCurrentUserFollows => 50usize,
            BatchLimits::CurrentUserPlaylists => 50usize,
        }
    }

    /// Determines whether the specified batch size is valid based on the current batch limit.
    ///
    /// # Parameters
    /// - `wrapped`: An `Option<usize>` representing the size of the batch to validate. If `None` is provided, the batch is considered invalid.
    ///
    /// # Returns
    /// - `true` if the provided batch size is within the allowed limits for the current variant of `BatchLimits`.
    /// - `false` if the batch size exceeds the allowed limit, or if `wrapped` is `None`.
    ///
    /// # Behavior
    /// - Each variant of the `BatchLimits` enum has a specific upper limit for batch sizes.
    /// - This function checks that the given batch size does not exceed the limit associated with the current variant.
    ///
    /// # Example
    /// ```rust
    /// use spotify_assistant_core::enums::validation::BatchLimits;
    /// let limit = BatchLimits::Albums;
    /// assert_eq!(limit.is_valid(Some(10)), true);  // Within the valid limit
    /// assert_eq!(limit.is_valid(Some(25)), false); // Exceeds limit for Albums
    /// assert_eq!(limit.is_valid(None), false);     // None is invalid
    /// ```
    ///
    /// # Supported Variants
    /// - `BatchLimits::Albums`: Maximum size 20.
    /// - `BatchLimits::AlbumTracks`: Maximum size 50.
    /// - `BatchLimits::ModifyPlaylistItems`: Maximum size 100.
    /// - `BatchLimits::Recommendations`: Maximum size 100.
    /// - (and many other variants with a maximum size of 50, as listed in the match block).
    ///
    /// # Notes
    /// - The batch size limits vary based on the use case associated with the `BatchLimits` variant.
    /// - Ensure that the input is a valid `usize` and within acceptable ranges to prevent unexpected `false` results.
    pub fn is_valid(&self, wrapped: Option<usize>) -> bool {
        if wrapped.is_none() {
            false
        } else if let Some(vector_size) = wrapped {
            match self {
                BatchLimits::Albums => vector_size <= 20usize,
                BatchLimits::AlbumTracks => vector_size <= 50usize,
                BatchLimits::GetSavedAlbums => vector_size <= 50usize,
                BatchLimits::ModifyCurrentUserAlbums => vector_size <= 20usize,
                BatchLimits::NewReleases => vector_size <= 50usize,
                BatchLimits::Artists => vector_size <= 50usize,
                BatchLimits::ArtistAlbums => vector_size <= 50usize,
                BatchLimits::Audiobooks => vector_size <= 50usize,
                BatchLimits::AudiobookChapters => vector_size <= 50usize,
                BatchLimits::SavedAudiobooks => vector_size <= 50usize,
                BatchLimits::ModifyCurrentUserAudiobook => vector_size <= 50usize,
                BatchLimits::BrowseCategories => vector_size <= 50usize,
                BatchLimits::AudiobooksChapters => vector_size <= 50usize,
                BatchLimits::Episodes => vector_size <= 50usize,
                BatchLimits::GetSavedEpisodes => vector_size <= 50usize,
                BatchLimits::ModifyCurrentUserEpisodes => vector_size <= 50usize,
                BatchLimits::RecentlyPlayed => vector_size <= 50usize,
                BatchLimits::PlaylistItems => vector_size <= 50usize,
                BatchLimits::ModifyPlaylistItems => vector_size <= 100usize,
                BatchLimits::UserPlaylists => vector_size <= 50usize,
                BatchLimits::GetPlaylists => vector_size <= 50usize,
                BatchLimits::SearchItem => vector_size <= 50usize,
                BatchLimits::GetShows => vector_size <= 50usize,
                BatchLimits::GetShowEpisodes => vector_size <= 50usize,
                BatchLimits::GetUserSavedShows => vector_size <= 50usize,
                BatchLimits::ModifyCurrentUserShows => vector_size <= 50usize,
                BatchLimits::Tracks => vector_size <= 50usize,
                BatchLimits::GetSavedTracks => vector_size <= 50usize,
                BatchLimits::ModifyCurrentUserTracks => vector_size <= 50usize,
                BatchLimits::TracksAudioFeatures => vector_size <= 100usize,
                BatchLimits::Recommendations => vector_size <= 100usize,
                BatchLimits::CurrentUserTopItems => vector_size <= 50usize,
                BatchLimits::CurrentUserFollowedArtists => vector_size <= 50usize,
                BatchLimits::ModifyWhoCurrentUserFollows => vector_size <= 50usize,
                BatchLimits::CurrentUserPlaylists => vector_size <= 50usize,
            }
        } else {
            false
        }
    }
}

/// An enumeration representing different times of the day.
///
/// # Variants
///
/// * `Morning`
///   - Represents the period of time from sunrise until 12:00 PM.
///
/// * `Afternoon`
///   - Represents the period of time from 12:00 PM until approximately 5:00 PM.
///
/// * `Evening`
///   - Represents the period of time from approximately 5:00 PM until sunset or early night.
///
/// * `Night`
///   - Represents the period of time from sunset or late evening until sunrise.
///
/// # Traits
///
/// * `Debug`
///   - Allows formatting the `TimeOfDay` variants for debugging purposes.
///
/// * `PartialEq`
///   - Enables comparison of `TimeOfDay` variants for equality and inequality.
///
/// # Examples
///
/// ```rust
/// use spotify_assistant_core::enums::validation::TimeOfDay;
/// let morning = TimeOfDay::Morning;
/// assert_eq!(morning, TimeOfDay::Morning);
/// println!("{:?}", morning); // Output: Morning
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum TimeOfDay {
    Morning,
    Afternoon,
    Evening,
    Night,
}

impl TimeOfDay {
    /// Determines the `TimeOfDay` variant based on the provided hour.
    ///
    /// # Parameters
    /// - `hour`: An unsigned 32-bit integer representing the hour of the day (0 through 23).
    ///
    /// # Returns
    /// - A `TimeOfDay` enum instance corresponding to the provided hour:
    ///   - `6..=11` maps to `TimeOfDay::Morning`.
    ///   - `12..=17` maps to `TimeOfDay::Afternoon`.
    ///   - `18..=23` maps to `TimeOfDay::Evening`.
    ///   - `0..=5` maps to `TimeOfDay::Night`.
    ///
    /// # Note
    /// If an out-of-range hour value is provided, the function defaults to returning `TimeOfDay::Morning`.
    ///
    /// # Examples
    /// ```
    /// use spotify_assistant_core::enums::validation::TimeOfDay;
    /// let morning = TimeOfDay::from_hour(8);
    /// assert_eq!(morning, TimeOfDay::Morning);
    ///
    /// let evening = TimeOfDay::from_hour(20);
    /// assert_eq!(evening, TimeOfDay::Evening);
    ///
    /// let default = TimeOfDay::from_hour(25); // Out of range
    /// assert_eq!(default, TimeOfDay::Morning);
    /// ```
    ///
    /// # Panics
    /// This function does not panic.
    pub fn from_hour(hour: u32) -> Self {
        match hour {
            6..=11 => TimeOfDay::Morning,
            12..=17 => TimeOfDay::Afternoon,
            18..=23 => TimeOfDay::Evening,
            0..=5 => TimeOfDay::Night,
            _ => TimeOfDay::Morning,
        }
    }

    /// Converts a `TimeOfDay` instance into its corresponding string representation.
    ///
    /// # Returns
    /// A `String` representing the time of day. Specifically:
    /// - `"morning"` for `TimeOfDay::Morning`
    /// - `"afternoon"` for `TimeOfDay::Afternoon`
    /// - `"evening"` for `TimeOfDay::Evening`
    /// - `"night"` for `TimeOfDay::Night`
    ///
    /// # Example
    /// ```
    /// use spotify_assistant_core::enums::validation::TimeOfDay;
    /// let time_of_day = TimeOfDay::Morning;
    /// assert_eq!(time_of_day.string(), "morning".to_string());
    /// ```
    pub fn string(&self) -> String {
        match self {
            TimeOfDay::Morning => "morning".to_string(),
            TimeOfDay::Afternoon => "afternoon".to_string(),
            TimeOfDay::Evening => "evening".to_string(),
            TimeOfDay::Night => "night".to_string(),
        }
    }

    /// Determines if the current `TimeOfDay` instance is `Morning`.
    ///
    /// # Returns
    ///
    /// * `true` - if the `TimeOfDay` variant is `Morning`.
    /// * `false` - otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use spotify_assistant_core::enums::validation::TimeOfDay;
    /// let time_of_day = TimeOfDay::Morning;
    /// assert!(time_of_day.is_morning());
    ///
    /// let time_of_day = TimeOfDay::Evening;
    /// assert!(!time_of_day.is_morning());
    /// ```
    pub fn is_morning(&self) -> bool {
        matches!(self, TimeOfDay::Morning)
    }

    /// Checks if the `TimeOfDay` instance represents the afternoon period.
    ///
    /// # Returns
    ///
    /// * `true` if the instance is equal to `TimeOfDay::Afternoon`.
    /// * `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use spotify_assistant_core::enums::validation::TimeOfDay;
    /// let time = TimeOfDay::Afternoon;
    /// assert!(time.is_afternoon());
    ///
    /// let time = TimeOfDay::Morning;
    /// assert!(!time.is_afternoon());
    /// ```
    pub fn is_afternoon(&self) -> bool {
        matches!(self, TimeOfDay::Afternoon)
    }

    /// Determines if the current `TimeOfDay` instance represents the evening.
    ///
    /// # Returns
    /// A boolean value:
    /// - `true` if the `TimeOfDay` is `Evening`.
    /// - `false` otherwise.
    ///
    /// # Example
    /// ```
    /// use spotify_assistant_core::enums::validation::TimeOfDay;
    /// let evening = TimeOfDay::Evening;
    /// let morning = TimeOfDay::Morning;
    ///
    /// assert!(evening.is_evening());
    /// assert!(!morning.is_evening());
    /// ```
    pub fn is_evening(&self) -> bool {
        matches!(self, TimeOfDay::Evening)
    }

    /// Checks if the current `TimeOfDay` instance represents the nighttime.
    ///
    /// # Returns
    /// * `true` - If the `TimeOfDay` instance is `Night`.
    /// * `false` - Otherwise.
    ///
    /// # Examples
    /// ```
    /// use spotify_assistant_core::enums::validation::TimeOfDay;
    /// let time = TimeOfDay::Night;
    /// assert!(time.is_night());
    ///
    /// let time = TimeOfDay::Morning;
    /// assert!(!time.is_night());
    /// ```
    pub fn is_night(&self) -> bool {
        matches!(self, TimeOfDay::Night)
    }
}
