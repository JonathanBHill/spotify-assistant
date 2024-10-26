use tracing::{info, Level};

use crate::core::queries::user::UserData;

#[cfg(any(feature = "sqlite", feature = "alldb"))]
pub struct UserDB {
    pub id: String,
    pub name: String,
    pub spotify_url: String,
    pub href: String,
    pub image: String,
    pub email: String,
    pub plan: String,
    pub followers: u32,
    pub explicit_filter_enabled: bool,
    pub explicit_filter_locked: bool,
    pub last_updated: chrono::NaiveDateTime,
}
impl UserDB {
    pub async fn new() -> Self {
        let span = tracing::span!(Level::INFO, "UserDB.new");
        let _enter = span.enter();
        info!("Initializing a UserDB object");

        let user_data = UserData::new().await;
        let dt = chrono::Local::now().format("%m/%d/%Y %H:%M").to_string();
        UserDB {
            id: user_data.user_id(None),
            name: user_data.user_id(Some("display_name".to_string())),
            spotify_url: user_data.urls().get("spotify").unwrap_or(&"".to_string()).clone(),
            href: user_data.urls().get("href").unwrap_or(&"".to_string()).clone(),
            image: user_data.image(),
            email: user_data.user_id(Some("email".to_string())),
            plan: user_data.product_as_string(),
            followers: user_data.followers(),
            explicit_filter_enabled: *user_data.explicit_content().get("filter_enabled").unwrap_or(&false),
            explicit_filter_locked: *user_data.explicit_content().get("filter_locked").unwrap_or(&false),
            last_updated: chrono::naive::NaiveDateTime::parse_from_str(&dt, "%m/%d/%Y %H:%M").unwrap(),
        }
    }
}
