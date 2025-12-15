use crate::collect_model_field;
use crate::enums::extractors::album::AlbumsExtractor;
use crate::enums::extractors::artist::ArtistsExtractor;
use crate::enums::extractors::helper::ExtractorHelper;
use crate::errors::SpotifyAssistantError;
use crate::errors::enums::EnumError::NotAvailableForVariant;
use crate::utilities::general::format_duration;
use rspotify::model::{
    FullTrack, PlayableId, SavedTrack, SimplifiedTrack, TrackId, TrackLink, Type,
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum TracksExtractor {
    SavedTracks(Vec<SavedTrack>),
    FullTracks(Vec<FullTrack>),
    SimplifiedTracks(Vec<SimplifiedTrack>),
    TrackLinks(Vec<TrackLink>),
}

impl ExtractorHelper for TracksExtractor {
    fn is_empty(&self) -> bool {
        match self {
            TracksExtractor::SavedTracks(tracks) => tracks.is_empty(),
            TracksExtractor::FullTracks(tracks) => tracks.is_empty(),
            TracksExtractor::SimplifiedTracks(tracks) => tracks.is_empty(),
            TracksExtractor::TrackLinks(tracks) => tracks.is_empty(),
        }
    }
    fn len(&self) -> usize {
        match self {
            TracksExtractor::SavedTracks(tracks) => tracks.len(),
            TracksExtractor::FullTracks(tracks) => tracks.len(),
            TracksExtractor::SimplifiedTracks(tracks) => tracks.len(),
            TracksExtractor::TrackLinks(tracks) => tracks.len(),
        }
    }
}
impl TracksExtractor {
    pub fn added_at(&self) -> Result<Vec<String>, SpotifyAssistantError> {
        match self {
            TracksExtractor::SavedTracks(tracks) => {
                Ok(tracks.iter().map(|track| track.added_at.to_rfc3339()).collect())
            },
            _ => Err(NotAvailableForVariant {
                action: "return added_at information",
                variant_label: "[TracksExtractor::FullTracks, TracksExtractor::SimplifiedTracks, TracksExtractor::TrackLinks] variants"
            }.into()),
        }
    }

    pub fn available_markets(&self) -> Result<Vec<Vec<String>>, SpotifyAssistantError> {
        let markets = match self {
            TracksExtractor::SavedTracks(tracks) => {
                collect_model_field!(umap, tracks, |track: &SavedTrack| track
                    .track
                    .available_markets
                    .clone())
            }
            TracksExtractor::FullTracks(tracks) => {
                collect_model_field!(umap, tracks, |track: &FullTrack| track
                    .available_markets
                    .clone())
            }
            TracksExtractor::SimplifiedTracks(tracks) => {
                collect_model_field!(
                    umap,
                    tracks,
                    |track: &SimplifiedTrack| { track.available_markets.clone() },
                    vec![String::new()]
                )
            }
            TracksExtractor::TrackLinks(_) => {
                return Err(NotAvailableForVariant {
                    action: "return added_at information",
                    variant_label: "TracksExtractor::TrackLinks variant",
                }
                .into());
            }
        };
        Ok(markets)
    }
    pub fn disc_numbers(&self) -> Result<Vec<u32>, SpotifyAssistantError> {
        let disk_nums = match self {
            TracksExtractor::SavedTracks(tracks) => {
                collect_model_field!(umap, tracks, |track: &SavedTrack| track
                    .track
                    .disc_number
                    .to_string()
                    .parse::<u32>()
                    .unwrap_or(0))
            }
            TracksExtractor::FullTracks(tracks) => {
                collect_model_field!(umap, tracks, |track: &FullTrack| track
                    .disc_number
                    .to_string()
                    .parse::<u32>()
                    .unwrap_or(0))
            }
            TracksExtractor::SimplifiedTracks(tracks) => {
                collect_model_field!(umap, tracks, |track: &SimplifiedTrack| track
                    .disc_number
                    .to_string()
                    .parse::<u32>()
                    .unwrap_or(0))
            }
            TracksExtractor::TrackLinks(_) => {
                return Err(NotAvailableForVariant {
                    action: "return added_at information",
                    variant_label: "TracksExtractor::TrackLinks variant",
                }
                .into());
            }
        };
        Ok(disk_nums)
    }
    pub fn durations(&self) -> Result<Vec<String>, SpotifyAssistantError> {
        let durations = match self {
            TracksExtractor::SavedTracks(tracks) => {
                collect_model_field!(umap, tracks, |track: &SavedTrack| format_duration(
                    track.track.duration
                ))
            }
            TracksExtractor::FullTracks(tracks) => {
                collect_model_field!(umap, tracks, |track: &FullTrack| format_duration(
                    track.duration
                ))
            }
            TracksExtractor::SimplifiedTracks(tracks) => {
                collect_model_field!(umap, tracks, |track: &SimplifiedTrack| format_duration(
                    track.duration
                ))
            }
            TracksExtractor::TrackLinks(_) => {
                return Err(NotAvailableForVariant {
                    action: "return added_at information",
                    variant_label: "TracksExtractor::TrackLinks variant",
                }
                .into());
            }
        };
        Ok(durations)
    }
    pub fn explicit(&self) -> Result<Vec<bool>, SpotifyAssistantError> {
        let explicits = match self {
            TracksExtractor::SavedTracks(tracks) => {
                collect_model_field!(umap, tracks, |track: &SavedTrack| track.track.explicit)
            }
            TracksExtractor::FullTracks(tracks) => {
                collect_model_field!(umap, tracks, |track: &FullTrack| track.explicit)
            }
            TracksExtractor::SimplifiedTracks(tracks) => {
                collect_model_field!(umap, tracks, |track: &SimplifiedTrack| track.explicit)
            }
            TracksExtractor::TrackLinks(_) => {
                return Err(NotAvailableForVariant {
                    action: "return added_at information",
                    variant_label: "TracksExtractor::TrackLinks variant",
                }
                .into());
            }
        };
        Ok(explicits)
    }

    pub fn external_ids(&self) -> Result<Vec<HashMap<String, String>>, SpotifyAssistantError> {
        let external_ids = match self {
            TracksExtractor::SavedTracks(tracks) => {
                collect_model_field!(umap, tracks, |track: &SavedTrack| track
                    .track
                    .external_ids
                    .clone())
            }
            TracksExtractor::FullTracks(tracks) => {
                collect_model_field!(umap, tracks, |track: &FullTrack| track.external_ids.clone())
            }
            TracksExtractor::SimplifiedTracks(_) | TracksExtractor::TrackLinks(_) => {
                return Err(NotAvailableForVariant {
                    action: "return added_at information",
                    variant_label: "[TracksExtractor::SimplifiedTracks, TracksExtractor::TrackLinks] variants"
                }.into())
            },
        };
        Ok(external_ids)
    }
    pub fn external_urls(&self) -> Vec<HashMap<String, String>> {
        match self {
            TracksExtractor::SavedTracks(tracks) => {
                collect_model_field!(umap, tracks, |track: &SavedTrack| track
                    .track
                    .external_urls
                    .clone())
            }
            TracksExtractor::FullTracks(tracks) => {
                collect_model_field!(umap, tracks, |track: &FullTrack| track
                    .external_urls
                    .clone())
            }
            TracksExtractor::SimplifiedTracks(tracks) => {
                collect_model_field!(umap, tracks, |track: &SimplifiedTrack| track
                    .external_urls
                    .clone())
            }
            TracksExtractor::TrackLinks(tracks) => {
                collect_model_field!(umap, tracks, |track: &TrackLink| track
                    .external_urls
                    .clone())
            }
        }
    }
    pub fn hrefs(&self) -> Vec<String> {
        match self {
            TracksExtractor::SavedTracks(tracks) => {
                collect_model_field!(umap, tracks, |track: &SavedTrack| {
                    track
                        .track
                        .href
                        .clone()
                        .unwrap_or_else(|| String::from("unknown"))
                })
            }
            TracksExtractor::FullTracks(tracks) => {
                collect_model_field!(umap, tracks, |track: &FullTrack| {
                    track
                        .href
                        .clone()
                        .unwrap_or_else(|| String::from("unknown"))
                })
            }
            TracksExtractor::SimplifiedTracks(tracks) => {
                collect_model_field!(umap, tracks, |track: &SimplifiedTrack| {
                    track
                        .href
                        .clone()
                        .unwrap_or_else(|| String::from("unknown"))
                })
            }
            TracksExtractor::TrackLinks(tracks) => {
                collect_model_field!(umap, tracks, |track: &TrackLink| { track.href.clone() })
            }
        }
    }

    pub fn ids(&self) -> Vec<TrackId<'_>> {
        let default_id =
            TrackId::from_id("unknown").expect("Failed to create TrackId from unknown ID");
        match self {
            TracksExtractor::SavedTracks(tracks) => {
                collect_model_field!(
                    umap,
                    tracks,
                    |track: &SavedTrack| track.track.id.clone(),
                    default_id.clone()
                )
            }
            TracksExtractor::FullTracks(tracks) => {
                collect_model_field!(
                    umap,
                    tracks,
                    |track: &FullTrack| track.id.clone(),
                    default_id.clone()
                )
            }
            TracksExtractor::SimplifiedTracks(tracks) => {
                collect_model_field!(
                    umap,
                    tracks,
                    |track: &SimplifiedTrack| track.id.clone(),
                    default_id.clone()
                )
            }
            TracksExtractor::TrackLinks(tracks) => {
                collect_model_field!(
                    umap,
                    tracks,
                    |track: &TrackLink| track.id.clone(),
                    default_id.clone()
                )
            }
        }
    }

    pub fn is_local(&self) -> Result<Vec<bool>, SpotifyAssistantError> {
        let is_local = match self {
            TracksExtractor::SavedTracks(tracks) => {
                collect_model_field!(umap, tracks, |track: &SavedTrack| track.track.is_local)
            }
            TracksExtractor::FullTracks(tracks) => {
                collect_model_field!(umap, tracks, |track: &FullTrack| track.is_local)
            }
            TracksExtractor::SimplifiedTracks(tracks) => {
                collect_model_field!(umap, tracks, |track: &SimplifiedTrack| track.is_local)
            }
            TracksExtractor::TrackLinks(_) => {
                return Err(NotAvailableForVariant {
                    action: "return added_at information",
                    variant_label: "TracksExtractor::TrackLinks variant",
                }
                .into());
            }
        };
        Ok(is_local)
    }

    pub fn is_playable(&self) -> Result<Vec<bool>, SpotifyAssistantError> {
        let is_playable = match self {
            TracksExtractor::SavedTracks(tracks) => {
                collect_model_field!(umap, tracks, |track: &SavedTrack| {
                    track.track.is_playable.unwrap_or(false)
                })
            }
            TracksExtractor::FullTracks(tracks) => {
                collect_model_field!(umap, tracks, |track: &FullTrack| {
                    track.is_playable.unwrap_or(false)
                })
            }
            TracksExtractor::SimplifiedTracks(tracks) => {
                collect_model_field!(umap, tracks, |track: &SimplifiedTrack| {
                    track.is_playable.unwrap_or(false)
                })
            }
            TracksExtractor::TrackLinks(_) => {
                return Err(NotAvailableForVariant {
                    action: "return added_at information",
                    variant_label: "TracksExtractor::TrackLinks variant",
                }
                .into());
            }
        };
        Ok(is_playable)
    }

    pub fn linked_from(&self) -> Result<Vec<TrackLink>, SpotifyAssistantError> {
        let default_link = TrackLink {
            external_urls: HashMap::default(),
            id: Option::from(
                TrackId::from_uri("unknown").expect("Failed to creaet TrackId from unknown ID"),
            ),
            uri: String::default(),
            href: String::default(),
            r#type: Type::Track,
        };
        let linked_from = match self {
            TracksExtractor::SavedTracks(tracks) => {
                collect_model_field!(umap, tracks, |track: &SavedTrack| {
                    track
                        .track
                        .linked_from
                        .clone()
                        .unwrap_or(default_link.clone())
                })
            }
            TracksExtractor::FullTracks(tracks) => {
                collect_model_field!(umap, tracks, |track: &FullTrack| {
                    track.linked_from.clone().unwrap_or(default_link.clone())
                })
            }
            TracksExtractor::SimplifiedTracks(tracks) => {
                collect_model_field!(umap, tracks, |track: &SimplifiedTrack| {
                    track.linked_from.clone().unwrap_or(default_link.clone())
                })
            }
            TracksExtractor::TrackLinks(_) => {
                return Err(NotAvailableForVariant {
                    action: "return added_at information",
                    variant_label: "TracksExtractor::TrackLinks variant",
                }
                .into());
            }
        };
        Ok(linked_from)
    }

    pub fn names(&self) -> Result<Vec<String>, SpotifyAssistantError> {
        let names = match self {
            TracksExtractor::SavedTracks(tracks) => {
                collect_model_field!(umap, tracks, |track: &SavedTrack| track.track.name.clone())
            }
            TracksExtractor::FullTracks(tracks) => {
                collect_model_field!(umap, tracks, |track: &FullTrack| track.name.clone())
            }
            TracksExtractor::SimplifiedTracks(tracks) => {
                collect_model_field!(umap, tracks, |track: &SimplifiedTrack| track.name.clone())
            }
            TracksExtractor::TrackLinks(_) => {
                return Err(NotAvailableForVariant {
                    action: "return added_at information",
                    variant_label: "TracksExtractor::TrackLinks variant",
                }
                .into());
            }
        };
        Ok(names)
    }

    pub fn track_numbers(&self) -> Result<Vec<u32>, SpotifyAssistantError> {
        let track_numbers = match self {
            TracksExtractor::SavedTracks(tracks) => {
                collect_model_field!(umap, tracks, |track: &SavedTrack| track.track.track_number)
            }
            TracksExtractor::FullTracks(tracks) => {
                collect_model_field!(umap, tracks, |track: &FullTrack| track.track_number)
            }
            TracksExtractor::SimplifiedTracks(tracks) => {
                collect_model_field!(umap, tracks, |track: &SimplifiedTrack| track.track_number)
            }
            TracksExtractor::TrackLinks(_) => {
                return Err(NotAvailableForVariant {
                    action: "return added_at information",
                    variant_label: "TracksExtractor::TrackLinks variant",
                }
                .into());
            }
        };
        Ok(track_numbers)
    }

    pub fn playable_ids(&self) -> Vec<PlayableId<'_>> {
        self.ids()
            .into_iter()
            .map(|id| PlayableId::Track(id).into_static())
            .collect()
    }

    pub fn restrictions(&self) -> Result<Vec<String>, SpotifyAssistantError> {
        let restrictions = match self {
            TracksExtractor::SavedTracks(tracks) => {
                collect_model_field!(umap, tracks, |track: &SavedTrack| {
                    Self::restrict_reason_as_string(&track.track.restrictions)
                })
            }
            TracksExtractor::FullTracks(tracks) => {
                collect_model_field!(umap, tracks, |track: &FullTrack| {
                    Self::restrict_reason_as_string(&track.restrictions)
                })
            }
            TracksExtractor::SimplifiedTracks(tracks) => {
                collect_model_field!(umap, tracks, |track: &SimplifiedTrack| {
                    Self::restrict_reason_as_string(&track.restrictions)
                })
            }
            TracksExtractor::TrackLinks(_) => {
                return Err(NotAvailableForVariant {
                    action: "return added_at information",
                    variant_label: "TracksExtractor::TrackLinks variant",
                }
                .into());
            }
        };
        Ok(restrictions)
    }

    pub fn popularity(&self) -> Result<Vec<u32>, SpotifyAssistantError> {
        let popularity = match self {
            TracksExtractor::SavedTracks(tracks) => {
                collect_model_field!(umap, tracks, |track: &SavedTrack| track.track.popularity)
            }
            TracksExtractor::FullTracks(tracks) => {
                collect_model_field!(umap, tracks, |track: &FullTrack| track.popularity)
            }
            TracksExtractor::SimplifiedTracks(_) | TracksExtractor::TrackLinks(_) => {
                return Err(NotAvailableForVariant {
                    action: "return added_at information",
                    variant_label: "[TracksExtractor::SimplifiedTracks, TracksExtractor::TrackLinks] variants"
                }.into())
            },
        };
        Ok(popularity)
    }

    pub fn preview_urls(&self) -> Result<Vec<String>, SpotifyAssistantError> {
        const UNKNOWN: &str = "unknown";
        let preview_urls = match self {
            TracksExtractor::SavedTracks(tracks) => {
                collect_model_field!(umap, tracks, |track: &SavedTrack| {
                    track
                        .track
                        .preview_url
                        .clone()
                        .unwrap_or(UNKNOWN.to_string())
                })
            }
            TracksExtractor::FullTracks(tracks) => {
                collect_model_field!(umap, tracks, |track: &FullTrack| {
                    track.preview_url.clone().unwrap_or(UNKNOWN.to_string())
                })
            }
            TracksExtractor::SimplifiedTracks(tracks) => {
                collect_model_field!(umap, tracks, |track: &SimplifiedTrack| {
                    track.preview_url.clone().unwrap_or(UNKNOWN.to_string())
                })
            }
            TracksExtractor::TrackLinks(_) => {
                return Err(NotAvailableForVariant {
                    action: "return added_at information",
                    variant_label: "TracksExtractor::TrackLinks variant",
                }
                .into());
            }
        };
        Ok(preview_urls)
    }

    pub fn uris(&self) -> Vec<String> {
        const UNKNOWN_URI: &str = "spotify:track:unknown";

        match self {
            TracksExtractor::SavedTracks(tracks) => {
                collect_model_field!(
                    umap,
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
            TracksExtractor::FullTracks(tracks) => {
                collect_model_field!(
                    umap,
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
            TracksExtractor::SimplifiedTracks(tracks) => {
                collect_model_field!(
                    umap,
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
            TracksExtractor::TrackLinks(tracks) => {
                collect_model_field!(umap, tracks, |track_link: &TrackLink| track_link
                    .uri
                    .clone())
            }
        }
    }

    pub fn albums(&self) -> Result<AlbumsExtractor, SpotifyAssistantError> {
        let albums = match self {
            TracksExtractor::SavedTracks(tracks) => {
                collect_model_field!(umap, tracks, |track: &SavedTrack| {
                    track.track.album.clone()
                })
            }
            TracksExtractor::FullTracks(tracks) => {
                collect_model_field!(umap, tracks, |track: &FullTrack| { track.album.clone() })
            }
            TracksExtractor::SimplifiedTracks(tracks) => {
                collect_model_field!(umap, tracks, |track: &SimplifiedTrack| {
                    track.album.clone().unwrap_or_default()
                })
            }
            TracksExtractor::TrackLinks(_) => {
                return Err(NotAvailableForVariant {
                    action: "return an albums extractor",
                    variant_label: "TracksExtractor::TrackLinks",
                }
                .into());
            }
        };
        Ok(AlbumsExtractor::SimplifiedAlbums(albums.clone()))
    }

    pub fn album_artists(&self) -> Result<Vec<ArtistsExtractor>, SpotifyAssistantError> {
        match self.albums() {
            Ok(albums) => Ok(albums.artists()),
            Err(err) => Err(err),
        }
    }

    pub fn artists(&self) -> Result<Vec<ArtistsExtractor>, SpotifyAssistantError> {
        let artists = match self {
            TracksExtractor::SavedTracks(tracks) => {
                collect_model_field!(umap, tracks, |track: &SavedTrack| {
                    let artists = track.track.artists.clone();
                    ArtistsExtractor::SimplifiedArtists(artists)
                })
            }
            TracksExtractor::FullTracks(tracks) => {
                collect_model_field!(umap, tracks, |track: &FullTrack| {
                    let artists = track.artists.clone();
                    ArtistsExtractor::SimplifiedArtists(artists)
                })
            }
            TracksExtractor::SimplifiedTracks(tracks) => {
                collect_model_field!(umap, tracks, |track: &SimplifiedTrack| {
                    let artists = track.artists.clone();
                    ArtistsExtractor::SimplifiedArtists(artists)
                })
            }
            TracksExtractor::TrackLinks(_) => {
                return Err(NotAvailableForVariant {
                    action: "return an artists extractor",
                    variant_label: "TrackLinks",
                }
                .into());
            }
        };
        Ok(artists)
    }
}
