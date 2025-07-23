use rspotify::model::{FullTrack, SavedTrack, SimplifiedTrack};

/// Trait that abstracts all fields of `SimplifiedTrack`
pub trait TrackInfo {
    // fn album(&self) -> Option<&SimplifiedAlbum>;
    // fn artists(&self) -> &[SimplifiedArtist];
    // fn available_markets(&self) -> Option<&[String]>;
    // fn disc_number(&self) -> u32;
    // fn duration(&self) -> Duration;
    // fn explicit(&self) -> bool;
    // fn external_urls(&self) -> &HashMap<String, String>;
    // fn href(&self) -> Option<&str>;
    // fn id(&self) -> Option<&TrackId>;
    // fn is_local(&self) -> bool;
    // fn is_playable(&self) -> Option<bool>;
    // fn linked_from(&self) -> Option<&TrackLink>;
    // fn restrictions(&self) -> Option<&Restriction>;
    // fn name(&self) -> &str;
    // fn preview_url(&self) -> Option<&str>;
    // fn track_number(&self) -> u32;
}
// pub trait FullTrackInfo: TrackInfo {
//     fn full_track(&self) -> &FullTrack;
//     fn popularity(&self) -> u32;
//     fn external_ids(&self) -> Option<&HashMap<String, String>>;
// }
impl TrackInfo for SimplifiedTrack {
    // fn album(&self) -> Option<&SimplifiedAlbum> {
    //     Option::from(&self.album)
    // }
    // fn artists(&self) -> &[SimplifiedArtist] {
    //     &self.artists
    // }
    //
    // fn available_markets(&self) -> Option<&[String]> {
    //     self.available_markets.as_deref()
    // }
    //
    // fn disc_number(&self) -> u32 {
    //     self.disc_number as u32
    // }
    //
    // fn duration(&self) -> Duration {
    //     Duration::from_millis(self.duration.num_milliseconds() as u64)
    // }
    //
    // fn explicit(&self) -> bool {
    //     self.explicit
    // }
    //
    // fn external_urls(&self) -> &HashMap<String, String> {
    //     &self.external_urls
    // }
    //
    // fn href(&self) -> Option<&str> {
    //     self.href.as_deref()
    // }
    //
    // fn id(&self) -> Option<&TrackId> {
    //     self.id.as_ref()
    // }
    //
    // fn is_local(&self) -> bool {
    //     self.is_local
    // }
    //
    // fn is_playable(&self) -> Option<bool> {
    //     self.is_playable
    // }
    //
    // fn linked_from(&self) ->Option<&TrackLink> {
    //     self.linked_from.as_ref()
    // }
    //
    // fn restrictions(&self) -> Option<&Restriction> {
    //     self.restrictions.as_ref()
    // }
    //
    // fn name(&self) -> &str {
    //     &self.name
    // }
    //
    // fn preview_url(&self) -> Option<&str> {
    //     self.preview_url.as_deref()
    // }
    //
    // fn track_number(&self) -> u32 {
    //     self.track_number
    // }
}
impl TrackInfo for FullTrack {
    // fn album(&self) -> Option<&SimplifiedAlbum> {
    //     Some(&self.album)
    // }
    //
    // fn artists(&self) -> &[SimplifiedArtist] {
    //     &self.artists
    // }
    //
    // fn available_markets(&self) -> Option<&[String]> {
    //     Some(&self.available_markets)
    // }
    //
    // fn disc_number(&self) -> u32 {
    //     self.disc_number as u32
    // }
    //
    // fn duration(&self) -> Duration {
    //     Duration::from_millis(self.duration.num_milliseconds() as u64)
    // }
    //
    // fn explicit(&self) -> bool {
    //     self.explicit
    // }
    //
    // fn external_urls(&self) -> &HashMap<String, String> {
    //     &self.external_urls
    // }
    //
    // fn href(&self) -> Option<&str> {
    //     self.href.as_deref()
    // }
    // fn id(&self) -> Option<&TrackId> {
    //     self.id.as_ref()
    // }
    //
    // fn is_local(&self) -> bool {
    //     self.is_local
    // }
    //
    // fn is_playable(&self) -> Option<bool> {
    //     self.is_playable
    // }
    //
    // fn linked_from(&self) ->Option<&TrackLink> {
    //     self.linked_from.as_ref()
    // }
    //
    // fn restrictions(&self) -> Option<&Restriction> {
    //     self.restrictions.as_ref()
    // }
    //
    // fn name(&self) -> &str {
    //     &self.name
    // }
    //
    // fn preview_url(&self) -> Option<&str> {
    //     self.preview_url.as_deref()
    // }
    //
    // fn track_number(&self) -> u32 {
    //     self.track_number
    // }
}
// impl FullTrackInfo for FullTrack {
//     fn full_track(&self) -> &FullTrack {
//         self
//     }
//     fn popularity(&self) -> u32 {
//         self.popularity
//     }
//
//     fn external_ids(&self) -> Option<&HashMap<String, String>> {
//         Some(&self.external_ids)
//     }
// }

impl TrackInfo for SavedTrack {
    // fn album(&self) -> Option<&SimplifiedAlbum> {
    //     Some(&self.track.album)
    // }
    //
    // fn artists(&self) -> &[SimplifiedArtist] {
    //     &self.track.artists
    // }
    //
    // fn available_markets(&self) -> Option<&[String]> {
    //     Some(&self.track.available_markets)
    // }
    //
    // fn disc_number(&self) -> u32 {
    //     self.track.disc_number as u32
    // }
    //
    // fn duration(&self) -> Duration {
    //     Duration::from_millis(self.track.duration.num_milliseconds() as u64)
    // }
    //
    // fn explicit(&self) -> bool {
    //     self.track.explicit
    // }
    //
    // fn external_urls(&self) -> &HashMap<String, String> {
    //     &self.track.external_urls
    // }
    //
    // fn href(&self) -> Option<&str> {
    //     self.track.href.as_deref()
    // }
    // fn id(&self) -> Option<&TrackId> {
    //     self.track.id.as_ref()
    // }
    //
    // fn is_local(&self) -> bool {
    //     self.track.is_local
    // }
    //
    // fn is_playable(&self) -> Option<bool> {
    //     self.track.is_playable
    // }
    //
    // fn linked_from(&self) ->Option<&TrackLink> {
    //     self.track.linked_from.as_ref()
    // }
    //
    // fn restrictions(&self) -> Option<&Restriction> {
    //     self.track.restrictions.as_ref()
    // }
    //
    // fn name(&self) -> &str {
    //     &self.track.name
    // }
    //
    // fn preview_url(&self) -> Option<&str> {
    //     self.track.preview_url.as_deref()
    // }
    //
    // fn track_number(&self) -> u32 {
    //     self.track.track_number
    // }
}

// impl FullTrackInfo for SavedTrack {
//     fn full_track(&self) -> &FullTrack {
//         &self.track
//     }
//     fn popularity(&self) -> u32 {
//         self.track.popularity
//     }
//
//     fn external_ids(&self) -> Option<&HashMap<String, String>> {
//         Some(&self.track.external_ids)
//     }
// }
