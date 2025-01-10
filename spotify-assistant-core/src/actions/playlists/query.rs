use std::collections::HashSet;
use std::ops::Index;

use rspotify::clients::BaseClient;
use rspotify::model::{FullPlaylist, PlaylistId, SearchResult, SearchType, SimplifiedPlaylist};
use rspotify::{scopes, AuthCodeSpotify, ClientError};

use crate::traits::apis::Api;

pub struct PlaylistQuery {
    pub client: AuthCodeSpotify,
}

impl Api for PlaylistQuery {
    fn select_scopes() -> HashSet<String> {
        scopes!(
            "playlists-read-private",
            "playlists-read-collaborative",
            "playlists-modify-public",
            "playlists-modify-private"
        )
    }
}
impl PlaylistQuery {
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
