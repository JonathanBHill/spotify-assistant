use crate::enums::fs::ProjectDirectories;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    general: General,
    behavior: Behavior,
    cli: Cli,
    paths: Paths,
    preferences: Preferences,
    spotify: Spotify,
}
impl Configuration {
    fn configuration_file_path() -> PathBuf {
        let config_path = ProjectDirectories::Config.path();
        config_path.join("config.toml")
    }
    pub fn new() -> Configuration {
        match fs::read_to_string(Self::configuration_file_path()) {
            Ok(string) => match toml::from_str(&string) {
                Ok(configuration) => configuration,
                Err(err) => panic!("Error deserializing toml string into a usable configuration: {:?}", err),
            },
            Err(err) => panic!("Error reading the configuratino file: {:?}", err),
        }
    }
    pub fn general(&self) -> General {
        self.general.clone()
    }
    pub fn behavior(&self) -> Behavior {
        self.behavior.clone()
    }
    pub fn cli(&self) -> Cli {
        self.cli.clone()
    }
    pub fn paths(&self) -> Paths {
        self.paths.clone()
    }
    pub fn preferences(&self) -> Preferences {
        self.preferences.clone()
    }
    pub fn spotify(&self) -> Spotify {
        self.spotify.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct General {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Behavior {
    duplicates: Duplicates,
}
impl Behavior {
    pub fn duplicates(&self) -> Duplicates {
        self.duplicates.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Duplicates {
    custom_release_radar: bool,
    query_playlist_for_blacklist: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cli {
    default_shell: String,
    artist_id_format: String, // ? may want to change to an enum with variants URI & ID
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Paths {
    files: Files,
    folders: Folders,
}
impl Paths {
    pub fn files(&self) -> Files {
        self.files.clone()
    }
    pub fn folders(&self) -> Folders {
        self.folders.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Files {
    env: PathBuf,
    blacklist: PathBuf,
    config: PathBuf,
    top_tracks: PathBuf,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Folders {
    databases: PathBuf,
    listening_history: PathBuf,
    spotify_account_data: PathBuf,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Preferences {
    length_of_recently_played: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Spotify {
    default_user: String,
    content_ids: ContentIDs,
}
impl Spotify {
    pub fn content_ids(&self) -> ContentIDs {
        self.content_ids.clone()
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContentIDs {
    stock_release_radar: String,
    custom_release_radar: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read() {
        let x = Configuration::new();
        println!("{:#?}", x)
    }
}
