use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::path::PathBuf;

use rspotify::model::SimplifiedArtist;
use serde::{Deserialize, Serialize};
use tracing::{event, span, Level};
use unicode_normalization::char::is_combining_mark;
use unicode_normalization::UnicodeNormalization;

use crate::enums::fs::ProjectDirectories;

/// Represents an artist to be blacklisted in the application.
///
/// Each artist is identified by a `name` and `id`.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash, Clone)]
pub struct BlacklistArtist {
    name: String,
    id: String,
}

impl BlacklistArtist {
    /// Creates a new instance of `BlacklistArtist`.
    ///
    /// # Arguments
    /// * `name` - The name of the artist.
    /// * `id` - The unique ID of the artist.
    ///
    /// # Returns
    /// A new `BlacklistArtist` instance.
    pub fn new(name: String, id: String) -> Self {
        let id = id.split(':').collect::<Vec<&str>>()[2].to_string();
        BlacklistArtist { name, id }
    }

    /// Returns the name of the artist.
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// Returns the ID of the artist.
    pub fn id(&self) -> String {
        self.id.clone()
    }
}

/// Represents the underlying data for the blacklist, containing a set of blacklisted artists.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlacklistData {
    artists: HashSet<BlacklistArtist>,
}

impl BlacklistData {
    /// Returns the set of all blacklisted artists.
    fn artists(&self) -> HashSet<BlacklistArtist> {
        self.artists.clone()
    }

    /// Adds an artist to the blacklist.
    ///
    /// # Arguments
    /// * `artist` - The `BlacklistArtist` to add to the blacklist.
    ///
    /// # Returns
    /// `true` if the artist was successfully added, `false` if the artist was already present in the set.
    fn add(&mut self, artist: BlacklistArtist) -> bool {
        self.artists.insert(artist)
    }
}

/// Represents the blacklist, with methods to manage and persist blacklisted artists.
#[derive(Serialize, Deserialize, Debug)]
pub struct Blacklist {
    blacklist: BlacklistData,
}

impl Blacklist {
    /// Creates a new `Blacklist` instance by reading from the persisted blacklist file.
    ///
    /// # Returns
    /// A new `Blacklist` instance containing the persisted blacklist data.
    pub fn new() -> Self {
        let blacklist = Blacklist::read_blacklist().blacklist;
        Blacklist { blacklist }
    }

    /// Determines the file path for the blacklist file.
    ///
    /// # Returns
    /// A `PathBuf` representing the file path of the blacklist file.
    fn blacklist_file_path() -> PathBuf {
        let config_path = ProjectDirectories::Config.path();
        config_path.join("blacklist.toml")
    }

    /// Writes the current blacklist data to the file for persistence.
    ///
    /// If an error occurs during serialization or writing, this method panics.
    fn update_blacklist(&self) {
        let span = span!(Level::INFO, "Blacklist.write_self");
        let _enter = span.enter();
        event!(Level::INFO, "Writing the blacklist to the file.");

        let toml_string = match toml::to_string_pretty(self) {
            Ok(string) => string,
            Err(err) => {
                panic!("Could not write the blacklist to the file.\nError: {:?}", err)
            }
        };

        match fs::write(Self::blacklist_file_path(), toml_string) {
            Ok(_) => (),
            Err(err) => {
                panic!("Could not write the blacklist to the file.\nError: {:?}", err)
            }
        };
    }

    /// Adds a single artist to the blacklist.
    ///
    /// Updates the blacklist file after adding the artist.
    ///
    /// # Arguments
    /// * `artist` - The `BlacklistArtist` to add to the blacklist.
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
    /// Updates the blacklist file after adding all the artists.
    ///
    /// # Arguments
    /// * `artists` - A vector of `BlacklistArtist` instances to add to the blacklist.
    pub fn add_artists(&mut self, artists: Vec<BlacklistArtist>) {
        artists.iter().for_each(|artist| {
            self.add_artist(artist.clone());
        });
    }

