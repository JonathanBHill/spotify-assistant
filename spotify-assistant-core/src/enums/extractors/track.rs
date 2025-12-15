use crate::collect_model_field;
use crate::enums::extractors::album::AlbumExtractor;
use crate::enums::extractors::artist::ArtistExtractor;
use crate::enums::extractors::helper::ExtractorHelper;
use crate::utilities::general::format_duration;
use rspotify::model::{
    FullTrack, PlayableId, Restriction, RestrictionReason, SavedTrack, SimplifiedArtist,
    SimplifiedTrack, TrackId, TrackLink,
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum TrackExtractor {
    SavedTracks(Vec<SavedTrack>),
    FullTracks(Vec<FullTrack>),
    SimplifiedTracks(Vec<SimplifiedTrack>),
    TrackLinks(Vec<TrackLink>),
}

impl ExtractorHelper for TrackExtractor {
    fn is_empty(&self) -> bool {
        match self {
            TrackExtractor::SavedTracks(tracks) => tracks.is_empty(),
            TrackExtractor::FullTracks(tracks) => tracks.is_empty(),
            TrackExtractor::SimplifiedTracks(tracks) => tracks.is_empty(),
            TrackExtractor::TrackLinks(tracks) => tracks.is_empty(),
        }
    }
    fn len(&self) -> usize {
        match self {
            TrackExtractor::SavedTracks(tracks) => tracks.len(),
            TrackExtractor::FullTracks(tracks) => tracks.len(),
            TrackExtractor::SimplifiedTracks(tracks) => tracks.len(),
            TrackExtractor::TrackLinks(tracks) => tracks.len(),
        }
    }
}
impl TrackExtractor {
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
                collect_model_field!(map, tracks, |track: &SavedTrack| track
                    .track
                    .available_markets
                    .clone())
            }
            TrackExtractor::FullTracks(tracks) => {
                collect_model_field!(map, tracks, |track: &FullTrack| track
                    .available_markets
                    .clone())
            }
            TrackExtractor::SimplifiedTracks(tracks) => {
                collect_model_field!(
                    map,
                    tracks,
                    |track: &SimplifiedTrack| { track.available_markets.clone() },
                    vec![String::new()]
                )
            }
            TrackExtractor::TrackLinks(_) => None,
        }
    }
    pub fn disc_numbers(&self) -> Option<Vec<u32>> {
        match self {
            TrackExtractor::SavedTracks(tracks) => {
                collect_model_field!(map, tracks, |track: &SavedTrack| track
                    .track
                    .disc_number
                    .to_string()
                    .parse::<u32>()
                    .unwrap_or(0))
            }
            TrackExtractor::FullTracks(tracks) => {
                collect_model_field!(map, tracks, |track: &FullTrack| track
                    .disc_number
                    .to_string()
                    .parse::<u32>()
                    .unwrap_or(0))
            }
            TrackExtractor::SimplifiedTracks(tracks) => {
                collect_model_field!(map, tracks, |track: &SimplifiedTrack| track
                    .disc_number
                    .to_string()
                    .parse::<u32>()
                    .unwrap_or(0))
            }
            TrackExtractor::TrackLinks(_) => None,
        }
    }
    pub fn durations(&self) -> Option<Vec<String>> {
        match self {
            TrackExtractor::SavedTracks(tracks) => {
                collect_model_field!(map, tracks, |track: &SavedTrack| format_duration(
                    track.track.duration
                ))
            }
            TrackExtractor::FullTracks(tracks) => {
                collect_model_field!(map, tracks, |track: &FullTrack| format_duration(
                    track.duration
                ))
            }
            TrackExtractor::SimplifiedTracks(tracks) => {
                collect_model_field!(map, tracks, |track: &SimplifiedTrack| format_duration(
                    track.duration
                ))
            }
            TrackExtractor::TrackLinks(_) => None,
        }
    }
    pub fn explicit(&self) -> Option<Vec<bool>> {
        match self {
            TrackExtractor::SavedTracks(tracks) => {
                collect_model_field!(map, tracks, |track: &SavedTrack| track.track.explicit)
            }
            TrackExtractor::FullTracks(tracks) => {
                collect_model_field!(map, tracks, |track: &FullTrack| track.explicit)
            }
            TrackExtractor::SimplifiedTracks(tracks) => {
                collect_model_field!(map, tracks, |track: &SimplifiedTrack| track.explicit)
            }
            TrackExtractor::TrackLinks(_) => None,
        }
    }

    pub fn external_ids(&self) -> Option<Vec<HashMap<String, String>>> {
        match self {
            TrackExtractor::SavedTracks(tracks) => {
                collect_model_field!(map, tracks, |track: &SavedTrack| track
                    .track
                    .external_ids
                    .clone())
            }
            TrackExtractor::FullTracks(tracks) => {
                collect_model_field!(map, tracks, |track: &FullTrack| track.external_ids.clone())
            }
            TrackExtractor::SimplifiedTracks(_) | TrackExtractor::TrackLinks(_) => None,
        }
    }
    pub fn external_urls(&self) -> Option<Vec<HashMap<String, String>>> {
        match self {
            TrackExtractor::SavedTracks(tracks) => {
                collect_model_field!(map, tracks, |track: &SavedTrack| track
                    .track
                    .external_urls
                    .clone())
            }
            TrackExtractor::FullTracks(tracks) => {
                collect_model_field!(map, tracks, |track: &FullTrack| track.external_urls.clone())
            }
            TrackExtractor::SimplifiedTracks(tracks) => {
                collect_model_field!(map, tracks, |track: &SimplifiedTrack| track
                    .external_urls
                    .clone())
            }
            TrackExtractor::TrackLinks(tracks) => {
                collect_model_field!(map, tracks, |track: &TrackLink| track.external_urls.clone())
            }
        }
    }
    pub fn hrefs(&self) -> Option<Vec<String>> {
        match self {
            TrackExtractor::SavedTracks(tracks) => {
                collect_model_field!(map, tracks, |track: &SavedTrack| {
                    track
                        .track
                        .href
                        .clone()
                        .unwrap_or_else(|| String::from("unknown"))
                })
            }
            TrackExtractor::FullTracks(tracks) => {
                collect_model_field!(map, tracks, |track: &FullTrack| {
                    track
                        .href
                        .clone()
                        .unwrap_or_else(|| String::from("unknown"))
                })
            }
            TrackExtractor::SimplifiedTracks(tracks) => {
                collect_model_field!(map, tracks, |track: &SimplifiedTrack| {
                    track
                        .href
                        .clone()
                        .unwrap_or_else(|| String::from("unknown"))
                })
            }
            TrackExtractor::TrackLinks(tracks) => {
                collect_model_field!(map, tracks, |track: &TrackLink| { track.href.clone() })
            }
        }
    }

    pub fn ids(&self) -> Option<Vec<TrackId<'_>>> {
        let default_id =
            TrackId::from_id("unknown").expect("Failed to create TrackId from unknown ID");
        match self {
            TrackExtractor::SavedTracks(tracks) => {
                collect_model_field!(
                    map,
                    tracks,
                    |track: &SavedTrack| track.track.id.clone(),
                    default_id.clone()
                )
            }
            TrackExtractor::FullTracks(tracks) => {
                collect_model_field!(
                    map,
                    tracks,
                    |track: &FullTrack| track.id.clone(),
                    default_id.clone()
                )
            }
            TrackExtractor::SimplifiedTracks(tracks) => {
                collect_model_field!(
                    map,
                    tracks,
                    |track: &SimplifiedTrack| track.id.clone(),
                    default_id.clone()
                )
            }
            TrackExtractor::TrackLinks(tracks) => {
                collect_model_field!(
                    map,
                    tracks,
                    |track: &TrackLink| track.id.clone(),
                    default_id.clone()
                )
            }
        }
    }

    pub fn is_local(&self) -> Option<Vec<bool>> {
        match self {
            TrackExtractor::SavedTracks(tracks) => {
                collect_model_field!(map, tracks, |track: &SavedTrack| track.track.is_local)
            }
            TrackExtractor::FullTracks(tracks) => {
                collect_model_field!(map, tracks, |track: &FullTrack| track.is_local)
            }
            TrackExtractor::SimplifiedTracks(tracks) => {
                collect_model_field!(map, tracks, |track: &SimplifiedTrack| track.is_local)
            }
            TrackExtractor::TrackLinks(_) => None,
        }
    }

    pub fn is_playable(&self) -> Option<Vec<bool>> {
        match self {
            TrackExtractor::SavedTracks(tracks) => {
                collect_model_field!(map, tracks, |track: &SavedTrack| {
                    track.track.is_playable.unwrap_or(false)
                })
            }
            TrackExtractor::FullTracks(tracks) => {
                collect_model_field!(map, tracks, |track: &FullTrack| {
                    track.is_playable.unwrap_or(false)
                })
            }
            TrackExtractor::SimplifiedTracks(tracks) => {
                collect_model_field!(map, tracks, |track: &SimplifiedTrack| {
                    track.is_playable.unwrap_or(false)
                })
            }
            TrackExtractor::TrackLinks(_) => None,
        }
    }

    pub fn linked_from(&self) -> Option<Vec<TrackLink>> {
        match self {
            TrackExtractor::SavedTracks(tracks) => {
                collect_model_field!(filter_map, tracks, |track: &SavedTrack| {
                    track.track.linked_from.clone()
                })
            }
            TrackExtractor::FullTracks(tracks) => {
                collect_model_field!(filter_map, tracks, |track: &FullTrack| {
                    track.linked_from.clone()
                })
            }
            TrackExtractor::SimplifiedTracks(tracks) => {
                collect_model_field!(filter_map, tracks, |track: &SimplifiedTrack| {
                    track.linked_from.clone()
                })
            }
            TrackExtractor::TrackLinks(_) => None,
        }
    }

    pub fn names(&self) -> Option<Vec<String>> {
        match self {
            TrackExtractor::SavedTracks(tracks) => {
                collect_model_field!(map, tracks, |track: &SavedTrack| track.track.name.clone())
            }
            TrackExtractor::FullTracks(tracks) => {
                collect_model_field!(map, tracks, |track: &FullTrack| track.name.clone())
            }
            TrackExtractor::SimplifiedTracks(tracks) => {
                collect_model_field!(map, tracks, |track: &SimplifiedTrack| track.name.clone())
            }
            TrackExtractor::TrackLinks(_) => None,
        }
    }

    pub fn numbers(&self) -> Option<Vec<u32>> {
        match self {
            TrackExtractor::SavedTracks(tracks) => {
                collect_model_field!(map, tracks, |track: &SavedTrack| track.track.track_number)
            }
            TrackExtractor::FullTracks(tracks) => {
                collect_model_field!(map, tracks, |track: &FullTrack| track.track_number)
            }
            TrackExtractor::SimplifiedTracks(tracks) => {
                collect_model_field!(map, tracks, |track: &SimplifiedTrack| track.track_number)
            }
            TrackExtractor::TrackLinks(_) => None,
        }
    }

    pub fn playable_ids(&self) -> Option<Vec<PlayableId<'_>>> {
        self.ids()
            .unwrap()
            .into_iter()
            .map(|id| Some(PlayableId::Track(id).into_static()))
            .collect()
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
                collect_model_field!(map, tracks, |track: &SavedTrack| {
                    Self::restrictions_to_string_helper(&track.track.restrictions)
                })
            }
            TrackExtractor::FullTracks(tracks) => {
                collect_model_field!(map, tracks, |track: &FullTrack| {
                    Self::restrictions_to_string_helper(&track.restrictions)
                })
            }
            TrackExtractor::SimplifiedTracks(tracks) => {
                collect_model_field!(map, tracks, |track: &SimplifiedTrack| {
                    Self::restrictions_to_string_helper(&track.restrictions)
                })
            }
            TrackExtractor::TrackLinks(_) => None,
        }
    }

    pub fn popularity(&self) -> Option<Vec<u32>> {
        match self {
            TrackExtractor::SavedTracks(tracks) => {
                collect_model_field!(map, tracks, |track: &SavedTrack| track.track.popularity)
            }
            TrackExtractor::FullTracks(tracks) => {
                collect_model_field!(map, tracks, |track: &FullTrack| track.popularity)
            }
            TrackExtractor::SimplifiedTracks(_) | TrackExtractor::TrackLinks(_) => None,
        }
    }

    pub fn preview_urls(&self) -> Option<Vec<String>> {
        const UNKNOWN: &str = "unknown";
        match self {
            TrackExtractor::SavedTracks(tracks) => {
                collect_model_field!(
                    map,
                    tracks,
                    |track: &SavedTrack| { track.track.preview_url.clone() },
                    UNKNOWN.to_string()
                )
            }
            TrackExtractor::FullTracks(tracks) => {
                collect_model_field!(
                    map,
                    tracks,
                    |track: &FullTrack| { track.preview_url.clone() },
                    UNKNOWN.to_string()
                )
            }
            TrackExtractor::SimplifiedTracks(tracks) => {
                collect_model_field!(
                    map,
                    tracks,
                    |track: &SimplifiedTrack| { track.preview_url.clone() },
                    UNKNOWN.to_string()
                )
            }
            TrackExtractor::TrackLinks(_) => None,
        }
    }

    pub fn uris(&self) -> Option<Vec<String>> {
        const UNKNOWN_URI: &str = "spotify:track:unknown";

        match self {
            TrackExtractor::SavedTracks(tracks) => {
                collect_model_field!(
                    map,
                    tracks,
                    |track: &SavedTrack| {
                        track
                            .track
                            .linked_from
                            .as_ref()
                            .map(|track_link| track_link.uri.clone())
                    },
                    UNKNOWN_URI.to_string()
                )
            }
            TrackExtractor::FullTracks(tracks) => {
                collect_model_field!(
                    map,
                    tracks,
                    |track: &FullTrack| {
                        track
                            .linked_from
                            .as_ref()
                            .map(|track_link| track_link.uri.clone())
                    },
                    UNKNOWN_URI.to_string()
                )
            }
            TrackExtractor::SimplifiedTracks(tracks) => {
                collect_model_field!(
                    map,
                    tracks,
                    |track: &SimplifiedTrack| {
                        track
                            .linked_from
                            .as_ref()
                            .map(|track_link| track_link.uri.clone())
                    },
                    UNKNOWN_URI.to_string()
                )
            }
            TrackExtractor::TrackLinks(tracks) => {
                collect_model_field!(map, tracks, |track_link: &TrackLink| track_link.uri.clone())
            }
        }
    }

    pub fn albums(&self) -> Option<AlbumExtractor> {
        match self {
            TrackExtractor::SavedTracks(tracks) => {
                let albums = collect_model_field!(map, tracks, |track: &SavedTrack| {
                    track.track.album.clone()
                })
                .unwrap();
                Some(AlbumExtractor::SimplifiedAlbums(albums))
            }
            TrackExtractor::FullTracks(tracks) => {
                let albums =
                    collect_model_field!(map, tracks, |track: &FullTrack| { track.album.clone() })
                        .unwrap();
                Some(AlbumExtractor::SimplifiedAlbums(albums))
            }
            TrackExtractor::SimplifiedTracks(tracks) => {
                let albums = collect_model_field!(map, tracks, |track: &SimplifiedTrack| {
                    track.album.clone().unwrap_or_default()
                })
                .unwrap();
                Some(AlbumExtractor::SimplifiedAlbums(albums))
            }
            TrackExtractor::TrackLinks(_) => None,
        }
    }
    pub fn album_artists(&self) -> Option<ArtistExtractor> {
        if let Some(album_extractor) = self.albums() {
            album_extractor.artists()
        } else {
            None
        }
    }

    pub fn artists(&self) -> Option<ArtistExtractor> {
        match self {
            TrackExtractor::SavedTracks(tracks) => {
                let artists: Vec<Vec<SimplifiedArtist>> =
                    collect_model_field!(map, tracks, |track: &SavedTrack| {
                        track.track.artists.clone()
                    })
                    .unwrap();
                Some(ArtistExtractor::SimplifiedArtists(artists))
            }
            TrackExtractor::FullTracks(tracks) => {
                let artists: Vec<Vec<SimplifiedArtist>> =
                    collect_model_field!(map, tracks, |track: &FullTrack| {
                        track.artists.clone()
                    })
                    .unwrap();
                Some(ArtistExtractor::SimplifiedArtists(artists))
            }
            TrackExtractor::SimplifiedTracks(tracks) => {
                let artists: Vec<Vec<SimplifiedArtist>> =
                    collect_model_field!(map, tracks, |track: &SimplifiedTrack| {
                        track.artists.clone()
                    })
                    .unwrap();
                Some(ArtistExtractor::SimplifiedArtists(artists))
            }
            TrackExtractor::TrackLinks(_) => None,
        }
    }
}
