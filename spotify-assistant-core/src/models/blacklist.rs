use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;

use rspotify::model::SimplifiedArtist;
use serde::{Deserialize, Serialize};
use tracing::{Level, event, span};
use unicode_normalization::UnicodeNormalization;
use unicode_normalization::char::is_combining_mark;

use crate::enums::fs::ProjectDirectories;

/// Represents an artist who is blacklisted.
///
/// This struct is used to store information about a blacklisted artist,
/// including their name and unique identifier.
///
/// # Attributes
/// - `name` (String): The name of the blacklisted artist.
/// - `id` (String): The unique identifier of the blacklisted artist.
///
/// # Derives
/// - `Serialize`: Enables serialization of the `BlacklistArtist` structure,
///   allowing it to be converted into formats such as JSON.
/// - `Deserialize`: Enables deserialization of the `BlacklistArtist` structure,
///   allowing it to be created from formats such as JSON.
/// - `Debug`: Automatically formats the struct for debugging purposes.
/// - `Eq`: Allows for equality comparison of two `BlacklistArtist` instances.
/// - `PartialEq`: Enables partial equality comparison for `BlacklistArtist` instances.
/// - `Hash`: Makes the struct hashable, giving compatibility with hashed data structures (e.g., `HashMap` and `HashSet`).
/// - `Clone`: Allows the `BlacklistArtist` struct to be cloned.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash, Clone)]
pub struct BlacklistArtist {
    name: String,
    id: String,
}

impl BlacklistArtist {
    /// Creates a new `BlacklistArtist` instance.
    ///
    /// This function takes in the `name` and a raw `id` of the artist as `String` values.
    /// The `id` string is expected to follow a specific format that includes multiple
    /// components separated by colons (`:`). The function extracts the third component
    /// (index 2) of the `id` string (post-split) and uses it as the actual `id` for
    /// the `BlacklistArtist` instance.
    ///
    /// # Arguments
    ///
    /// * `name` - A `String` representing the name of the artist.
    /// * `id` - A `String` representing the raw ID of the artist. The ID is expected
    ///   to follow a specific format with colon-separated components.
    ///
    /// # Returns
    ///
    /// An instance of `BlacklistArtist` initialized with the provided `name` and
    /// the extracted `id`.
    ///
    /// # Panics
    ///
    /// This function will panic if the `id` string doesn't contain at least three
    /// colon-separated components (i.e., `id.split(':')` results in less than three elements).
    ///
    /// # Example
    /// ```no_run,ignore
    /// use spotify_assistant_core::models::blacklist::BlacklistArtist;
    /// let artist = BlacklistArtist::new(
    ///     "Artist Name".to_string(),
    ///     "some:prefix:actual_id".to_string(),
    /// );
    /// assert_eq!(artist.name(), "Artist Name");
    /// assert_eq!(artist.id(), "actual_id");
    /// ```
    pub fn new(name: String, id: String) -> Self {
        let id = id.split(':').collect::<Vec<&str>>()[2].to_string();
        BlacklistArtist { name, id }
    }

    pub fn new_from_artist(artist: &SimplifiedArtist) -> Self {
        let name = artist.name.clone();
        let uri = artist
            .id
            .clone()
            .expect("Could not obtain artist ID")
            .to_string();
        let id = uri.split(':').collect::<Vec<&str>>()[2].to_string();
        BlacklistArtist { name, id }
    }

