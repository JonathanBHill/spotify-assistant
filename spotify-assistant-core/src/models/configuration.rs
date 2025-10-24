use crate::enums::fs::ProjectDirectories;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// The `Configuration` structure represents the application's configuration settings,
/// organized into several distinct sections as fields. Each field contains settings
/// related to a specific aspect of the application's functionality.
///
/// This structure is serializable and deserializable using the `Serialize` and `Deserialize`
/// traits provided by the `serde` crate. It also implements the `Debug` trait for easily
/// printing the contents in a formatted manner.
///
/// # Fields
/// - `general` (`General`): Stores general configuration settings for the application.
/// - `behavior` (`Behavior`): Contains configuration settings related to the application's behavior.
/// - `cli` (`Cli`): Holds configuration values specific to the command-line interface features.
/// - `paths` (`Paths`): Represents paths and directory-related settings used by the application.
/// - `preferences` (`Preferences`): Captures user preferences and customizable options.
/// - `spotify` (`Spotify`): Manages Spotify-related configuration, such as authentication or API integration.
///
/// This structure is commonly used for loading, modifying, and saving application setup
/// specified by users or defaults across different components of the application.
#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    general: General,
    behavior: Behavior,
    cli: Cli,
    paths: Paths,
    preferences: Preferences,
    spotify: Spotify,
}
impl Default for Configuration {
    /// Provides the default implementation for the `Configuration` struct by attempting to load it
    /// from a configuration file. The file is expected to be in TOML format.
    ///
    /// # Behavior
    /// - The method reads the configuration file from the path returned by `Self::configuration_file_path()`.
    /// - If the file is found and read successfully, it attempts to deserialize its content into a `Configuration` object.
    /// - If the deserialization succeeds, the resulting `Configuration` is returned.
    /// - If an error occurs while reading the file or deserializing its content, the function will panic with an error message.
    ///
    /// # Panics
    /// - Panics if the configuration file cannot be read. The error message will include the reason for the failure.
    /// - Panics if the content of the configuration file cannot be deserialized into a `Configuration` object. The error message will describe the deserialization error.
    ///
    /// # Returns
    /// - A valid `Configuration` object if the file is successfully read and deserialized.
    ///
    /// # Example
    /// ```rust
    /// use spotify_assistant_core::models::configuration::Configuration;
    /// let config = Configuration::default();
    /// ```
    ///
    /// # Remarks
    /// - Ensure that the file path returned by `Self::configuration_file_path()` points to a valid TOML file
    ///   with the expected structure for the `Configuration` object.
    /// - Consider handling the errors gracefully if the possibility of the file being missing or corrupted exists.
    fn default() -> Configuration {
        match fs::read_to_string(Self::configuration_file_path()) {
            Ok(string) => toml::from_str(&string).unwrap_or_else(|err| {
                panic!("Error deserializing toml string into a usable configuration: {err:?}")
            }),
            Err(err) => panic!("Error reading the configuratino file: {err:?}"),
        }
    }
}
impl Configuration {
    /// Returns the file path to the configuration file.
    ///
    /// This function utilizes the `directories` crate to determine the operating system-specific
    /// directory for storing configuration files. It retrieves the base configuration directory
    /// (e.g., `.config` on Linux) using the `ProjectDirectories::Config` constant and appends
    /// "config.toml" to create the full configuration file path.
    ///
    /// # Returns
    ///
    /// A `PathBuf` representing the absolute path to the `config.toml` file.
    ///
    /// # Note
    /// Ensure that the `ProjectDirectories` instance is properly initialized in the scope where
    /// this function is called. This function assumes the `directories` crate is used to manage
    /// project directories.
    ///
    /// # Dependencies
    /// This function requires the `directories` crate to work.
    ///
    /// # Panics
    /// This function does not explicitly handle errors and may panic if, for example,
    /// the `directories` crate fails to resolve a valid configuration path.
    fn configuration_file_path() -> PathBuf {
        let config_path = ProjectDirectories::Config.path();
        config_path.join("config.toml")
    }

    /// Retrieves a cloned instance of the `General` struct associated with the current object.
    ///
    /// # Returns
    ///
    /// * `General` - A cloned instance of the `General` struct.
    ///
    /// # Example
    ///
    /// ```
    /// use spotify_assistant_core::models::configuration::{Configuration};
    /// let instance = Configuration::default();
    /// let general_instance = instance.general();
    /// ```
    ///
    /// Note: The `General` struct must implement the `Clone` trait for this method to work.
    pub fn general(&self) -> General {
        self.general.clone()
    }

