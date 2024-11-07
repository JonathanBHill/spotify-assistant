use std::borrow::Cow;
use std::env;

use rspotify::model::PlaylistId;

use crate::enums::fs::ProjectFiles;

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

#[cfg(test)]
mod tests {
    use rspotify::model::Id;

    use super::*;

    #[test]
    fn test_get_id() {
        let stock_rr = PlaylistType::StockRR;
        let my_rr = PlaylistType::MyRR;
        let stock_id = stock_rr.get_id();
        let my_id = my_rr.get_id();
        assert_eq!(stock_id.id(), "37i9dQZEVXbdINACbjb1qu");
        assert_eq!(my_id.id(), "46mIugmIiN2HYVwAwlaBAr");
    }
}