    /// Removes an artist by name and ID.
    ///
    /// Updates the blacklist file after removing the artist.
    ///
    /// # Arguments
    /// * `target_name` - The name of the artist to remove.
    /// * `target_id` - The ID of the artist to remove.
    pub fn remove_artist(&mut self, target_name: &str, target_id: &str) {
        let artist_to_remove = BlacklistArtist {
            name: target_name.to_string(),
            id: target_id.to_string(),
        };

        if self.blacklist.artists.remove(&artist_to_remove) {
            println!("Artist removed: {:?}", artist_to_remove);
        } else {
            println!("Artist not found: {:?}", artist_to_remove);
        }

        self.update_blacklist();
    }

    /// Reads the persisted blacklist from the file.
    ///
    /// # Returns
    /// A `Blacklist` instance created from the file data.
    ///
    /// # Panics
    /// If the file cannot be read or deserialized.
    fn read_blacklist() -> Blacklist {
        match fs::read_to_string(Self::blacklist_file_path()) {
            Ok(string) => match toml::from_str(&string) {
                Ok(blacklist) => blacklist,
                Err(err) => panic!("Error deserializing toml string into the blacklist: {:?}", err),
            },
            Err(err) => panic!("Error reading the blacklist file: {:?}", err),
        }
    }

    /// Compares the normalized names of two artists (input vs file) for matches.
    ///
    /// # Arguments
    /// * `console_input` - The artist's name provided by the user.
    /// * `name_from_file` - The name of the artist from the blacklist file.
    ///
    /// # Returns
    /// `true` if the names are equal after normalization, `false` otherwise.
    pub fn are_names_equal(&self, console_input: &str, name_from_file: String) -> bool {
        Self::normalize(console_input.to_string()) == Self::normalize(name_from_file)
    }

    /// Normalizes a name by removing diacritical marks and converting it to lowercase.
    ///
    /// # Arguments
    /// * `name` - The name to normalize.
    ///
    /// # Returns
    /// A normalized `String`.
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

    /// Prints all the artists in the blacklist to the terminal.
    pub fn print_blacklist(&self) {
        let artists = self.blacklist.artists();
        artists.iter().enumerate().for_each(|(index, artist)| {
            println!(" {}: {} ({})", index + 1, artist.name(), artist.id());
        });
    }

    /// Returns a set of all blacklisted artists.
    pub fn artists(&self) -> HashSet<BlacklistArtist> {
        self.blacklist.artists()
    }

    /// Allows the user to select an artist to remove from the blacklist via an interactive prompt.
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

    /// Allows the user to select an artist to add to the blacklist from a provided list of artists grouped by album.
    ///
    /// # Arguments
    /// * `artists_with_album` - A map of album names to lists of artists associated with those albums.
    ///
    /// # Returns
    /// An `Option<BlacklistArtist>` representing the selected artist, or `None` if no artist was selected.
    pub fn select_artist_to_add_by_album(
        &mut self,
        artists_with_album: HashMap<String, Vec<SimplifiedArtist>>,
    ) -> Option<BlacklistArtist> {
        let artists = artists_with_album;
        let mut formatted_options = vec!["Cancel".to_string()];
        artists.iter().for_each(|(album_name, artists)| {
            artists.iter().enumerate().for_each(|(index, artist)| {
                if index == 0 {
                    formatted_options.push(format!("1: {} ({:?}) | Album - {}", artist.name, artist.id.clone().expect("Could not get ID").to_string(), album_name))
                } else {
                    formatted_options.push(format!("\t{}: {} ({:?})", index + 1, artist.name, artist.id.clone().expect("Could not get ID").to_string()))
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
            let split_option = selected_option_string.split(&[':', '\"', '(']).collect::<Vec<&str>>();
            println!("Full: {:?}\nSplit: {:?}", selected_option_string, split_option);
            Some(BlacklistArtist {
                name: split_option[1].trim().to_string(),
                id: split_option[5].trim().to_string()
            })
        }
    }
}