    /// Retrieves a cloned instance of the `behavior` field.
    ///
    /// This method returns a clone of the `Behavior` object stored in the `self.behavior`
    /// field of the struct. Cloning ensures that the original value remains unchanged,
    /// and the caller receives a separate instance.
    ///
    /// # Returns
    ///
    /// * `Behavior` - A clone of the `behavior` field.
    ///
    /// # Example
    ///
    /// ```
    /// use spotify_assistant_core::models::configuration::{Configuration};
    /// let instance = Configuration::default();
    /// let cloned_behavior = instance.behavior();
    /// ```
    pub fn behavior(&self) -> Behavior {
        self.behavior.clone()
    }

    /// Returns a cloned instance of the `Cli` associated with the current object.
    ///
    /// # Returns
    ///
    /// A cloned `Cli` instance.
    ///
    /// # Example
    ///
    /// ```
    /// use spotify_assistant_core::models::configuration::{Configuration};
    /// let instance = Configuration::default();
    /// let cli_instance = instance.cli();
    /// ```
    pub fn cli(&self) -> Cli {
        self.cli.clone()
    }

    /// Returns a clone of the `Paths` object associated with the current instance.
    ///
    /// # Returns
    ///
    /// A `Paths` object, which is a clone of the `paths` field in the current instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use spotify_assistant_core::models::configuration::{Configuration};
    /// let instance = Configuration::default();
    /// let paths_clone = instance.paths();
    /// ```
    ///
    /// # Notes
    ///
    /// - The `paths` object in the current instance remains unchanged.
    /// - Cloning can be expensive if the `Paths` structure is large. Use this method judiciously in performance-critical code.
    pub fn paths(&self) -> Paths {
        self.paths.clone()
    }

    /// Retrieves the user's preferences.
    ///
    /// This function returns a clone of the `Preferences` associated with the current instance.
    /// As the preferences are cloned, any modifications made to the returned `Preferences` object
    /// will not affect the original preferences stored in the instance.
    ///
    /// # Returns
    ///
    /// A `Preferences` object representing the user's current preferences.
    ///
    /// # Example
    ///
    /// ```rust
    /// use spotify_assistant_core::models::configuration::{Configuration};
    /// let instance = Configuration::default();
    /// let user_preferences = instance.preferences();
    /// // The preferences can now be read or modified independently of the original user instance.
    /// ```
    ///
    /// # Note
    /// Ensure that cloning the `Preferences` structure is necessary for your use case to
    /// avoid unintended performance overhead due to cloning large or complex structures.
    pub fn preferences(&self) -> Preferences {
        self.preferences.clone()
    }

    /// Returns a clone of the `Spotify` instance associated with the current object.
    ///
    /// This method provides access to the `Spotify` instance by returning a cloned
    /// copy of it. Since the returned instance is a clone, you can use it independently
    /// without affecting the original `Spotify` instance stored within the object.
    ///
    /// # Returns
    ///
    /// A cloned instance of the `Spotify` object.
    ///
    /// # Example
    ///
    /// ```
    /// use spotify_assistant_core::models::configuration::{Configuration};
    /// let instance = Configuration::default();
    /// let spotify_clone = instance.spotify();
    /// // `spotify_clone` can now be used independently of `original_instance`
    /// ```
    pub fn spotify(&self) -> Spotify {
        self.spotify.clone()
    }
}

/// The `General` struct is an empty data structure, which is derived with the following traits:
///
/// - `Serialize`: Enables the `General` struct to be converted into a format suitable for serialization, such as JSON or other data formats.
/// - `Deserialize`: Allows the `General` struct to be reconstructed from serialized data.
/// - `Debug`: Provides functionality for formatting the `General` struct using the `{:?}` formatter, which is useful for debugging purposes.
/// - `Clone`: Allows for creating a duplicate of the `General` struct.
///
/// This struct is currently empty but can serve as a placeholder or base for handling general-purpose data or functionality
/// in future implementations.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct General {}

