use std::collections::{HashMap, HashSet};

use rspotify::{AuthCodeSpotify, scopes};
use rspotify::clients::OAuthClient;
use rspotify::model::{FullTrack, Id, PrivateUser, SubscriptionLevel, TimeRange};
use tracing::{event, info, Level};

use crate::core::models::traits::Api;
use crate::core::utilities::recordkeeping::tracks::TracksIO;

#[derive(Debug)]
pub struct UserData {
    client: AuthCodeSpotify,
    pub user: PrivateUser,
}

impl Api for UserData {
    fn select_scopes() -> HashSet<String> {
        scopes!(
            "user-read-private",
            "user-read-email",
            "user-read-recently-played",
            "user-top-read"
        )
    }
}
impl UserData {
    pub async fn new() -> Self {
        let span = tracing::span!(Level::INFO, "UserData.new");
        let _enter = span.enter();
        let client = UserData::set_up_client(false, Some(UserData::select_scopes())).await;
        event!(Level::INFO, "User has been authenticated with client.");
        let user = client.current_user().await.ok().unwrap();
        event!(Level::INFO, "User data has been retrieved.");
        let user_data = UserData {
            client,
            user,
        };
        event!(Level::INFO, "User data has been initialized.");
        user_data
    }
    pub fn product(&self) -> SubscriptionLevel {
        self.user.product.unwrap_or(SubscriptionLevel::Free)
    }
    pub fn product_as_string(&self) -> String {
        let product = self.user.product.unwrap_or(SubscriptionLevel::Free);
        match product {
            SubscriptionLevel::Premium => { "Premium".to_string() },
            SubscriptionLevel::Free => { "Free".to_string() },
        }
    }
    pub fn urls(&self) -> HashMap<String, String> {
        let mut urls = HashMap::new();
        for (key, value) in self.user.external_urls.iter() {
            urls.insert(key.to_string(), value.to_string());
        }
        urls.insert("href".to_string(), self.user.href.clone());
        urls
    }
    pub fn image(&self) -> String {
        let images = self.user.images.clone().expect("Could not get the user's images");
        images[0].url.clone()
    }
    pub fn followers_as_string(&self) -> String {
        self.user.followers.clone().unwrap_or_default().total.to_string()
    }
    pub fn followers(&self) -> u32 {
        self.user.followers.clone().unwrap_or_default().total
    }
    pub fn user_id(&self, id_type: Option<String>) -> String {
        match id_type {
            Some(data_unwrapped) => {
                match data_unwrapped.as_str() {
                    "display_name" => self.user
                        .display_name
                        .clone()
                        .expect("Could not get display name")
                        .to_string(),
                    "email" => self.user.email
                        .clone()
                        .expect("Could not get email")
                        .to_string(),
                    _ => self.user.id.id().to_string(),
                }
            },
            None => self.user
                .display_name
                .clone()
                .expect("Could not get display name")
                .to_string(),
        }
    }
    pub fn explicit_content(&self) -> HashMap<&str, bool> {
        let mut explicit_settings = HashMap::new();
        match self.user.explicit_content.clone() {
            None => {unreachable!("Explicit content settings not found")},
            Some(explicit) => {
                explicit_settings.insert("filter_enabled", explicit.filter_enabled);
                explicit_settings.insert("filter_locked", explicit.filter_locked);
                explicit_settings
            }
        }
    }
    pub async fn recently_played(&self) -> Vec<(String, String, String)> {
        let x = self.client.current_user_recently_played(Some(50), None).await;
        let ux = x.expect("Issue unwrapping");
        let uxc = ux.items;
        // println!("{:?}, {:?}", ux.total, ux.limit);
        let items = uxc.iter().map(|track| {
            let track_name = track.track.name.clone();
            let context = match track.context.clone() {
                Some(context) => context._type.to_string(),
                None => "not known".to_string()
            };
            let x = track.played_at.naive_local().to_string();
            (track_name, context, x)
        }).collect::<Vec<(String, String, String)>>();
        items
    }
    pub async fn top_tracks(&self, save_to_file: bool) -> Vec<FullTrack> {
        let span = tracing::span!(Level::INFO, "UserData.top-tracks");
        let _enter = span.enter();
        
        let total_top_tracks = match self.client
            .current_user_top_tracks_manual(
                Some(TimeRange::ShortTerm),
                Some(1),
                None
            ).await {
            Ok(top_track) => top_track.total,
            Err(error) => panic!("Could not get top tracks: {:?}", error)
        };
        let mut top_vec = Vec::with_capacity(total_top_tracks as usize);
        let page_size = 50;
        let pages_no_remainder = (total_top_tracks / page_size) as i32;
        let pages = if total_top_tracks % page_size > 0 {
            info!("pages with remainder: {}", pages_no_remainder + 1);
            pages_no_remainder + 1
        } else {
            info!("pages w/o remainder: {pages_no_remainder}");
            pages_no_remainder
        };
        for page in 0..pages {
            let offset = page_size * page as u32;
            let multiplier = page_size as usize * page as usize;
            let offset_top_tracks = match self.client
                .current_user_top_tracks_manual(
                    Some(TimeRange::ShortTerm),
                    Some(page_size),
                    Some(offset)
                ).await {
                Ok(page) => page.items.into_iter(),
                Err(error) => panic!("{:?}", error)
            };
            for (index, track) in offset_top_tracks.enumerate() {
                let track_number = index + multiplier;
                top_vec.insert(track_number, track);
            }
            info!("Page {}/{} appended", page + 1, pages)
        }
        info!("A total of {} top tracks have been collected.", top_vec.len());
        if save_to_file {
            let io = TracksIO::new("toptracks".to_string());
            io.serialize(&top_vec);
        }
        top_vec
    }

    pub async fn playlists(&self) {
        let span = tracing::span!(Level::INFO, "UserData.playlists");
        let _enter = span.enter();
        let playlists = match self.client
            .current_user_playlists_manual(
                Some(1), None
            ).await {
            Ok(playlists) => playlists,
            Err(error) => panic!("Could not get playlists: {:?}", error)
        };
        playlists.items.iter().for_each(|playlist| {
            info!("{:?}", playlist.name);
        });
        info!("Total playlists: {}", playlists.total);
    }
}
