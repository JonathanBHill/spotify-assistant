use std::collections::{HashMap, HashSet};

use rspotify::{AuthCodeSpotify, scopes};
use rspotify::clients::BaseClient;
use rspotify::model::{
    ArtistId, FullTrack, Recommendations, RecommendationsAttribute, RecommendationsSeedType,
    SimplifiedTrack, TrackId,
};
use tracing::{info, Level};

use spotify_assistant_core::enums::validation::BatchLimits;
use spotify_assistant_core::traits::apis::{Api, Querying};

use crate::mongo::models::RecommendedRecord;

pub struct RecommendStruct {
    client: AuthCodeSpotify,
    // pub tracks: Vec<SimplifiedTrack>,
}

impl Api for RecommendStruct {
    fn select_scopes() -> HashSet<String> {
        scopes!("user-library-read")
    }
}

impl Querying for RecommendStruct {
    async fn new() -> Self {
        let client = Self::set_up_client(false, Some(Self::select_scopes())).await;
        RecommendStruct { client }
    }
}

impl RecommendStruct {
    pub async fn get_recommendations(&self) -> Recommendations {
        let track_ids = vec![TrackId::from_id("0tkweWSDmerhKMIaM2midQ").unwrap()];
        let ids = vec![
            ArtistId::from_id("7jIewPOjOwffB1mcJIk4vP").unwrap(),
            ArtistId::from_id("28H813zcseKDMDftpws5ZC").unwrap(),
        ];
        let genres = vec!["dubstep", "trap"];
        // let test = client.albums()
        let test = vec![RecommendationsAttribute::TargetEnergy(0.5)];
        let recs = self
            .client
            .recommendations(
                test,
                Some(ids),
                Some(genres),
                Some(track_ids),
                Some(Self::market()),
                Some(100),
            )
            .await
            .expect("Could not retrieve recommendations");
        recs
    }
    pub async fn simple_to_full_tracks(&self, tracks: Vec<SimplifiedTrack>) -> Vec<FullTrack> {
        let span = tracing::span!(Level::INFO, "RecommendStruct.simple_to_full_tracks");
        let _enter = span.enter();

        let track_ids = tracks
            .iter()
            .map(|track| track.id.clone().unwrap())
            .collect::<Vec<TrackId>>();
        let limit = BatchLimits::Tracks.get_limit();
        let mut full_tracks = Vec::new();
        for id_chunk in track_ids.chunks(limit) {
            let full_tracks_chunk = match self
                .client
                .tracks(id_chunk.to_vec(), Some(Self::market()))
                .await
            {
                Ok(full_tracks) => {
                    info!("Retrieved {} full tracks", full_tracks.len());
                    full_tracks
                }
                Err(err) => {
                    panic!("Could not retrieve full tracks: {:?}", err);
                }
            };
            full_tracks.extend(full_tracks_chunk);
        }
        full_tracks
    }
    pub async fn mongo_fmt(&self) -> RecommendedRecord {
        let now = chrono::Local::now();
        let date_formatted = now.format("%m-%d-%Y").to_string();
        let time_formatted = now.format("%H:%M:%S").to_string();
        let mut datetime = HashMap::new();
        datetime.insert("date".to_string(), date_formatted.clone());
        datetime.insert("time".to_string(), time_formatted);
        let recommends = self.get_recommendations().await;
        let generation_seeds = recommends.seeds;
        let tracks = self.simple_to_full_tracks(recommends.tracks).await;
        let mut tracker: HashMap<&str, i8> =
            HashMap::from([("artists", 0), ("genres", 0), ("tracks", 0)]);
        generation_seeds
            .iter()
            .for_each(|seed| match seed._type.clone() {
                RecommendationsSeedType::Artist => {
                    let current = tracker.remove("artists").unwrap();
                    let new = current + 1;
                    tracker.insert("artists", new);
                }
                RecommendationsSeedType::Genre => {
                    let current = tracker.remove("genres").unwrap();
                    let new = current + 1;
                    tracker.insert("genres", new);
                }
                RecommendationsSeedType::Track => {
                    let current = tracker.remove("tracks").unwrap();
                    let new = current + 1;
                    tracker.insert("tracks", new);
                }
            });
        let name = format!(
            "{}art_{}gen_{}trk_{}",
            tracker.get("artists").unwrap(),
            tracker.get("genres").unwrap(),
            tracker.get("tracks").unwrap(),
            date_formatted.clone()
        );
        RecommendedRecord {
            name,
            generation_seeds,
            tracks,
            datetime,
        }
    }
}