/// A struct representing a `Behavior`, which includes the handling of duplicate items.
///
/// This struct is marked with `#[derive(Serialize, Deserialize, Debug, Clone)]` to provide the following functionalities:
/// - Serialization and deserialization of the struct for use in formats such as JSON, TOML, etc., via the `serde` library.
/// - Debug output capability, allowing instances of this struct to be formatted using the `fmt::Debug` trait.
/// - Clone capability, enabling the creation of duplicate instances of this struct.
///
/// # Fields
/// - `duplicates: Duplicates`
///   Represents the handling of duplicates within the behavior. The exact structure or functionality of `Duplicates`
///   is not detailed here and is expected to be defined elsewhere in the codebase or imported.
///
/// # Usage
/// This struct can be used to configure or represent behaviors with specific duplicate handling mechanisms,
/// depending on the implementation or constraints defined in the `Duplicates` type.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Behavior {
    duplicates: Duplicates,
}

impl Behavior {
    /// Returns a clone of the `Duplicates` associated with the current object.
    ///
    /// The `duplicates` method provides access to the `Duplicates` instance stored
    /// within the object. It returns a cloned copy of the `Duplicates`, ensuring that
    /// the original instance remains unaltered. This allows safe access to the duplicate
    /// data without transferring ownership.
    ///
    /// # Returns
    ///
    /// A `Duplicates` instance cloned from the `self.duplicates` field.
    pub fn duplicates(&self) -> Duplicates {
        self.duplicates.clone()
    }
}

/// Represents the configuration options for handling duplicate items in playlists.
///
/// The `Duplicates` struct contains two fields that determine specific behaviors for managing duplicates.
///
/// # Fields
///
/// * `custom_release_radar` - A boolean field indicating whether custom logic should be applied to filter duplicates
///   in the release radar playlist. If `true`, custom handling for duplicates is activated.
///
/// * `query_playlist_for_blacklist` - A boolean field indicating whether playlists should be queried to check
///   for blacklist entries when managing duplicates. If `true`, this enables additional filtering logic
///   against a blacklist during duplicate checks.
///
/// This struct is derived from `Serialize`, `Deserialize`, `Debug`, and `Clone` to enable serialization,
/// debugging, and cloning operations where necessary.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Duplicates {
    custom_release_radar: bool,
    query_playlist_for_blacklist: bool,
}

/// Represents the structure for command-line interface (CLI) configuration settings.
///
/// # Fields
///
/// * `default_shell` (`String`) -
///   Specifies the default shell to be used by the CLI. This is usually a command-line
///   interpreter (e.g., bash, zsh, powershell).
///
/// * `artist_id_format` (`String`) -
///   Defines the format used for artist identification.
///   This is subject to future enhancements where this may be changed to an enumeration
///   with variants such as `URI` and `ID` for explicit determination of the format.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cli {
    default_shell: String,
    artist_id_format: String, // ? may want to change to an enum with variants URI & ID
}

/// Represents the structure for managing file and folder paths.
///
/// The `Paths` struct is composed of two primary fields:
/// - `files`: Represents files and their associated data through the `Files` type.
/// - `folders`: Represents folders and their associated data through the `Folders` type.
///
/// This struct derives the following traits:
/// - `Serialize` and `Deserialize` from `serde` for enabling serialization and deserialization.
/// - `Debug` for formatting the struct with its values for debugging purposes.
/// - `Clone` for creating a duplicate instance of the struct.
///
/// # Fields
/// - `files` (`Files`): A field representing the file-related paths or data.
/// - `folders` (`Folders`): A field representing the folder-related paths or data.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Paths {
    files: Files,
    folders: Folders,
}
impl Paths {
    /// Returns a clone of the `Files` instance associated with the current object.
    ///
    /// This method provides access to the `Files` data by cloning it, thereby ensuring
    /// the integrity of the original `Files` instance is maintained. Cloning allows
    /// the caller to work with an independent copy of the `Files` data without affecting
    /// the state of the original object.
    ///
    /// # Returns
    ///
    /// A cloned instance of the `Files` object.
    pub fn files(&self) -> Files {
        self.files.clone()
    }

    /// Returns a clone of the `folders` field.
    ///
    /// This function provides access to the `folders` field from the current instance,
    /// returning a cloned copy of the `Folders` object. Cloning is performed to ensure
    /// that the original data remains unchanged and safe from unintended modifications.
    ///
    /// # Returns
    /// * `Folders` - A cloned copy of the `folders` object contained within the instance.
    pub fn folders(&self) -> Folders {
        self.folders.clone()
    }
}

