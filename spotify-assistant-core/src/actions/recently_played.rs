use std::collections::HashSet;

use chrono::{DateTime, Utc};
use rspotify::clients::OAuthClient;
use rspotify::model::{CursorBasedPage, PlayHistory, TimeLimits};
use rspotify::{scopes, AuthCodeSpotify};

use crate::traits::apis::Api;

pub struct UserListeningHistory {
    client: AuthCodeSpotify,
    tracks: Vec<PlayHistory>,
    next: String,
}

impl Api for UserListeningHistory {
    fn select_scopes() -> HashSet<String> {
        scopes!(
            "user-read-recently-played"
        )
    }
}

impl UserListeningHistory {
    pub async fn new() -> Self {
        let client = Self::set_up_client(false, Some(Self::select_scopes())).await;
        let results = match client.current_user_recently_played(Some(50), None).await {
            Ok(results) => { results }
            Err(err) => { panic!("Could not retrieve your listening history: {:?}", err) }
        };
        let next = match results.next {
            Some(string) => { string }
            None => {
                eprintln!("Error: No more pages to retrieve.");
                "".to_string()
            }
        };
        UserListeningHistory {
            client,
            tracks: results.items.into_iter().rev().collect::<Vec<PlayHistory>>(),
            next,
        }
    }
    pub fn tracks(&self) -> Vec<PlayHistory> {
        self.tracks.clone()
    }
    pub async fn next(&self) -> CursorBasedPage<PlayHistory> {
        let next = self.get_time_limit();
        let results = match self.client.current_user_recently_played(Some(50), Some(next)).await {
            Ok(results) => { results }
            Err(err) => { panic!("Could not retrieve your listening history: {:?}", err) }
        };
        results
    }
    fn get_time_limit(&self) -> TimeLimits {
        let timestamp = self.next
                            .split("before=").collect::<Vec<&str>>()[1]
            .split("&limit=").collect::<Vec<&str>>()[0]
            .to_string();
        let timestamp_parsed = match timestamp.parse() {
            Ok(timestamp) => { timestamp }
            Err(_) => { panic!("Could not parse timestamp") }
        };
        let datetime: DateTime<Utc> = match DateTime::from_timestamp(timestamp_parsed, 0) {
            Some(datetime) => { datetime }
            None => { panic!("Could not convert to DateTime") }
        };
        TimeLimits::Before(datetime)
    }
    pub async fn extend_history(&mut self, number_of_loops: u32) {
        for _ in 0..number_of_loops {
            let next_page = self.next().await;
            let tracks = next_page.items.into_iter().rev().collect::<Vec<PlayHistory>>();
            self.tracks.extend(tracks);
            self.next = match next_page.next {
                Some(next) => { next }
                None => { "".to_string() }
            };
        }
    }
}
