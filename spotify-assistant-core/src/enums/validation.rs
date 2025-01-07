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

#[derive(Debug, PartialEq)]
pub enum TimeOfDay {
    Morning,
    Afternoon,
    Evening,
    Night,
}

impl TimeOfDay {
    pub fn from_hour(hour: u32) -> Self {
        match hour {
            6..=11 => TimeOfDay::Morning,
            12..=17 => TimeOfDay::Afternoon,
            18..=23 => TimeOfDay::Evening,
            0..=5 => TimeOfDay::Night,
            _ => TimeOfDay::Morning,
        }
    }
    pub fn string(&self) -> String {
        match self {
            TimeOfDay::Morning => "morning".to_string(),
            TimeOfDay::Afternoon => "afternoon".to_string(),
            TimeOfDay::Evening => "evening".to_string(),
            TimeOfDay::Night => "night".to_string(),
        }
    }
    pub fn is_morning(&self) -> bool {
        matches!(self, TimeOfDay::Morning)
    }
    pub fn is_afternoon(&self) -> bool {
        matches!(self, TimeOfDay::Afternoon)
    }
    pub fn is_evening(&self) -> bool {
        matches!(self, TimeOfDay::Evening)
    }
    pub fn is_night(&self) -> bool {
        matches!(self, TimeOfDay::Night)
    }
}