/// Represents a collection of file paths required for the application.
///
/// This struct holds file paths that correspond to different
/// configuration and data files essential for the program's functionality.
///
/// # Fields
///
/// * `env` - A `PathBuf` pointing to the environment configuration file (e.g., `.env` file).
/// * `blacklist` - A `PathBuf` pointing to a blacklist file (e.g., a file containing items to exclude).
/// * `config` - A `PathBuf` pointing to the main configuration file.
/// * `top_tracks` - A `PathBuf` pointing to a file containing a list of top tracks (e.g., user or system generated data).
///
/// # Derives
///
/// * `Serialize` - Enables the struct to be serialized into formats like JSON or TOML.
/// * `Deserialize` - Enables the struct to be deserialized from formats like JSON or TOML.
/// * `Debug` - Provides formatting for easier debugging of struct instances.
/// * `Clone` - Allows the struct to be cloned, creating a deep copy of the instance.
///
/// # Example
///
/// ```rust
/// use serde::{Serialize, Deserialize};
/// use std::path::PathBuf;
///
/// #[derive(Serialize, Deserialize, Debug, Clone)]
/// pub struct Files {
///     env: PathBuf,
///     blacklist: PathBuf,
///     config: PathBuf,
///     top_tracks: PathBuf,
/// }
///
/// // Example of initializing a `Files` instance:
/// let file_paths = Files {
///     env: PathBuf::from(".env"),
///     blacklist: PathBuf::from("blacklist.txt"),
///     config: PathBuf::from("config.toml"),
///     top_tracks: PathBuf::from("top_tracks.json"),
/// };
///
/// println!("{:?}", file_paths);
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Files {
    env: PathBuf,
    blacklist: PathBuf,
    config: PathBuf,
    top_tracks: PathBuf,
}

/// The `Folders` struct represents the directory paths used by the application.
///
/// # Fields
///
/// - `databases` (*PathBuf*):
///   Path to the folder where the application's databases are stored.
///
/// - `listening_history` (*PathBuf*):
///   Path to the folder where the user's listening history is stored.
///
/// - `spotify_account_data` (*PathBuf*):
///   Path to the folder where Spotify account-related data is stored.
///
/// This struct derives:
/// - `Serialize`: Allows the struct to be serialized into formats like JSON or TOML.
/// - `Deserialize`: Allows the struct to be deserialized from formats like JSON or TOML.
/// - `Debug`: Enables the struct to be formatted using the `{:?}` formatter.
/// - `Clone`: Enables the struct to be cloned for creating duplicate instances.
///
/// # Example
/// ```
/// use serde::{Serialize, Deserialize};
/// use std::path::PathBuf;
///
/// #[derive(Serialize, Deserialize, Debug, Clone)]
/// pub struct Folders {
///     databases: PathBuf,
///     listening_history: PathBuf,
///     spotify_account_data: PathBuf,
/// }
///
/// let folders = Folders {
///     databases: PathBuf::from("/path/to/databases"),
///     listening_history: PathBuf::from("/path/to/listening_history"),
///     spotify_account_data: PathBuf::from("/path/to/spotify_account_data"),
/// };
///
/// println!("{:?}", folders);
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Folders {
    databases: PathBuf,
    listening_history: PathBuf,
    spotify_account_data: PathBuf,
}

/// Represents a user's preferences related to application settings.
///
/// This struct is used to configure the behavior of features within the application,
/// specifically for settings related to recently played items.
///
/// # Fields
/// - `length_of_recently_played` (`i32`): The number of items to retain in the
///   recently played list. This value determines the maximum number of entries
///   stored in the list.
///
/// # Traits
/// - `Serialize`: Allows the struct to be serialized, which can be useful for saving or transmitting
///   user preferences in JSON or other formats.
/// - `Deserialize`: Allows the struct to be deserialized, enabling loading of user preferences
///   from saved data.
/// - `Debug`: Facilitates debugging by allowing the struct to be formatted using the `{:?}` formatter.
/// - `Clone`: Allows for creating a duplicate instance of the struct.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Preferences {
    length_of_recently_played: i32,
}

