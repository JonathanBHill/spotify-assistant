use regex::Regex;
use rspotify::model::{FullTrack, SimplifiedArtist};
use rspotify::prelude::Id;
use std::hash::{Hash, Hasher};
use tracing::{debug, debug_span, trace};

#[derive(Debug, Clone, Default)]
pub struct PlaylistFingerprints {
    pub full_fp: Vec<FullTrackFingerprint>,
    pub distinct_fp: Vec<FullTrackFingerprint>,
    pub duplicates_fp: Vec<FullTrackFingerprint>,
}

impl PlaylistFingerprints {
    pub fn new(tracks: &Vec<FullTrack>) -> Self {
        let _distinct_fp_span = debug_span!("distinct-fp").entered();
        let full_fp = tracks.iter().map(FullTrackFingerprint::new).collect();
        let (distinct_fp, duplicates_fp) = PlaylistFingerprints::distinct_fingerprints(&full_fp);
        // FullTrackFingerprints { fingerprints }
        PlaylistFingerprints {
            full_fp,
            distinct_fp,
            duplicates_fp,
        }
    }
    pub fn insert_track(&mut self, track: &FullTrack) {
        let fp = FullTrackFingerprint::new(track);
        self.full_fp.push(fp.clone());
        if !self.distinct_fp.contains(&fp) {
            trace!(distinct = ?fp, distinct_count = self.distinct_fp.len() + 1);
            self.distinct_fp.push(fp);
        } else {
            trace!(duplicate = ?fp, duplicate_count = self.duplicates_fp.len() + 1);
            self.duplicates_fp.push(fp);
        }
    }
    pub fn distinct_fingerprints(
        track_fp: &Vec<FullTrackFingerprint>,
    ) -> (Vec<FullTrackFingerprint>, Vec<FullTrackFingerprint>) {
        let mut seen = std::collections::HashSet::new();
        let mut distinct_fp = Vec::new();
        let mut duplicates_fp = Vec::new();
        track_fp.iter().for_each(|fp| {
            let fingerprint = fp.clone();
            if !seen.insert(fingerprint.clone()) {
                trace!(duplicate = ?fingerprint);
                duplicates_fp.push(fingerprint);
            } else {
                trace!(distinct = ?fingerprint);
                distinct_fp.push(fingerprint);
            }
        });
        (distinct_fp, duplicates_fp)
    }
    pub fn get_track_ids(&self, fp_type: &str) -> Vec<String> {
        let fp_vec = match fp_type {
            "full" => self.full_fp.clone(),
            "distinct" => self.distinct_fp.clone(),
            "duplicates" => self.duplicates_fp.clone(),
            _ => panic!("Invalid fingerprint type: {}", fp_type),
        };
        fp_vec.iter().map(|fp| fp.title.clone()).collect()
    }
    pub fn filter_duplicates(&self, other: PlaylistFingerprints) -> Vec<FullTrackFingerprint> {
        let _filter_span = debug_span!("filter-duplicates").entered();
        let mut dup_tracks = Vec::new();
        let mut count = 0;
        for fp in self.distinct_fp.iter() {
            if !other.distinct_fp.contains(fp) {
                dup_tracks.push(fp.clone());
            } else {
                count += 1;
                debug!(track_fp = ?fp, "Duplicate track skipped");
            }
        }
        debug!(duplicate_track_count = count);
        dup_tracks
    }
}
#[derive(Debug, Clone, Eq, Default)]
pub struct FullTrackFingerprint {
    isrc: Option<String>,
    title: String,
    base_artists: Vec<String>,
    duration: i32,
    id: String,
}

impl PartialEq for FullTrackFingerprint {
    fn eq(&self, other: &Self) -> bool {
        self.isrc == other.isrc
            && self.base_artists == other.base_artists
            && self.duration == other.duration
    }
}
impl Hash for FullTrackFingerprint {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.isrc.hash(state);
        self.base_artists.hash(state);
        self.duration.hash(state);
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
            title: norm_title,
            base_artists,
            duration: dur_bucket,
            id,
        }
    }
    fn normalize_title(t: &str) -> String {
        t.to_lowercase()
    }
    fn lower_names(xs: &[SimplifiedArtist]) -> Vec<String> {
        xs.iter().map(|a| a.name.to_lowercase()).collect()
    }
}
