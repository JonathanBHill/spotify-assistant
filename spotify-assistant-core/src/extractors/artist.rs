use crate::enums::duplication::DuplicatePolicy;
use rspotify::model::{FullAlbum, Id, SimplifiedArtist};

/// Flatten album -> tracks -> artists (as provided by Spotify "album tracks" items).
pub fn artists_for_album(album: &FullAlbum) -> Vec<SimplifiedArtist> {
    album
        .tracks
        .items
        .iter()
        .flat_map(|t| t.artists.iter().cloned())
        .collect()
}
/// Dedup by `artist.id` while keeping first occurrence order.
pub fn dedup_artists_by_id(mut artists: Vec<SimplifiedArtist>) -> Vec<SimplifiedArtist> {
    use std::collections::HashSet;
    let mut seen: HashSet<String> = HashSet::new();
    artists.retain(|a| {
        if let Some(id) = a.id.as_ref().map(|id| id.id().to_string()) {
            seen.insert(id)
        } else {
            // if no id, drop it (or return true if you want to keep)
            false
        }
    });
    artists
}

/// Build a single entry: album name -> artist list (optionally deduped)
pub fn artists_entry_for_album(
    album: &FullAlbum,
    policy: DuplicatePolicy,
) -> (String, Vec<SimplifiedArtist>) {
    let mut v = artists_for_album(album);
    if let DuplicatePolicy::Remove = policy {
        v = dedup_artists_by_id(v);
    }
    (album.name.clone(), v)
}
