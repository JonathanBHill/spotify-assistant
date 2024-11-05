use std::collections::HashMap;

use rspotify::model::{FullTrack, RecommendationsSeed};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RecommendedRecord {
    #[serde(rename = "nickname")]
    pub name: String,
    #[serde(rename = "generation_parameters")]
    pub generation_seeds: Vec<RecommendationsSeed>,
    #[serde(rename = "track_recommendations")]
    pub tracks: Vec<FullTrack>,
    #[serde(rename = "generated_on")]
    pub datetime: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GenerationSeeds {
    #[serde(rename = "artist_ids", skip_serializing_if = "Vec::is_empty", default)]
    pub seed_artists: Vec<RecommendationsSeed>,
    #[serde(rename = "genres", skip_serializing_if = "Vec::is_empty", default)]
    pub seed_genres: Vec<RecommendationsSeed>,
    #[serde(rename = "track_ids", skip_serializing_if = "Vec::is_empty", default)]
    pub seed_tracks: Vec<RecommendationsSeed>,
}

#[cfg(test)]
mod tests {
    use rspotify::model::RecommendationsSeedType;

    use super::*;

    #[test]
    fn test_generation_seeds() {
        let artist_seed = RecommendationsSeed {
            after_filtering_size: 100,
            after_relinking_size: 100,
            href: Some("https://api.spotify.com/v1/artists/7jIewPOjOwffB1mcJIk4vP".to_string()),
            id: "7jIewPOjOwffB1mcJIk4vP".to_string(),
            initial_pool_size: 100,
            _type: RecommendationsSeedType::Artist,
        };
        let genre_seed = RecommendationsSeed {
            after_filtering_size: 100,
            after_relinking_size: 100,
            href: Some("https://api.spotify.com/v1/genres/dubstep".to_string()),
            id: "dubstep".to_string(),
            initial_pool_size: 100,
            _type: RecommendationsSeedType::Genre,
        };
        let track_seed = RecommendationsSeed {
            after_filtering_size: 100,
            after_relinking_size: 100,
            href: Some("https://api.spotify.com/v1/tracks/0tkweWSDmerhKMIaM2midQ".to_string()),
            id: "0tkweWSDmerhKMIaM2midQ".to_string(),
            initial_pool_size: 100,
            _type: RecommendationsSeedType::Track,
        };
        let gen_seed = GenerationSeeds {
            seed_artists: vec![artist_seed.clone(), artist_seed.clone(), artist_seed],
            seed_genres: vec![genre_seed.clone(), genre_seed.clone(), genre_seed],
            seed_tracks: vec![track_seed.clone(), track_seed.clone(), track_seed],
        };
        println!("{:?}", gen_seed);
    }
}
