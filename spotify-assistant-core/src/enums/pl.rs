use std::borrow::Cow;

use crate::traits::file_readers::ConfigReader;
use crate::utilities::filesystem::constants::Constants;
use rspotify::model::PlaylistId;
use tracing::{debug, debug_span};

/// Enum `PlaylistType`
///
/// This enumeration represents different types of playlists available within the application. Each variant specifies a unique
/// type of playlist, which may have different characteristics, data, or purposes. These playlist types are predefined as follows:
///
/// Variants:
/// - `StockRR`: Represents a stock playlist with round-robin functionality. Typically used for default or system-generated playlists.
/// - `MyRR`: Represents a user-defined playlist with round-robin functionality.
/// - `Top24`: Refers to a specific top playlist for the year 24. This could represent a curated or most popular selection for that year.
/// - `Top23`: Represents the top playlist for the year 23.
/// - `Top22`: Represents the top playlist for the year 22.
/// - `Top21`: Represents the top playlist for the year 21.
/// - `Top20`: Represents the top playlist for the year 20.
/// - `Top19`: Represents the top playlist for the year 19.
/// - `Top18`: Represents the top playlist for the year 18.
///
/// Usage of this enum allows for clear categorization and handling of playlists across the application.
#[derive(Debug)]
pub enum PlaylistType {
    StockRR,
    MyRR,
    MyLaggingRR,
}

impl PlaylistType {
    /// Retrieves the static `PlaylistId` associated with a specific `PlaylistType` by loading
    /// environment variables, extracting the corresponding playlist ID, and creating a `PlaylistId`
    /// from it.
    ///
    /// # Workflow
    ///
    /// 1. It attempts to load environment configurations from a `.env` file located at
    ///    `ProjectFiles::DotEnv.path()`.
    ///    - If the file cannot be found or there is an issue, the function continues execution.
    /// 2. Depending on the `PlaylistType` variant, it selects the appropriate environment variable name:
    ///    - `PlaylistType::StockRR` -> `"RELEASE_RADAR_ID"`
    ///    - `PlaylistType::MyRR` -> `"MY_RELEASE_RADAR_ID"`
    ///    - `PlaylistType::Top24` -> `"TOP_24_ID"`
    ///    - `PlaylistType::Top23` -> `"TOP_23_ID"`
    ///    - `PlaylistType::Top22` -> `"TOP_22_ID"`
    ///    - `PlaylistType::Top21` -> `"TOP_21_ID"`
    ///    - `PlaylistType::Top20` -> `"TOP_20_ID"`
    ///    - `PlaylistType::Top19` -> `"TOP_19_ID"`
    ///    - `PlaylistType::Top18` -> `"TOP_18_ID"`
    /// 3. It retrieves the value of the selected environment variable using `std::env::var`.
    ///    - If the environment variable is not found, the function panics with the error message:
    ///      `"Error: The MY_RELEASE_RADAR_ID environmental variable was not found".`
    /// 4. It creates a `PlaylistId` from this retrieved ID using `PlaylistId::from_id(Cow::from(rr_id))`.
    ///    - If the creation fails, it panics with the error message:
    ///      `"Error: The PlaylistId could not be created from the playlists ID".`
    /// 5. Converts the resulting `PlaylistId` into a static version using `into_static()` and returns it.
    ///
    /// # Returns
    ///
    /// A `'static` `PlaylistId` representing the playlist corresponding to the current `PlaylistType`
    /// variant.
    ///
    /// # Panics
    ///
    /// - If the required environment variable for the specified `PlaylistType` is not found,
    ///   the function will panic with a clear error message.
    /// - If a `PlaylistId` cannot be created from the retrieved playlist ID, the function will panic
    ///   with a descriptive error message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use spotify_assistant_core::enums::pl::PlaylistType;
    /// let playlist_type = PlaylistType::MyRR;
    /// let playlist_id = playlist_type.get_id();
    /// println!("Playlist ID: {:?}", playlist_id);
    /// ```
    ///
    /// Ensure the required environment variables are set or accessible through a `.env` file
    /// before calling this method.
    pub fn get_id(&self) -> PlaylistId<'static> {
        let _pl_id_span = debug_span!("get_pl_id").entered();
        let constants = Constants::new();
        let playlist_key = self.get_constant_key();
        let constant_playlist_id = constants.playlist_id(playlist_key);
        debug!(id_variant = ?self, "Checking constants file for id: {:?}", playlist_key);
        let pl_id = PlaylistId::from_id(Cow::from(constant_playlist_id))
            .expect("Error: The PlaylistId could not be created from the playlists ID");
        pl_id.into_static()
    }
    fn get_constant_key(&self) -> &'static str {
        match self {
            PlaylistType::StockRR => "stock",
            PlaylistType::MyRR => "custom",
            PlaylistType::MyLaggingRR => "lagging",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspotify::model::Id;

    #[test]
    fn test_get_id() {
        let stock_rr = PlaylistType::StockRR;
        let my_rr = PlaylistType::MyRR;
        let stock_id = stock_rr.get_id();
        let my_id = my_rr.get_id();

        assert_eq!(
            stock_id.id().to_string(),
            "3WuaniG4xcoEXAH3ZBmbqX".to_string()
        );
        assert_eq!(my_id.id().to_string(), "46mIugmIiN2HYVwAwlaBAr".to_string());
    }
}
