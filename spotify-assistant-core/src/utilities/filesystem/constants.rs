use crate::enums::fs::ProjectDirectories;
use crate::traits::file_readers::ConfigReader;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Constants {
    ids: ConstIDs,
}
impl ConfigReader for Constants {
    fn file_path() -> PathBuf {
        let config_path = ProjectDirectories::Config.path();
        config_path.join("constants.toml")
    }

    fn new() -> Constants {
        match fs::read_to_string(Self::file_path()) {
            Ok(string) => toml::from_str(&string).unwrap_or_else(|err| {
                println!("{:?}", Self::file_path());
                panic!(
                    "Error deserializing toml string into a usable constants configuration: {err:?}"
                )
            }),
            Err(err) => {
                println!("{:?}", Self::file_path());
                panic!("Error reading the constants file: {err:?}")
            }
        }
    }
}

impl Constants {
    pub fn playlist_id(&self, name: &str) -> String {
        let playlist = self
            .ids
            .playlists
            .iter()
            .find(|p| {
                let key = match name {
                    "stock" => "stock_release_radar",
                    "custom" => "custom_release_radar",
                    "lagging" => "lagging_release_radar",
                    _ => panic!("Playlist {} not found in constants.toml", name),
                };
                key == p.name
            })
            .unwrap();
        playlist.id.clone()
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ConstIDs {
    playlists: HashSet<ConstPlaylist>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash)]
struct ConstPlaylist {
    name: String,
    id: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{constants_toml, with_test_environment};

    #[test]
    fn constants_load_from_test_environment() {
        with_test_environment(|env| {
            fs::write(env.config_file("constants.toml"), constants_toml(env))
                .expect("failed to write test constants");

            let constants = Constants::new();

            assert_eq!(constants.playlist_id("stock"), "3WuaniG4xcoEXAH3ZBmbqX");

            assert_eq!(constants.playlist_id("custom"), "46mIugmIiN2HYVwAwlaBAr");
        });
    }
}
