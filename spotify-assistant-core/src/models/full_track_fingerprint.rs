use regex::Regex;
use rspotify::model::{FullTrack, SimplifiedArtist};
use rspotify::prelude::Id;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Eq)]
pub struct FullTrackFingerprint {
    isrc: Option<String>,
    norm_title: String,
    base_artists: Vec<String>,
    dur_bucket: i32,
    id: String,
}

impl PartialEq for FullTrackFingerprint {
    fn eq(&self, other: &Self) -> bool {
        self.isrc == other.isrc
            && self.norm_title == other.norm_title
            && self.base_artists == other.base_artists
            && self.dur_bucket == other.dur_bucket
    }
}
impl Hash for FullTrackFingerprint {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.isrc.hash(state);
        self.norm_title.hash(state);
        self.base_artists.hash(state);
        self.dur_bucket.hash(state);
    }
}

impl FullTrackFingerprint {
    pub fn new(track: &FullTrack) -> Self {
        let external = &track.external_ids;
        let isrc = external.get("isrc").unwrap().clone();
        let norm_title = FullTrackFingerprint::normalize_title(&track.name);
        let dur_bucket = (track.duration.num_milliseconds() as i32) / 1000; // seconds
        let base_artists = FullTrackFingerprint::lower_names(&track.artists);
        let id = track.id.clone().unwrap().id().to_string();
        FullTrackFingerprint {
            isrc: Some(isrc),
            norm_title,
            base_artists,
            dur_bucket,
            id,
        }
    }
    fn normalize_title(t: &str) -> String {
        let t = t.to_lowercase();
        let re_feat = Regex::new(r"(?i)\s*[(\[]?(feat\.|featuring).+?[\)\]]?").unwrap();
        let re_sp = Regex::new(r"\s+").unwrap();
        let t = re_feat.replace(&t, "");
        let t = t.replace(" - radio edit", "").replace(" - remastered", "");
        re_sp.replace_all(t.trim(), " ").into_owned()
    }
    fn lower_names(xs: &[SimplifiedArtist]) -> Vec<String> {
        xs.iter().map(|a| a.name.to_lowercase()).collect()
    }
}
