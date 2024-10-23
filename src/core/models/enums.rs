use std::borrow::Cow;
use std::env;

use rspotify::model::PlaylistId;

use crate::core::enums::fs::ProjectFiles;

pub enum PlaylistType {
    StockRR,
    MyRR,
}

impl PlaylistType {
    pub fn get_id(&self) -> PlaylistId<'static> {
        dotenv::from_path(ProjectFiles::DotEnv.path()).ok();
        let env_id = match self {
            PlaylistType::StockRR => "RELEASE_RADAR_ID",
            PlaylistType::MyRR => "MY_RELEASE_RADAR_ID",
        };
        let rr_id = env::var(env_id)
            .expect("Error: The MY_RELEASE_RADAR_ID environmental variable was not found");
        let pl_id = PlaylistId::from_id(Cow::from(rr_id))
            .expect("Error: The PlaylistId could not be created from the playlist ID");
        pl_id.into_static()
    }
}