    /// Returns a clone of the `name` field.
    ///
    /// This method provides access to the `name` field of the struct
    /// in the form of a new `String` instance. It clones the value of
    /// the `name` field to ensure that the internal value remains
    /// unmodified and its ownership is not transferred.
    ///
    /// # Returns
    ///
    /// A `String` containing the value of the `name` field.
    ///
    /// # Examples
    /// ```no_run,ignore
    /// struct Example {
    ///     name: String,
    /// }
    ///
    /// impl Example {
    ///     pub fn name(&self) -> String {
    ///         self.name.clone()
    ///     }
    /// }
    /// let example = Example { name: String::from("Alice") };
    /// assert_eq!(example.name(), "Alice".to_string());
    /// ```
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// Returns a clone of the `id` field of the current instance.
    ///
    /// # Returns
    ///
    /// A `String` representing the `id` of the instance. The returned value
    /// is a clone of the internal `id` property, ensuring the original data
    /// remains intact and unmodified.
    ///
    /// # Examples
    /// ```no_run,ignore
    /// struct Example {
    ///     id: String,
    /// }
    ///
    /// impl Example {
    ///     pub fn id(&self) -> String {
    ///         self.id.clone()
    ///     }
    /// }
    ///
    /// let instance = Example { id: String::from("abcd1234") };
    /// let id = instance.id();
    /// assert_eq!(id, "abcd1234");
    /// ```
    pub fn id(&self) -> String {
        self.id.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{ENV_MUTEX, TestEnvironment, blacklist_toml, invalid_blacklist_toml};
    use std::fs;

    fn blacklist_fixture() -> Blacklist {
        toml::from_str(&blacklist_toml()).expect("fixture blacklist should deserialize")
    }

    #[test]
    fn blacklist_serialization_round_trip_preserves_artists() {
        let blacklist = blacklist_fixture();
        let serialized = toml::to_string_pretty(&blacklist).expect("serialization should succeed");
        let deserialized: Blacklist =
            toml::from_str(&serialized).expect("round-trip deserialization should succeed");

        assert_eq!(
            deserialized.blacklist.artists, blacklist.blacklist.artists,
            "Artist sets should remain unchanged after round-trip"
        );
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn blacklist_default_reads_from_temp_environment() {
        let _guard = ENV_MUTEX.lock().unwrap_or_else(|err| err.into_inner());
        let env = unsafe { TestEnvironment::new() };
        let toml = blacklist_toml();
        fs::write(env.config_file("blacklist.toml"), &toml)
            .expect("failed to write blacklist fixture");

        let expected = blacklist_fixture();
        let loaded = Blacklist::default();

        assert_eq!(
            loaded.blacklist.artists, expected.blacklist.artists,
            "Loaded blacklist should match fixture"
        );
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn blacklist_default_panics_on_invalid_toml() {
        let _guard = ENV_MUTEX.lock().unwrap_or_else(|err| err.into_inner());
        let env = unsafe { TestEnvironment::new() };
        fs::write(env.config_file("blacklist.toml"), invalid_blacklist_toml())
            .expect("failed to write malformed blacklist fixture");

        let result = std::panic::catch_unwind(Blacklist::default);
        println!("{:?}", env.config_file("blacklist.toml"));
        assert!(
            result.is_err(),
            "Malformed blacklist TOML should trigger a panic"
        );
    }
}

/// A struct representing data for managing a blacklist of artists.
///
/// The `BlacklistData` struct contains a collection of uniquely identified
/// artists that have been added to a blacklist. This can be utilized
/// in applications to exclude or filter content associated with the listed artists.
///
/// ## Fields
///
/// - `artists`:
///   A `HashSet` containing `BlacklistArtist` elements. This ensures that each
///   artist in the blacklist is unique and allows for efficient lookups.
///
/// ## Traits
///
/// - `Serialize`: Enables the `BlacklistData` struct to be serialized, e.g., converting
///   it into JSON or other formats for storage or transmission.
///
/// - `Deserialize`: Allows the deserialization of the `BlacklistData` struct, e.g.,
///   reconstructing it from a JSON or other serialized format.
///
/// - `Debug`: Implements debugging capabilities, providing formatted representations
///   of `BlacklistData` for debugging and logging purposes.
///
/// - `Clone`: Enables the `BlacklistData` struct to be cloned, creating a complete
///   copy of its data.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlacklistData {
    artists: HashSet<BlacklistArtist>,
}

impl BlacklistData {
    /// Returns a `HashSet` containing the blacklist artists.
    ///
    /// This function retrieves a set of artists that are part of the blacklist,
    /// ensuring that the original collection remains unmodified by returning a clone.
    ///
    /// # Returns
    /// * `HashSet<BlacklistArtist>` - A clone of the `artists` set.
    ///
    /// Note: Cloning the `artists` set may have performance implications if the set is large.
    fn artists(&self) -> HashSet<BlacklistArtist> {
        self.artists.clone()
    }

    /// Adds a new artist to the blacklist.
    ///
    /// This method attempts to insert the specified `BlacklistArtist` into the blacklist.
    /// If the artist is successfully added (i.e., it was not already present in the blacklist),
    /// it returns `true`. If the artist is already in the blacklist, it returns `false`.
    ///
    /// # Parameters
    /// - `artist`: The `BlacklistArtist` instance to be added to the blacklist.
    ///
    /// # Returns
    /// - `true` if the artist was successfully added.
    /// - `false` if the artist was already present in the blacklist.
    fn add(&mut self, artist: BlacklistArtist) -> bool {
        self.artists.insert(artist)
    }
}

/// Represents a `Blacklist` structure that holds data regarding a blacklist.
///
/// This structure is designed to be serializable and deserializable using
/// Serde, making it convenient for converting to and from data formats such
/// as JSON or YAML. It also implements the `Debug` trait for easy debugging.
///
/// # Fields
/// - `blacklist`:
///   A field of type `BlacklistData` that contains the actual information
///   for the blacklist, where `BlacklistData` needs to be defined elsewhere
///   to specify the structure of the blacklist.
#[derive(Serialize, Deserialize, Debug)]
pub struct Blacklist {
    blacklist: BlacklistData,
}

impl Default for Blacklist {
    /// Creates a default instance of the `Blacklist` struct by reading a blacklist
    /// from an external source or predefined location.
    ///
    /// # Returns
    ///
    /// A new `Blacklist` instance with its `blacklist` field populated with data
    /// obtained from the `read_blacklist` method.
    ///
    /// # Example
    /// ```no_run,ignore
    /// use spotify_assistant_core::models::blacklist::Blacklist;
    /// let default_blacklist = Blacklist::default();
    /// ```
    ///
    /// # Notes
    ///
    /// The `read_blacklist` method is expected to provide the necessary data
    /// for constructing the `blacklist` field. Any errors occurring during this
    /// process should be handled inside the `read_blacklist` function.
    fn default() -> Self {
        Blacklist::read_blacklist()
    }
}
impl Blacklist {
    /// Retrieves the file path for the blacklist configuration file.
    ///
    /// This function constructs the file path for the blacklist file named `blacklist.toml`
    /// inside the configuration directory of the application. It utilizes the `config`
    /// directory as determined by the `ProjectDirectories` crate.
    ///
    /// # Returns
    ///
    /// A `PathBuf` that represents the absolute path to `blacklist.toml`.
    fn blacklist_file_path() -> PathBuf {
        let config_path = ProjectDirectories::Config.path();
        config_path.join("blacklist.toml")
    }

    /// Updates the blacklist by writing its current state to the file system.
    ///
    /// This function serializes the current state of the blacklist into a TOML string,
    /// and then writes this serialized string to the file specified by the `blacklist_file_path` method.
    /// Any errors encountered during serialization or file writing will cause the program to panic,
    /// as these are considered critical failures.
    ///
    /// ## Logging
    /// - An informational trace span (`Blacklist.write_self`) is created to indicate the context of this operation.
    /// - An informational event is logged to indicate that the blacklist is being written to a file.
    ///
    /// ## Panics
    /// - If the serialization of the blacklist into a TOML string fails, the function will panic
    ///   with a descriptive error message containing the serialization error.
    /// - If writing the serialized TOML string to the specified file path fails, the
    ///   function will also panic with a descriptive error message containing the file-system error.
    ///
    /// ## Usage
    /// This method is designed to ensure the persistence of the blacklist's state to disk. It is
    /// typically called when updates to the blacklist occur and need to be saved for future use.
    ///
    /// This will serialize the current state of the `Blacklist` object and persist it in a TOML
    /// format at the designated file path.
    fn update_blacklist(&self) {
        let span = span!(Level::INFO, "Blacklist.write_self");
        let _enter = span.enter();
        event!(Level::INFO, "Writing the blacklist to the file.");

        let toml_string = match toml::to_string_pretty(self) {
            Ok(string) => string,
            Err(err) => {
                panic!("Could not write the blacklist to the file.\nError: {err:?}")
            }
        };

        match fs::write(Self::blacklist_file_path(), toml_string) {
            Ok(_) => (),
            Err(err) => {
                panic!("Could not write the blacklist to the file.\nError: {err:?}")
            }
        };
    }

    /// Adds a new artist to the blacklist.
    ///
    /// This function accepts a `BlacklistArtist` instance and attempts to add it to the current blacklist.
    /// It uses a tracing span to log details about the operation and provides debug-level logs showing
    /// the blacklist before and after the addition. If the artist is already present in the blacklist,
    /// a message is printed to the console indicating that the entry already exists. After attempting to
    /// append the artist, it calls `update_blacklist` to apply any necessary updates.
    ///
    /// # Arguments
    ///
    /// * `artist` - A `BlacklistArtist` object representing the artist to be added to the blacklist.
    ///
    /// # Behavior
    ///
    /// - Logs the pre-addition state of the blacklist with a debug event.
    /// - If the artist already exists in the blacklist, prints a message to the console.
    /// - Logs the post-addition state of the blacklist with a debug event.
    /// - Calls the `update_blacklist` method to apply any changes.
    ///
    /// # Logging
    ///
    /// - `INFO` level span: Logs the operation of adding an artist to the blacklist.
    /// - `DEBUG` level event: Logs the state of the blacklist before and after the modification.
    ///
    /// # Example
    /// ```no_run,ignore
    /// use spotify_assistant_core::models::blacklist::{Blacklist, BlacklistArtist};
    /// let mut blacklist = Blacklist::default();
    /// let artist = BlacklistArtist::new("Artist Name".to_string(), "artist:id:12345".to_string());
    /// blacklist.add_artist(artist);
    /// ```
    pub fn add_artist(&mut self, artist: BlacklistArtist) {
        let span = span!(Level::INFO, "Blacklist.add_artist");
        let _enter = span.enter();
        event!(Level::DEBUG, "Original blacklist: {:?}", self.blacklist);
        if !self.blacklist.add(artist.clone()) {
            println!("Entry for {} already exists.", artist.name())
        };
        event!(Level::DEBUG, "Appended blacklist: {:?}", self.blacklist);
        self.update_blacklist();
    }

    /// Adds multiple artists to the blacklist.
    ///
    /// # Parameters
    /// - `artists`: A vector of `BlacklistArtist` items to be added to the blacklist.
    ///
    /// This method iterates over each artist in the provided vector and adds them
    /// to the blacklist by calling the internal `add_artist` method for each one.
    ///
    /// # Example
    /// ```no_run,ignore
    /// use spotify_assistant_core::models::blacklist::{Blacklist, BlacklistArtist};
    /// let mut blacklist = Blacklist::default();
    /// let artist1 = BlacklistArtist::new("Artist 1".to_string(), "artist:id:12345".to_string());
    /// let artist2 = BlacklistArtist::new("Artist 2".to_string(), "artist:id:67890".to_string());
    ///
    /// blacklist.add_artists(vec![artist1, artist2]);
    /// ```
    ///
    /// # Note
    /// Each artist is cloned before being added, which ensures the original vector
    /// remains unchanged.
    pub fn add_artists(&mut self, artists: Vec<BlacklistArtist>) {
        artists.iter().for_each(|artist| {
            self.add_artist(artist.clone());
        });
    }

    /// Removes an artist from the blacklist based on the provided name and ID.
    ///
    /// This method searches the `self.blacklist.artists` collection for an artist
    /// matching the specified name and ID. If the artist is found, it is removed
    /// from the blacklist. If the artist is not found, an appropriate message is
    /// logged. After the attempt to remove the artist (whether successful or not),
    /// the blacklist is updated by calling `self.update_blacklist()`.
    ///
    /// # Arguments
    ///
    /// * `target_name` - A reference to a string slice representing the name of the artist to be removed.
    /// * `target_id` - A reference to a string slice representing the ID of the artist to be removed.
    ///
    /// # Behavior
    ///
    /// * If the artist (matching both the given name and ID) exists in the blacklist,
    ///   they are removed, and a success message is printed to the console.
    /// * If the artist does not exist in the blacklist, a "not found" message is printed
    ///   to the console.
    /// * After attempting the removal operation, the `self.update_blacklist()` method is
    ///   called to apply any necessary updates to the blacklist state.
    ///
    /// # Example
    /// ```no_run,ignore
    /// use spotify_assistant_core::models::blacklist::{Blacklist, BlacklistArtist};
    /// let mut manager = Blacklist::default();
    /// let artist = BlacklistArtist::new("Artist Name".to_string(), "artist:id:12345".to_string());
    /// manager.add_artist(artist);
    /// manager.remove_artist("Artist Name", "12345"); // Removes the artist.
    /// manager.remove_artist("Nonexistent", "00000"); // Logs: Artist not found.
    /// ```
    ///
    /// Note: `BlacklistArtist` must implement `PartialEq` and `Hash` for this function
    /// to work as intended, as it relies on the ability to compare and hash artist instances.
    /// Additionally, `self.blacklist.artists` is assumed to be a `HashSet` or similar collection
    /// that supports the `.remove()` operation.
    ///
    /// # Side Effects
    ///
    /// - Modifies the `self.blacklist.artists` collection if the artist is found and removed.
    /// - Triggers an update to the blacklist via `self.update_blacklist()`.
    /// - Logs success or failure messages to the console.
    pub fn remove_artist(&mut self, target_name: &str, target_id: &str) {
        let artist_to_remove = BlacklistArtist {
            name: target_name.to_string(),
            id: target_id.to_string(),
        };

        if self.blacklist.artists.remove(&artist_to_remove) {
            println!("Artist removed: {artist_to_remove:?}");
        } else {
            println!("Artist not found: {artist_to_remove:?}");
        }

        self.update_blacklist();
    }

    /// Reads the blacklist from a file, deserializing it from a TOML string into a `Blacklist` object.
    ///
    /// # Returns
    ///
    /// * `Blacklist` - The deserialized blacklist object containing the necessary data.
    ///
    /// # Panics
    ///
    /// This function will panic in the following situations:
    ///
    /// 1. If the blacklist file cannot be read, it will panic with a message
    ///    indicating the error that occurred during the file read operation.
    /// 2. If the TOML string read from the file cannot be deserialized into a `Blacklist` object,
    ///    it will panic with a message indicating the deserialization error.
    ///
    /// # Behavior
    ///
    /// - The path to the blacklist file is determined by the `Self::blacklist_file_path()` function.
    /// - The file is read as a string using `fs::read_to_string`.
    /// - The string is then deserialized into a `Blacklist` object using `toml::from_str`.
    fn read_blacklist() -> Blacklist {
        match fs::read_to_string(Self::blacklist_file_path()) {
            Ok(string) => match toml::from_str(&string) {
                Ok(blacklist) => blacklist,
                Err(err) => panic!("Error deserializing toml string into the blacklist: {err:?}"),
            },
            Err(err) => panic!("Error reading the blacklist file: {err:?}"),
        }
    }

    ///
    /// Compares two names for equality after normalizing them.
    ///
    /// This function takes a name inputted from the console and a name retrieved from a file,
    /// normalizes both names using a standard normalization process, and checks if they are equal.
    ///
    /// # Arguments
    ///
    /// * `&self` - A reference to the instance of the structure or object this method belongs to.
    /// * `console_input` - A string slice representing the name input by the user via the console.
    /// * `name_from_file` - A `String` containing the name retrieved from a file.
    ///
    /// # Returns
    ///
    /// * A `bool` indicating whether the normalized `console_input` and `name_from_file` are equal.
    ///
    /// # Example
    /// ```no_run,ignore
    /// use spotify_assistant_core::models::blacklist::Blacklist;
    /// let instance = Blacklist::default();
    /// let is_equal = instance.are_names_equal("JohnDoe", String::from("john_doe"));
    /// assert!(is_equal);
    /// ```
    ///
    /// Note: This function relies on a `normalize` method to preprocess names.
    ///
    pub fn are_names_equal(&self, console_input: &str, name_from_file: String) -> bool {
        Self::normalize(console_input.to_string()) == Self::normalize(name_from_file)
    }

    /// Normalizes a given string by removing diacritical marks and converting all characters to lowercase.
    ///
    /// This function performs the following steps on the input string:
    /// - Decomposes Unicode characters into their base form (e.g., "é" becomes "e" and its diacritical mark).
    /// - Removes all diacritical marks (combining characters).
    /// - Converts all characters in the string to lowercase.
    ///
    /// # Arguments
    ///
    /// * `name` - A `String` that represents the input text to be normalized.
    ///
    /// # Returns
    ///
    /// This function returns a `String` with the normalized version of the `name` input.
    ///
    /// # Dependencies
    ///
    /// Ensure the `unicode-normalization` crate is included in your `Cargo.toml`:
    ///
    /// ```toml
    /// [dependencies]
    /// unicode-normalization = "0.1"
    /// ```
    fn normalize(name: String) -> String {
        let name = name.as_str();
        let remove_diacritics = |s: &str| {
            s.nfd() // Decompose Unicode
                .filter(|c| !is_combining_mark(*c)) // Remove diacritics
                .flat_map(char::to_lowercase) // Convert to lowercase
                .collect::<String>()
        };

        remove_diacritics(name)
    }

    /// Prints the list of blacklisted artists.
    ///
    /// This function retrieves the artists stored in the blacklist and
    /// displays them in the console. Each artist is displayed with an
    /// index (starting from 1), their name, and their unique ID.
    ///
    /// # Behavior
    /// - The function iterates over all blacklisted artists and prints their details.
    /// - The output is formatted as:
    ///   - <index><artist_name> (<artist_id>)
    /// - Example:
    ///   - ArtistName1 (12345)
    ///   - ArtistName2 (67890)
    ///
    /// # Note
    /// - This function assumes that the `blacklist` field in the containing struct is properly initialized
    ///   and has a method `artists()` that returns an iterable collection of artists.
    /// - Each `artist` is required to have `name()` and `id()` methods to retrieve relevant information.
    ///
    /// # Example
    /// ```no_run,ignore
    /// use spotify_assistant_core::models::blacklist::Blacklist;
    /// let blacklist = Blacklist::default();
    /// blacklist.print_blacklist();
    /// ```
    pub fn print_blacklist(&self) {
        let artists = self.blacklist.artists();
        artists.iter().enumerate().for_each(|(index, artist)| {
            println!(" {}: {} ({})", index + 1, artist.name(), artist.id());
        });
    }

    /// Retrieves a set of blacklisted artists.
    ///
    /// This function accesses the blacklist stored within the current instance and
    /// returns a `HashSet` containing all the blacklisted artists.
    ///
    /// # Returns
    ///
    /// A `HashSet<BlacklistArtist>` representing the collection of blacklisted artists.
    ///
    /// # Example
    /// ```no_run,ignore
    /// use spotify_assistant_core::models::blacklist::Blacklist;
    /// let instance = Blacklist::default();
    /// let blacklist = instance.artists();
    /// for artist in blacklist {
    ///     println!("{}", artist.name());
    /// }
    /// ```
    ///
    /// # Note
    /// The function delegates the operation to the `artists` method of the `blacklist`
    /// field.
    ///
    /// # Dependencies
    /// Ensure that the related `blacklist` field is properly initialized before calling this function.
    pub fn artists(&self) -> HashSet<BlacklistArtist> {
        self.blacklist.artists()
    }

    /// This method provides the user with a terminal-based interface to select an artist
    /// from the blacklist to be removed. The available artists are sorted alphabetically
    /// by name and presented in a selectable list.
    ///
    /// The method performs the following steps:
    /// 1. Retrieves the list of currently blacklisted artists.
    /// 2. Sorts the blacklisted artists alphabetically based on their names.
    /// 3. Prepares a vector of artist names for display, inserting a "Cancel" option at the top.
    /// 4. Presents a selection menu to the user via the `dialoguer::Select` interface, allowing them to
    ///    choose an artist to remove or cancel the operation.
    /// 5. If the "Cancel" option is chosen, no action is taken, and a message is printed.
    /// 6. If an artist is selected, the corresponding artist is removed from the blacklist by calling
    ///    the `remove_artist` method.
    ///
    /// # Behavior:
    /// - The "Cancel" option is always the first item in the list, with index 0.
    /// - If the "Cancel" option is selected, no modification is made to the blacklist, and a confirmation
    ///   message is displayed.
    /// - If a valid artist is selected, the method removes the artist from the blacklist using the artist's
    ///   name and ID and calls `self.remove_artist()`.
    ///
    /// # Example:
    /// ```no_run,ignore
    /// // Imagine `self` is an instance of the relevant struct that contains this method.
    /// // Calling this method will show a terminal menu for selecting an artist to remove:
    /// use spotify_assistant_core::models::blacklist::Blacklist;
    /// let mut blacklist = Blacklist::default();
    /// blacklist.select_artist_to_remove();
    /// // The user selects an artist, and the selected artist is removed from the blacklist.
    /// ```
    ///
    /// # Dependencies:
    /// - `dialoguer::Select` is used to present the selection menu in the terminal.
    /// - The `BlacklistArtist` struct is expected to provide `name()` and `id()` methods.
    ///
    /// # Note:
    /// - This method assumes the `artists()` method exists in `self` and returns an iterable collection of
    ///   `BlacklistArtist` objects.
    /// - The `remove_artist()` method is called internally with the selected artist's name and ID to complete
    ///   the removal process.
    ///
    /// # Errors:
    /// The method does not explicitly handle errors thrown by `dialoguer::Select`.
    /// If an error occurs (e.g., terminal input/output failure), it will panic.
    pub fn select_artist_to_remove(&mut self) {
        let artists = self.artists();
        let mut artist_vec = artists.iter().collect::<Vec<&BlacklistArtist>>();
        artist_vec.sort_by(|a, b| a.name().cmp(&b.name()));
        let mut artist_names = artist_vec
            .iter()
            .map(|artist| artist.name())
            .collect::<Vec<String>>();
        artist_names.insert(0, "Cancel".to_string());

        let selection = dialoguer::Select::new()
            .items(&artist_names)
            .default(0)
            .interact()
            .unwrap();

        if selection == 0 {
            println!("No artist was removed.");
        } else {
            let artist = artist_vec[selection - 1];
            self.remove_artist(&*artist.name(), &artist.id());
        }
    }

    /// A method for selecting an artist to add to a blacklist by iterating through a map of albums and their associated artists.
    ///
    /// # Parameters
    /// - `artists_with_album`: A `HashMap` where the key is a `String` representing the album name, and the value is a `Vec` of `SimplifiedArtist` objects representing the artists associated with the album.
    ///
    /// # Returns
    /// Returns an `Option<BlacklistArtist>`:
    /// - `Some(BlacklistArtist)` if an artist is selected.
    /// - `None` if no artist is selected (e.g., user cancels the selection).
    ///
    /// # Behavior
    /// 1. The method receives a `HashMap` of artists grouped by album.
    /// 2. An interactive CLI-based menu is displayed allowing the user to select an artist or cancel the selection.
    ///    - Each artist is labeled with a formatted string that includes the index, artist name, artist ID, and album name.
    ///    - "Cancel" is presented as the first option in the selection list.
    /// 3. If the user selects an artist:
    ///    - Extracts and parses the selected artist's details (`name` and `id`) from the menu using string splitting.
    ///    - Constructs a `BlacklistArtist` object with the parsed details.
    /// 4. If "Cancel" is selected (index 0 in the selection menu):
    ///    - A message is printed indicating that no artist was selected.
    ///    - Returns `None`.
    /// 5. If an artist is successfully selected, returns a `BlacklistArtist` object with the selected details.
    ///
    /// # Panics
    /// - Panics if the `artist.id` for any artist in the map is `None`.
    /// - Panics if the interaction with the user fails during the selection process.
    ///
    /// # Notes
    /// - This method is interactive and depends on the `dialoguer` crate for user input handling.
    /// - Ensure the `artists_with_album` contains valid artist data with non-`None` IDs to avoid panics.
    pub fn select_artist_to_add_by_album(
        &mut self,
        artists_with_album: HashMap<String, Vec<SimplifiedArtist>>,
    ) -> Option<BlacklistArtist> {
        let artists = artists_with_album;
        let mut formatted_options = vec!["Cancel".to_string()];
        artists.iter().for_each(|(album_name, artists)| {
            artists.iter().enumerate().for_each(|(index, artist)| {
                if index == 0 {
                    formatted_options.push(format!(
                        "1: {} ({:?}) | Album - {}",
                        artist.name,
                        artist.id.clone().expect("Could not get ID").to_string(),
                        album_name
                    ))
                } else {
                    formatted_options.push(format!(
                        "\t{}: {} ({:?})",
                        index + 1,
                        artist.name,
                        artist.id.clone().expect("Could not get ID").to_string()
                    ))
                }
            });
        });
        let selection = dialoguer::Select::new()
            .items(&formatted_options)
            .default(0)
            .interact()
            .unwrap();
        if selection == 0 {
            println!("No artist was selected.");
            None
        } else {
            let selected_option_string = &formatted_options[selection];
            let split_option = selected_option_string
                .split(&[':', '\"', '('])
                .collect::<Vec<&str>>();
            println!("Full: {selected_option_string:?}\nSplit: {split_option:?}");
            Some(BlacklistArtist {
                name: split_option[1].trim().to_string(),
                id: split_option[5].trim().to_string(),
            })
        }
    }
}