/// The `Spotify` struct represents a configuration or data structure related to Spotify.
///
/// # Fields
///
/// - `default_user` (`String`): The default user associated with Spotify.
/// - `content_ids` (`ContentIDs`): A collection of content IDs related to Spotify.
///
/// # Traits
///
/// This struct derives the following traits:
/// - `Serialize`: Allows the `Spotify` struct to be serialized into formats like JSON.
/// - `Deserialize`: Allows the `Spotify` struct to be deserialized from formats like JSON.
/// - `Debug`: Enables formatting of the `Spotify` struct using the `{:?}` formatter for debugging purposes.
/// - `Clone`: Creates a copy of the `Spotify` struct.
///
/// # Example
///
/// ```rust
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Serialize, Deserialize, Debug, Clone)]
/// pub struct ContentIDs {
///     album_id: String,
///     track_id: String,
/// }
///
/// #[derive(Serialize, Deserialize, Debug, Clone)]
/// pub struct Spotify {
///     default_user: String,
///     content_ids: ContentIDs,
/// }
///
/// let content_ids = ContentIDs {
///     album_id: String::from("album123"),
///     track_id: String::from("track456"),
/// };
///
/// let spotify = Spotify {
///     default_user: String::from("user123"),
///     content_ids,
/// };
///
/// println!("{:?}", spotify);
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Spotify {
    default_user: String,
    content_ids: ContentIDs,
}

impl Spotify {
    /// Retrieves a clone of the `content_ids` field.
    ///
    /// This method returns a cloned instance of the `ContentIDs`,
    /// which is a collection or representation of content identifiers associated with the current object.
    ///
    /// # Returns
    ///
    /// A `ContentIDs` instance containing the content identifiers.
    ///
    /// # Note
    /// Since the method clones the `content_ids`, modifications to the returned value
    /// will not affect the original data in the parent object.
    pub fn content_ids(&self) -> ContentIDs {
        self.content_ids.clone()
    }
}
/// A struct representing a collection of content identifiers.
///
/// The `ContentIDs` struct is used to store unique identifiers for various types of release radars.
///
/// # Fields
///
/// * `stock_release_radar` - A `String` that represents the identifier for the stock release radar.
/// * `custom_release_radar` - A `String` that represents the identifier for the custom release radar.
///
/// # Traits
///
/// * `Serialize` - Enables serialization of this struct into formats such as JSON.
/// * `Deserialize` - Allows deserialization of this struct from formats such as JSON.
/// * `Debug` - Enables printing the struct using the `{:?}` formatter.
/// * `Clone` - Allows creating a deep copy of this struct.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContentIDs {
    stock_release_radar: String,
    custom_release_radar: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{
        configuration_toml, invalid_configuration_toml, TestEnvironment, ENV_MUTEX,
    };

    fn configuration_fixture(env: &TestEnvironment) -> Configuration {
        let toml = configuration_toml(env);
        toml::from_str(&toml).expect("fixture configuration should deserialize")
    }

    #[test]
    fn configuration_serializes_and_deserializes_round_trip() {
        let env = unsafe { TestEnvironment::new() };
        let configuration = configuration_fixture(&env);

        let serialized =
            toml::to_string_pretty(&configuration).expect("serialization should succeed");
        let deserialized: Configuration =
            toml::from_str(&serialized).expect("round-trip deserialization should succeed");

        assert_eq!(
            toml::to_string_pretty(&deserialized).unwrap(),
            serialized,
            "Round-trip serialization should be lossless"
        );
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn configuration_default_reads_from_temp_environment() {
        let _guard = ENV_MUTEX.lock().unwrap_or_else(|err| err.into_inner());
        let env = unsafe { TestEnvironment::new() };
        let toml = configuration_toml(&env);
        fs::write(env.config_file("config.toml"), &toml)
            .expect("failed to write configuration fixture");

        let expected = configuration_fixture(&env);
        let loaded = Configuration::default();

        assert_eq!(
            toml::to_string_pretty(&loaded).unwrap(),
            toml::to_string_pretty(&expected).unwrap(),
            "Loaded configuration should match fixture"
        );
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn configuration_default_panics_on_malformed_toml() {
        let _guard = ENV_MUTEX.lock().unwrap_or_else(|err| err.into_inner());
        let env = unsafe { TestEnvironment::new() };
        fs::write(env.config_file("config.toml"), invalid_configuration_toml())
            .expect("failed to write malformed configuration fixture");

        let result = std::panic::catch_unwind(Configuration::default);
        assert!(result.is_err(), "Malformed TOML should trigger a panic");
    }
}
