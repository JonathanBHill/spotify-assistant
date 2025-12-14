use crate::collect_track_field;
use crate::enums::extractors::album::AlbumExtractor;
use crate::utilities::general::format_duration;
use rspotify::model::{
    AlbumId, ArtistId, FullTrack, Image, PlayableId, Restriction, RestrictionReason, SavedTrack,
    SimplifiedArtist, SimplifiedTrack, TrackId, TrackLink,
};

#[derive(Debug, Clone)]
pub enum TrackExtractor {
    SavedTracks(Vec<SavedTrack>),
    FullTrack(Vec<FullTrack>),
    SimplifiedTrack(Vec<SimplifiedTrack>),
    TrackLink(Vec<TrackLink>),
}
impl TrackExtractor {
    pub fn is_empty(&self) -> bool {
        match self {
            TrackExtractor::SavedTracks(tracks) => tracks.is_empty(),
            TrackExtractor::FullTrack(tracks) => tracks.is_empty(),
            TrackExtractor::SimplifiedTrack(tracks) => tracks.is_empty(),
            TrackExtractor::TrackLink(tracks) => tracks.is_empty(),
        }
    }
    pub fn len(&self) -> usize {
        match self {
            TrackExtractor::SavedTracks(tracks) => tracks.len(),
            TrackExtractor::FullTrack(tracks) => tracks.len(),
            TrackExtractor::SimplifiedTrack(tracks) => tracks.len(),
            TrackExtractor::TrackLink(tracks) => tracks.len(),
        }
    }
    pub fn playable_ids(&self) -> Option<Vec<PlayableId<'_>>> {
        match self {
            TrackExtractor::SavedTracks(tracks) => Some(
                tracks
                    .iter()
                    .filter_map(|track| {
                        track
                            .track
                            .id
                            .clone()
                            .map(|id| PlayableId::Track(id).into_static())
                    })
                    .collect(),
            ),
            TrackExtractor::FullTrack(tracks) => Some(
                tracks
                    .iter()
                    .filter_map(|track| {
                        track
                            .id
                            .clone()
                            .map(|id| PlayableId::Track(id).into_static())
                    })
                    .collect(),
            ),
            TrackExtractor::SimplifiedTrack(tracks) => Some(
                tracks
                    .iter()
                    .filter_map(|track| {
                        track
                            .id
                            .clone()
                            .map(|id| PlayableId::Track(id).into_static())
                    })
                    .collect(),
            ),
            TrackExtractor::TrackLink(_) => None,
        }
    }
    pub fn names(&self) -> Option<Vec<String>> {
        match self {
            TrackExtractor::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| track.track.name.clone())
            }
            TrackExtractor::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| track.name.clone())
            }
            TrackExtractor::SimplifiedTrack(tracks) => {
                collect_track_field!(tracks, |track: &SimplifiedTrack| track.name.clone())
            }
            TrackExtractor::TrackLink(_) => None,
        }
    }
    pub fn ids(&self) -> Option<Vec<TrackId<'_>>> {
        let default_id =
            TrackId::from_id("unknown").expect("Failed to create TrackId from unknown ID");
        match self {
            TrackExtractor::SavedTracks(tracks) => {
                collect_track_field!(
                    tracks,
                    |track: &SavedTrack| track.track.id.clone(),
                    default_id.clone()
                )
            }
            TrackExtractor::FullTrack(tracks) => {
                collect_track_field!(
                    tracks,
                    |track: &FullTrack| track.id.clone(),
                    default_id.clone()
                )
            }
            TrackExtractor::SimplifiedTrack(tracks) => {
                collect_track_field!(
                    tracks,
                    |track: &SimplifiedTrack| track.id.clone(),
                    default_id.clone()
                )
            }
            TrackExtractor::TrackLink(tracks) => {
                collect_track_field!(
                    tracks,
                    |track: &TrackLink| track.id.clone(),
                    default_id.clone()
                )
            }
        }
    }
    pub fn added_at(&self) -> Option<Vec<String>> {
        match self {
            TrackExtractor::SavedTracks(tracks) => Some(
                tracks
                    .iter()
                    .map(|track| track.added_at.to_rfc3339())
                    .collect(),
            ),
            _ => None,
        }
    }

    pub fn available_markets(&self) -> Option<Vec<Vec<String>>> {
        match self {
            TrackExtractor::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| track
                    .track
                    .available_markets
                    .clone())
            }
            TrackExtractor::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| track.available_markets.clone())
            }
            TrackExtractor::SimplifiedTrack(tracks) => {
                collect_track_field!(tracks, |track: &SimplifiedTrack| {
                    match track.available_markets.clone() {
                        Some(markets) => markets.clone(),
                        None => vec![String::new()],
                    }
                })
            }
            TrackExtractor::TrackLink(_) => None,
        }
    }
    pub fn disc_numbers(&self) -> Option<Vec<u32>> {
        match self {
            TrackExtractor::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| track
                    .track
                    .disc_number
                    .to_string()
                    .parse::<u32>()
                    .unwrap_or(0))
            }
            TrackExtractor::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| track
                    .disc_number
                    .to_string()
                    .parse::<u32>()
                    .unwrap_or(0))
            }
            TrackExtractor::SimplifiedTrack(tracks) => {
                collect_track_field!(tracks, |track: &SimplifiedTrack| track
                    .disc_number
                    .to_string()
                    .parse::<u32>()
                    .unwrap_or(0))
            }
            TrackExtractor::TrackLink(_) => None,
        }
    }
    pub fn durations(&self) -> Option<Vec<String>> {
        match self {
            TrackExtractor::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| format_duration(
                    track.track.duration
                ))
            }
            TrackExtractor::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| format_duration(track.duration))
            }
            TrackExtractor::SimplifiedTrack(tracks) => {
                collect_track_field!(tracks, |track: &SimplifiedTrack| format_duration(
                    track.duration
                ))
            }
            TrackExtractor::TrackLink(_) => None,
        }
    }
    pub fn explicit(&self) -> Option<Vec<bool>> {
        match self {
            TrackExtractor::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| track.track.explicit)
            }
            TrackExtractor::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| track.explicit)
            }
            TrackExtractor::SimplifiedTrack(tracks) => {
                collect_track_field!(tracks, |track: &SimplifiedTrack| track.explicit)
            }
            TrackExtractor::TrackLink(_) => None,
        }
    }

    //todo: Implement fields that return hashmaps (external_ids, external_urls)
    pub fn hrefs(&self) -> Option<Vec<String>> {
        match self {
            TrackExtractor::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| {
                    track
                        .track
                        .href
                        .clone()
                        .unwrap_or_else(|| String::from("unknown"))
                })
            }
            TrackExtractor::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| {
                    track
                        .href
                        .clone()
                        .unwrap_or_else(|| String::from("unknown"))
                })
            }
            TrackExtractor::SimplifiedTrack(tracks) => {
                collect_track_field!(tracks, |track: &SimplifiedTrack| {
                    track
                        .href
                        .clone()
                        .unwrap_or_else(|| String::from("unknown"))
                })
            }
            TrackExtractor::TrackLink(tracks) => {
                collect_track_field!(tracks, |track: &TrackLink| { track.href.clone() })
            }
        }
    }
    pub fn is_local(&self) -> Option<Vec<bool>> {
        match self {
            TrackExtractor::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| track.track.is_local)
            }
            TrackExtractor::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| track.is_local)
            }
            TrackExtractor::SimplifiedTrack(tracks) => {
                collect_track_field!(tracks, |track: &SimplifiedTrack| track.is_local)
            }
            TrackExtractor::TrackLink(_) => None,
        }
    }
    pub fn is_playable(&self) -> Option<Vec<bool>> {
        match self {
            TrackExtractor::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| {
                    track.track.is_playable.unwrap_or(false)
                })
            }
            TrackExtractor::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| {
                    track.is_playable.unwrap_or(false)
                })
            }
            TrackExtractor::SimplifiedTrack(tracks) => {
                collect_track_field!(tracks, |track: &SimplifiedTrack| {
                    track.is_playable.unwrap_or(false)
                })
            }
            TrackExtractor::TrackLink(_) => None,
        }
    }
    pub fn uris(&self) -> Option<Vec<String>> {
        match self {
            TrackExtractor::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| {
                    match track.clone().track.linked_from {
                        Some(linked_from) => linked_from.uri,
                        None => "spotify:track:unknown".to_string(),
                    }
                })
            }
            TrackExtractor::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| {
                    match track.clone().linked_from {
                        Some(linked_from) => linked_from.uri,
                        None => "spotify:track:unknown".into(),
                    }
                })
            }
            TrackExtractor::SimplifiedTrack(tracks) => {
                collect_track_field!(tracks, |track: &SimplifiedTrack| {
                    match track.clone().linked_from {
                        Some(linked_from) => linked_from.uri,
                        None => "spotify:track:unknown".into(),
                    }
                })
            }
            TrackExtractor::TrackLink(tracks) => {
                collect_track_field!(tracks, |track: &TrackLink| track.uri.clone())
            }
        }
    }

    fn restrictions_to_string_helper(restrictions: &Option<Restriction>) -> String {
        let restriction = match restrictions {
            Some(restriction) => restriction,
            None => return String::from("none"),
        };
        match restriction.reason {
            RestrictionReason::Market => String::from("market"),
            RestrictionReason::Product => String::from("product"),
            RestrictionReason::Explicit => String::from("explicit"),
        }
    }

    pub fn restrictions(&self) -> Option<Vec<String>> {
        match self {
            TrackExtractor::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| {
                    Self::restrictions_to_string_helper(&track.track.restrictions)
                })
            }
            TrackExtractor::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| {
                    Self::restrictions_to_string_helper(&track.restrictions)
                })
            }
            TrackExtractor::SimplifiedTrack(tracks) => {
                collect_track_field!(tracks, |track: &SimplifiedTrack| {
                    Self::restrictions_to_string_helper(&track.restrictions)
                })
            }
            TrackExtractor::TrackLink(_) => None,
        }
    }

    pub fn popularity(&self) -> Option<Vec<u32>> {
        match self {
            TrackExtractor::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| track.track.popularity)
            }
            TrackExtractor::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| track.popularity)
            }
            TrackExtractor::SimplifiedTrack(_) => None,
            TrackExtractor::TrackLink(_) => None,
        }
    }

    pub fn preview_urls(&self) -> Option<Vec<String>> {
        match self {
            TrackExtractor::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| {
                    track
                        .track
                        .preview_url
                        .clone()
                        .unwrap_or_else(|| String::from("unknown"))
                })
            }
            TrackExtractor::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| {
                    track
                        .preview_url
                        .clone()
                        .unwrap_or_else(|| String::from("unknown"))
                })
            }
            TrackExtractor::SimplifiedTrack(tracks) => {
                collect_track_field!(tracks, |track: &SimplifiedTrack| {
                    track
                        .preview_url
                        .clone()
                        .unwrap_or_else(|| String::from("unknown"))
                })
            }
            TrackExtractor::TrackLink(_) => None,
        }
    }

    pub fn track_numbers(&self) -> Option<Vec<u32>> {
        match self {
            TrackExtractor::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| track.track.track_number)
            }
            TrackExtractor::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| track.track_number)
            }
            TrackExtractor::SimplifiedTrack(tracks) => {
                collect_track_field!(tracks, |track: &SimplifiedTrack| track.track_number)
            }
            TrackExtractor::TrackLink(_) => None,
        }
    }
    // Region albums
    fn albums_extractor(&self) -> Option<AlbumExtractor> {
        match self {
            TrackExtractor::SavedTracks(tracks) => {
                let albums = collect_track_field!(tracks, |track: &SavedTrack| {
                    track.track.album.clone()
                })
                .unwrap();
                Some(AlbumExtractor::SimplifiedAlbums(albums))
            }
            TrackExtractor::FullTrack(tracks) => {
                let albums =
                    collect_track_field!(tracks, |track: &FullTrack| { track.album.clone() })
                        .unwrap();
                Some(AlbumExtractor::SimplifiedAlbums(albums))
            }
            TrackExtractor::SimplifiedTrack(tracks) => {
                let albums = collect_track_field!(tracks, |track: &SimplifiedTrack| {
                    track.album.clone().unwrap_or_default()
                })
                .unwrap();
                Some(AlbumExtractor::SimplifiedAlbums(albums))
            }
            TrackExtractor::TrackLink(_) => None,
        }
    }
    pub fn album_groups(&self) -> Option<Vec<String>> {
        if let Some(album_extractor) = self.albums_extractor() {
            album_extractor.groups()
        } else {
            None
        }
    }
    pub fn album_types(&self) -> Option<Vec<String>> {
        if let Some(album_extractor) = self.albums_extractor() {
            album_extractor.types()
        } else {
            None
        }
    }
    pub fn album_artists(&self) -> Option<Vec<Vec<SimplifiedArtist>>> {
        if let Some(album_extractor) = self.albums_extractor() {
            album_extractor.artists()
        } else {
            None
        }
    }
    pub fn album_artist_hrefs(&self) -> Option<Vec<Vec<String>>> {
        match self {
            TrackExtractor::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| {
                    track
                        .track
                        .album
                        .artists
                        .iter()
                        .map(|artist| {
                            artist
                                .href
                                .clone()
                                .unwrap_or_else(|| String::from("unknown"))
                        })
                        .collect()
                })
            }
            TrackExtractor::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| {
                    track
                        .album
                        .artists
                        .iter()
                        .map(|artist| {
                            artist
                                .href
                                .clone()
                                .unwrap_or_else(|| String::from("unknown"))
                        })
                        .collect()
                })
            }
            TrackExtractor::SimplifiedTrack(tracks) => {
                collect_track_field!(tracks, |track: &SimplifiedTrack| {
                    match track.clone().album {
                        Some(album) => album
                            .artists
                            .iter()
                            .map(|artist| {
                                artist
                                    .href
                                    .clone()
                                    .unwrap_or_else(|| String::from("unknown"))
                            })
                            .collect(),
                        None => vec![String::from("unknown")],
                    }
                })
            }
            TrackExtractor::TrackLink(_) => None,
        }
    }
    pub fn album_artist_names(&self) -> Option<Vec<Vec<String>>> {
        match self {
            TrackExtractor::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| {
                    track
                        .track
                        .album
                        .artists
                        .iter()
                        .map(|artist| artist.name.clone())
                        .collect()
                })
            }
            TrackExtractor::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| {
                    track
                        .album
                        .artists
                        .iter()
                        .map(|artist| artist.name.clone())
                        .collect()
                })
            }
            TrackExtractor::SimplifiedTrack(tracks) => {
                collect_track_field!(tracks, |track: &SimplifiedTrack| {
                    match track.album.clone() {
                        Some(album) => album
                            .artists
                            .iter()
                            .map(|artist| artist.name.clone())
                            .collect(),
                        None => vec![String::from("unknown")],
                    }
                })
            }
            TrackExtractor::TrackLink(_) => None,
        }
    }
    pub fn album_artist_ids(&self) -> Option<Vec<Vec<ArtistId<'_>>>> {
        let default_artist_id =
            ArtistId::from_id("unknown").expect("Failed to create ArtistId from unknown ID");
        match self {
            TrackExtractor::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| {
                    track
                        .track
                        .album
                        .artists
                        .iter()
                        .map(|artist| match artist.id.clone() {
                            Some(id) => id,
                            None => default_artist_id.clone(),
                        })
                        .collect()
                })
            }
            TrackExtractor::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| {
                    track
                        .album
                        .artists
                        .iter()
                        .map(|artist| match artist.id.clone() {
                            Some(id) => id,
                            None => default_artist_id.clone(),
                        })
                        .collect()
                })
            }
            TrackExtractor::SimplifiedTrack(tracks) => {
                collect_track_field!(tracks, |track: &SimplifiedTrack| {
                    match track.clone().album {
                        Some(album) => album
                            .artists
                            .iter()
                            .map(|artist| match artist.id.clone() {
                                Some(id) => id,
                                None => default_artist_id.clone(),
                            })
                            .collect(),
                        None => vec![default_artist_id.clone()],
                    }
                })
            }
            TrackExtractor::TrackLink(_) => None,
        }
    }
    pub fn album_available_markets(&self) -> Option<Vec<Vec<String>>> {
        if let Some(album_extractor) = self.albums_extractor() {
            album_extractor.available_markets()
        } else {
            None
        }
    }
    pub fn album_hrefs(&self) -> Option<Vec<String>> {
        if let Some(album_extractor) = self.albums_extractor() {
            album_extractor.hrefs()
        } else {
            None
        }
    }
    pub fn album_images(&self) -> Option<Vec<Image>> {
        if let Some(album_extractor) = self.albums_extractor() {
            album_extractor.images()
        } else {
            None
        }
    }
    pub fn album_image_urls(&self) -> Option<Vec<String>> {
        if let Some(album_extractor) = self.albums_extractor() {
            album_extractor.image_urls()
        } else {
            None
        }
    }
    pub fn album_image_dimensions(&self) -> Option<Vec<(u32, u32)>> {
        if let Some(album_extractor) = self.albums_extractor() {
            album_extractor.image_dimensions()
        } else {
            None
        }
    }
    pub fn album_ids(&self) -> Option<Vec<AlbumId<'_>>> {
        if let Some(album_extractor) = self.albums_extractor() {
            album_extractor.ids()
        } else {
            None
        }
    }
    pub fn album_names(&self) -> Option<Vec<String>> {
        if let Some(album_extractor) = self.albums_extractor() {
            album_extractor.names()
        } else {
            None
        }
    }
    pub fn album_release_dates(&self) -> Option<Vec<String>> {
        if let Some(album_extractor) = self.albums_extractor() {
            album_extractor.release_dates()
        } else {
            None
        }
    }
    pub fn album_release_date_precision(&self) -> Option<Vec<String>> {
        if let Some(album_extractor) = self.albums_extractor() {
            album_extractor.reelease_date_precision()
        } else {
            None
        }
    }
    pub fn album_restrictions(&self) -> Option<Vec<String>> {
        if let Some(album_extractor) = self.albums_extractor() {
            album_extractor.restrictions()
        } else {
            None
        }
    }
    //End albums

    //Region artists
    pub fn artists(&self) -> Option<Vec<Vec<SimplifiedArtist>>> {
        match self {
            TrackExtractor::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| track.track.artists.to_vec())
            }
            TrackExtractor::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| track.artists.to_vec())
            }
            TrackExtractor::SimplifiedTrack(tracks) => {
                collect_track_field!(tracks, |track: &SimplifiedTrack| track.artists.to_vec())
            }
            TrackExtractor::TrackLink(_) => None,
        }
    }
    pub fn artist_ids(&self) -> Option<Vec<Vec<ArtistId<'_>>>> {
        match self {
            TrackExtractor::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| {
                    track
                        .track
                        .artists
                        .iter()
                        .map(|artist| match artist.id.clone() {
                            Some(id) => id,
                            None => ArtistId::from_id("spotify:artist:unknown")
                                .expect("Failed to create ArtistId from unknown ID"),
                        })
                        .collect::<Vec<ArtistId>>()
                })
            }
            TrackExtractor::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| {
                    track
                        .artists
                        .iter()
                        .map(|artist| match artist.id.clone() {
                            Some(id) => id,
                            None => ArtistId::from_id("spotify:artist:unknown")
                                .expect("Failed to create ArtistId from unknown ID"),
                        })
                        .collect::<Vec<ArtistId>>()
                })
            }
            TrackExtractor::SimplifiedTrack(tracks) => {
                collect_track_field!(tracks, |track: &SimplifiedTrack| {
                    track
                        .artists
                        .iter()
                        .map(|artist| match artist.id.clone() {
                            Some(id) => id,
                            None => ArtistId::from_id("spotify:artist:unknown")
                                .expect("Failed to create ArtistId from unknown ID"),
                        })
                        .collect::<Vec<ArtistId>>()
                })
            }
            TrackExtractor::TrackLink(_) => None,
        }
    }
    pub fn artist_names(&self) -> Option<Vec<Vec<String>>> {
        match self {
            TrackExtractor::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| {
                    track
                        .track
                        .artists
                        .iter()
                        .map(|artist| artist.name.clone())
                        .collect()
                })
            }
            TrackExtractor::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| {
                    track
                        .artists
                        .iter()
                        .map(|artist| artist.name.clone())
                        .collect()
                })
            }
            TrackExtractor::SimplifiedTrack(tracks) => {
                collect_track_field!(tracks, |track: &SimplifiedTrack| {
                    track
                        .artists
                        .iter()
                        .map(|artist| artist.name.clone())
                        .collect()
                })
            }
            TrackExtractor::TrackLink(_) => None,
        }
    }
    pub fn artist_hrefs(&self) -> Option<Vec<Vec<String>>> {
        match self {
            TrackExtractor::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| {
                    track
                        .track
                        .artists
                        .iter()
                        .map(|artist| {
                            artist
                                .href
                                .clone()
                                .unwrap_or_else(|| String::from("unknown"))
                        })
                        .collect()
                })
            }
            TrackExtractor::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| {
                    track
                        .artists
                        .iter()
                        .map(|artist| {
                            artist
                                .href
                                .clone()
                                .unwrap_or_else(|| String::from("unknown"))
                        })
                        .collect()
                })
            }
            TrackExtractor::SimplifiedTrack(tracks) => {
                collect_track_field!(tracks, |track: &SimplifiedTrack| {
                    track
                        .artists
                        .iter()
                        .map(|artist| {
                            artist
                                .href
                                .clone()
                                .unwrap_or_else(|| String::from("unknown"))
                        })
                        .collect()
                })
            }
            TrackExtractor::TrackLink(_) => None,
        }
    }

    //End artists
}
