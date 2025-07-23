use crate::collect_track_field;
use crate::utilities::general::format_duration;
use rspotify::model::{AlbumId, ArtistId, FullTrack, Image, PlayableId, Restriction, RestrictionReason, SavedTrack, SimplifiedArtist, SimplifiedTrack, TrackId, TrackLink};
// #[macro_export]
// macro_rules! collect_track_field {
//     ($tracks:expr, $field_path:expr, $default:expr) => {
//         Some($tracks.iter().map(|track| {
//             match $field_path(track).clone() {
//                 Some(field) => field,
//                 None => $default,
//             }
//         }).collect())
//     };
//     // Variant without Option handling for direct field access
//     ($tracks:expr, $field_path:expr) => {
//         Some($tracks.iter().map(|track| $field_path(track)).collect())
//     };
// }


#[derive(Debug, Clone)]
pub enum TrackCollection {
    SavedTracks(Vec<SavedTrack>),
    FullTrack(Vec<FullTrack>),
    SimplifiedTrack(Vec<SimplifiedTrack>),
    TrackLink(Vec<TrackLink>),
}
impl TrackCollection {
    pub fn is_empty(&self) -> bool {
        match self {
            TrackCollection::SavedTracks(tracks) => tracks.is_empty(),
            TrackCollection::FullTrack(tracks) => tracks.is_empty(),
            TrackCollection::SimplifiedTrack(tracks) => tracks.is_empty(),
            TrackCollection::TrackLink(tracks) => tracks.is_empty(),
        }
    }
    pub fn len(&self) -> usize {
        match self {
            TrackCollection::SavedTracks(tracks) => tracks.len(),
            TrackCollection::FullTrack(tracks) => tracks.len(),
            TrackCollection::SimplifiedTrack(tracks) => tracks.len(),
            TrackCollection::TrackLink(tracks) => tracks.len(),
        }
    }
    pub fn playable_ids(&self) -> Option<Vec<PlayableId>> {
        match self {
            TrackCollection::SavedTracks(tracks) => {
                Some(tracks.iter().filter_map(|track| {
                    track.track.id.clone().map(|id| {
                        PlayableId::Track(id).into_static()
                    })
                }).collect())
            },
            TrackCollection::FullTrack(tracks) => {
                Some(tracks.iter().filter_map(|track| {
                    track.id.clone().map(|id| {
                        PlayableId::Track(id).into_static()
                    })
                }).collect())
            },
            TrackCollection::SimplifiedTrack(tracks) => {
                Some(tracks.iter().filter_map(|track| {
                    track.id.clone().map(|id| {
                        PlayableId::Track(id).into_static()
                    })
                }).collect())
            },
            TrackCollection::TrackLink(_) => None
        }
    }
    pub fn names(&self) -> Option<Vec<String>> {
        match self {
            TrackCollection::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| track.track.name.clone())
            },
            TrackCollection::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| track.name.clone())
            },
            TrackCollection::SimplifiedTrack(tracks) => {
                collect_track_field!(tracks, |track: &SimplifiedTrack| track.name.clone())
            },
            TrackCollection::TrackLink(_) => None
        }
    }
    pub fn ids(&self) -> Option<Vec<TrackId>> {
        let default_id = TrackId::from_id("unknown")
            .expect("Failed to create TrackId from unknown ID");
        match self {
            TrackCollection::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| track.track.id.clone(), default_id.clone())
            },
            TrackCollection::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| track.id.clone(), default_id.clone())
            },
            TrackCollection::SimplifiedTrack(tracks) => {
                collect_track_field!(tracks, |track: &SimplifiedTrack| track.id.clone(), default_id.clone())
            },
            TrackCollection::TrackLink(tracks) => {
                collect_track_field!(tracks, |track: &TrackLink| track.id.clone(), default_id.clone())
            }
        }
    }
    pub fn added_at(&self) -> Option<Vec<String>> {
        match self {
            TrackCollection::SavedTracks(tracks) => Some(tracks.iter().map(|track| {
                track.added_at.to_rfc3339()
            }).collect()),
            _ => None,
        }
    }

    pub fn available_markets(&self) -> Option<Vec<Vec<String>>> {
        match self {
            TrackCollection::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| track.track.available_markets.clone())
            },
            TrackCollection::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| track.available_markets.clone())
            },
            TrackCollection::SimplifiedTrack(tracks) => {
                collect_track_field!(tracks, |track: &SimplifiedTrack| {
                    match track.available_markets.clone() {
                        Some(markets) => markets.clone(),
                        None => vec![String::new()]
                    }
                })
            },
            TrackCollection::TrackLink(_) => None
        }
    }
    pub fn disc_numbers(&self) -> Option<Vec<u32>> {
        match self {
            TrackCollection::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| track.track.disc_number.to_string().parse::<u32>().unwrap_or(0))
            },
            TrackCollection::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| track.disc_number.to_string().parse::<u32>().unwrap_or(0))
            },
            TrackCollection::SimplifiedTrack(tracks) => {
                collect_track_field!(tracks, |track: &SimplifiedTrack| track.disc_number.to_string().parse::<u32>().unwrap_or(0))
            },
            TrackCollection::TrackLink(_) => None
        }
    }
    pub fn durations(&self) -> Option<Vec<String>> {
        match self {
            TrackCollection::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| format_duration(track.track.duration))
            },
            TrackCollection::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| format_duration(track.duration))
            },
            TrackCollection::SimplifiedTrack(tracks) => {
                collect_track_field!(tracks, |track: &SimplifiedTrack| format_duration(track.duration))
            },
            TrackCollection::TrackLink(_) => None
        }
    }
    pub fn explicit(&self) -> Option<Vec<bool>> {
        match self {
            TrackCollection::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| track.track.explicit)
            },
            TrackCollection::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| track.explicit)
            },
            TrackCollection::SimplifiedTrack(tracks) => {
                collect_track_field!(tracks, |track: &SimplifiedTrack| track.explicit)
            },
            TrackCollection::TrackLink(_) => None
        }
    }

    //todo: Implement fields that return hashmaps (external_ids, external_urls)
    pub fn hrefs(&self) -> Option<Vec<String>> {
        match self {
            TrackCollection::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| {
                    track.track.href.clone().unwrap_or_else(|| String::from("unknown"))
                })
            },
            TrackCollection::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| {
                    track.href.clone().unwrap_or_else(|| String::from("unknown"))
                })
            },
            TrackCollection::SimplifiedTrack(tracks) => {
                collect_track_field!(tracks, |track: &SimplifiedTrack| {
                    track.href.clone().unwrap_or_else(|| String::from("unknown"))
                })
            },
            TrackCollection::TrackLink(tracks) => {
                collect_track_field!(tracks, |track: &TrackLink| {
                    track.href.clone()
                })
            }
        }
    }
    pub fn is_local(&self) -> Option<Vec<bool>> {
        match self {
            TrackCollection::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| track.track.is_local)
            },
            TrackCollection::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| track.is_local)
            },
            TrackCollection::SimplifiedTrack(tracks) => {
                collect_track_field!(tracks, |track: &SimplifiedTrack| track.is_local)
            },
            TrackCollection::TrackLink(_) => None
        }
    }
    pub fn is_playable(&self) -> Option<Vec<bool>> {
        match self {
            TrackCollection::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| {
                    track.track.is_playable.unwrap_or(false)
                })
            },
            TrackCollection::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| {
                    track.is_playable.unwrap_or(false)
                })
            },
            TrackCollection::SimplifiedTrack(tracks) => {
                collect_track_field!(tracks, |track: &SimplifiedTrack| {
                    track.is_playable.unwrap_or(false)
                })
            },
            TrackCollection::TrackLink(_) => None
        }
    }
    pub fn uris(&self) -> Option<Vec<String>> {
        match self {
            TrackCollection::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| {
                    match track.clone().track.linked_from {
                        Some(linked_from) => linked_from.uri,
                        None => "spotify:track:unknown".to_string()
                    }
                })
            },
            TrackCollection::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| {
                    match track.clone().linked_from {
                        Some(linked_from) => linked_from.uri,
                        None => "spotify:track:unknown".into()
                    }
                })
            },
            TrackCollection::SimplifiedTrack(tracks) => {
                collect_track_field!(tracks, |track: &SimplifiedTrack| {
                    match track.clone().linked_from {
                        Some(linked_from) => linked_from.uri,
                        None => "spotify:track:unknown".into()
                    }
                })
            },
            TrackCollection::TrackLink(tracks) => {
                collect_track_field!(tracks, |track: &TrackLink| track.uri.clone())
            }
        }
    }

    fn restrictions_to_string_helper(
        restrictions: &Option<Restriction>
    ) -> String {
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
            TrackCollection::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| {
                    Self::restrictions_to_string_helper(&track.track.restrictions)
                })
            },
            TrackCollection::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| {
                    Self::restrictions_to_string_helper(&track.restrictions)
                })
            },
            TrackCollection::SimplifiedTrack(tracks) => {
                collect_track_field!(tracks, |track: &SimplifiedTrack| {
                    Self::restrictions_to_string_helper(&track.restrictions)
                })
            },
            TrackCollection::TrackLink(_) => None
        }
    }

    pub fn popularity(&self) -> Option<Vec<u32>> {
        match self {
            TrackCollection::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| track.track.popularity)
            },
            TrackCollection::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| track.popularity)
            },
            TrackCollection::SimplifiedTrack(_) => None,
            TrackCollection::TrackLink(_) => None
        }
    }

    pub fn preview_urls(&self) -> Option<Vec<String>> {
        match self {
            TrackCollection::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| {
                    track.track.preview_url.clone().unwrap_or_else(|| String::from("unknown"))
                })
            },
            TrackCollection::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| {
                    track.preview_url.clone().unwrap_or_else(|| String::from("unknown"))
                })
            },
            TrackCollection::SimplifiedTrack(tracks) => {
                collect_track_field!(tracks, |track: &SimplifiedTrack| {
                    track.preview_url.clone().unwrap_or_else(|| String::from("unknown"))
                })
            },
            TrackCollection::TrackLink(_) => None
        }
    }

    pub fn track_numbers(&self) -> Option<Vec<u32>> {
        match self {
            TrackCollection::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| track.track.track_number)
            },
            TrackCollection::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| track.track_number)
            },
            TrackCollection::SimplifiedTrack(tracks) => {
                collect_track_field!(tracks, |track: &SimplifiedTrack| track.track_number)
            },
            TrackCollection::TrackLink(_) => None
        }
    }
    //Region albums
    pub fn album_groups(&self) -> Option<Vec<String>> {
        match self {
            TrackCollection::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| {
                    track.track.album.album_group.clone().unwrap_or_else(|| String::from("unknown"))
                })
            },
            TrackCollection::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| {
                    track.album.album_group.clone().unwrap_or_else(|| String::from("unknown"))
                })
            },
            TrackCollection::SimplifiedTrack(tracks) => {
                collect_track_field!(tracks, |track: &SimplifiedTrack| {
                    match track.clone().album {
                        Some(album) => album.album_group.clone().unwrap_or_else(|| String::from("unknown")),
                        None => String::from("unknown")
                    }
                })
            },
            TrackCollection::TrackLink(_) => None
        }
    }
    pub fn album_types(&self) -> Option<Vec<String>> {
        match self {
            TrackCollection::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| {
                    track.track.album.album_type.clone().unwrap_or_else(|| String::from("unknown"))
                })
            },
            TrackCollection::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| {
                    track.album.album_type.clone().unwrap_or_else(|| String::from("unknown"))
                })
            },
            TrackCollection::SimplifiedTrack(tracks) => {
                collect_track_field!(tracks, |track: &SimplifiedTrack| {
                    match track.clone().album {
                        Some(album) => album.album_type.clone().unwrap_or_else(|| String::from("unknown")),
                        None => String::from("unknown")
                    }
                })
            },
            TrackCollection::TrackLink(_) => None
        }
    }
    pub fn album_artists(&self) -> Option<Vec<Vec<SimplifiedArtist>>> {
        match self {
            TrackCollection::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| {
                    track.track.album.artists.to_vec()
                })
            },
            TrackCollection::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| {
                    track.album.artists.to_vec()
                })
            },
            TrackCollection::SimplifiedTrack(tracks) => {
                collect_track_field!(tracks, |track: &SimplifiedTrack| {
                    match track.clone().album {
                        Some(album) => album.artists.to_vec(),
                        None => vec![SimplifiedArtist::default()]
                    }
                })
            },
            TrackCollection::TrackLink(_) => None
        }
    }
    pub fn album_artist_hrefs(&self) -> Option<Vec<Vec<String>>> {
        match self {
            TrackCollection::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| {
                    track.track.album.artists.iter().map(|artist| {
                        artist.href.clone().unwrap_or_else(|| String::from("unknown"))
                    }).collect()
                })
            },
            TrackCollection::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| {
                    track.album.artists.iter().map(|artist| {
                        artist.href.clone().unwrap_or_else(|| String::from("unknown"))
                    }).collect()
                })
            },
            TrackCollection::SimplifiedTrack(tracks) => {
                collect_track_field!(tracks, |track: &SimplifiedTrack| {
                    match track.clone().album {
                        Some(album) => album.artists.iter().map(|artist| {
                            artist.href.clone().unwrap_or_else(|| String::from("unknown"))
                        }).collect(),
                        None => vec![String::from("unknown")]
                    }
                })
            },
            TrackCollection::TrackLink(_) => None
        }
    }
    pub fn album_artist_names(&self) -> Option<Vec<Vec<String>>> {
        match self {
            TrackCollection::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| {
                    track.track.album.artists.iter().map(|artist| {
                        artist.name.clone()
                    }).collect()
                })
            },
            TrackCollection::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| {
                    track.album.artists.iter().map(|artist| {
                        artist.name.clone()
                    }).collect()
                })
            },
            TrackCollection::SimplifiedTrack(tracks) => {
                collect_track_field!(tracks, |track: &SimplifiedTrack| {
                    match track.album.clone() {
                        Some(album) => album.artists.iter().map(|artist| {
                            artist.name.clone()
                        }).collect(),
                        None => vec![String::from("unknown")]
                    }
                })
            },
            TrackCollection::TrackLink(_) => None
        }
    }
    pub fn album_artist_ids(&self) -> Option<Vec<Vec<ArtistId>>> {
        let default_artist_id = ArtistId::from_id("unknown")
            .expect("Failed to create ArtistId from unknown ID");
        match self {
            TrackCollection::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| {
                    track.track.album.artists.iter().map(|artist| {
                        match artist.id.clone() {
                            Some(id) => id,
                            None => default_artist_id.clone()
                        }
                    }).collect()
                })
            },
            TrackCollection::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| {
                    track.album.artists.iter().map(|artist| {
                        match artist.id.clone() {
                            Some(id) => id,
                            None => default_artist_id.clone()
                        }
                    }).collect()
                })
            },
            TrackCollection::SimplifiedTrack(tracks) => {
                collect_track_field!(tracks, |track: &SimplifiedTrack| {
                    match track.clone().album {
                        Some(album) => album.artists.iter().map(|artist| {
                            match artist.id.clone() {
                                Some(id) => id,
                                None => default_artist_id.clone()
                            }
                        }).collect(),
                        None => vec![default_artist_id.clone()]
                    }
                })
            },
            TrackCollection::TrackLink(_) => None
        }
    }
    pub fn album_available_markets(&self) -> Option<Vec<Vec<String>>> {
        match self {
            TrackCollection::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| {
                    track.track.album.available_markets.clone()
                })
            },
            TrackCollection::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| {
                    track.album.available_markets.clone()
                })
            },
            TrackCollection::SimplifiedTrack(tracks) => {
                collect_track_field!(tracks, |track: &SimplifiedTrack| {
                    match track.clone().album {
                        Some(album) => album.available_markets.clone(),
                        None => vec![String::new()]
                    }
                })
            },
            TrackCollection::TrackLink(_) => None
        }
    }
    pub fn album_hrefs(&self) -> Option<Vec<String>> {
        match self {
            TrackCollection::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| {
                    track.track.album.href.clone().unwrap_or_else(|| String::from("unknown"))
                })
            },
            TrackCollection::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| {
                    track.album.href.clone().unwrap_or_else(|| String::from("unknown"))
                })
            },
            TrackCollection::SimplifiedTrack(tracks) => {
                collect_track_field!(tracks, |track: &SimplifiedTrack| {
                    match track.clone().album {
                        Some(album) => album.href.clone().unwrap_or_else(|| String::from("unknown")),
                        None => String::from("unknown")
                    }
                })
            },
            TrackCollection::TrackLink(_) => None
        }
    }
    pub fn album_images(&self) -> Option<Vec<Vec<Image>>> {
        match self {
            TrackCollection::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| {
                    match track.track.album.images.first() {
                        Some(image) => vec![image.clone()],
                        None => vec![Image::default()]
                    }
                })
            },
            TrackCollection::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| {
                    match track.album.images.first() {
                        Some(image) => vec![image.clone()],
                        None => vec![Image::default()]
                    }
                })
            },
            TrackCollection::SimplifiedTrack(tracks) => {
                collect_track_field!(tracks, |track: &SimplifiedTrack| {
                    match track.clone().album {
                        Some(album) => {
                            match album.images.first() {
                                Some(image) => vec![image.clone()],
                                None => vec![Image::default()]
                            }}
                        None => vec![Image::default()]
                    }
                })
            },
            TrackCollection::TrackLink(_) => None
        }
    }
    pub fn album_image_urls(&self) -> Option<Vec<String>> {
        match self {
            TrackCollection::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| {
                    match track.track.album.images.first() {
                        Some(image) => image.url.clone(),
                        None => String::from("unknown")
                    }
                })
            },
            TrackCollection::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| {
                    match track.album.images.first() {
                        Some(image) => image.url.clone(),
                        None => String::from("unknown")
                    }
                })
            },
            TrackCollection::SimplifiedTrack(tracks) => {
                collect_track_field!(tracks, |track: &SimplifiedTrack| {
                    match track.clone().album {
                        Some(album) => {
                            match album.images.first() {
                                Some(image) => image.url.clone(),
                                None => String::from("unknown")
                            }}
                        None => String::from("unknown")
                    }
                })
            },
            TrackCollection::TrackLink(_) => None
        }
    }
    pub fn album_image_dimensions(&self) -> Option<Vec<Vec<(u32, u32)>>> {
        match self {
            TrackCollection::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| {
                    track.track.album.images.iter().map(|image| {
                        (image.width.unwrap_or(64), image.height.unwrap_or(64))
                    }).collect()
                })
            },
            TrackCollection::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| {
                    track.album.images.iter().map(|image| {
                        (image.width.unwrap_or(64), image.height.unwrap_or(64))
                    }).collect()
                })
            },
            TrackCollection::SimplifiedTrack(tracks) => {
                collect_track_field!(tracks, |track: &SimplifiedTrack| {
                    match track.clone().album {
                        Some(album) => album.images.iter().map(|image| {
                            (image.width.unwrap_or(64), image.height.unwrap_or(64))
                        }).collect(),
                        None => vec![(64, 64)]
                    }
                })
            },
            TrackCollection::TrackLink(_) => None
        }
    }
    pub fn album_ids(&self) -> Option<Vec<AlbumId>> {
        let default_album_id = AlbumId::from_id("unknown")
            .expect("Failed to create AlbumId from unknown ID");
        match self {
            TrackCollection::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| track.track.album.id.clone(), default_album_id.clone())
            },
            TrackCollection::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| track.album.id.clone(), default_album_id.clone())
            },
            TrackCollection::SimplifiedTrack(tracks) => {
                collect_track_field!(tracks, |track: &SimplifiedTrack| {
                    match track.clone().album {
                        Some(album) => album.id.clone(),
                        None => None
                    }
                }, default_album_id.clone())
            },
            TrackCollection::TrackLink(_) => {
                None
            }
        }
    }
    pub fn album_names(&self) -> Option<Vec<String>> {
        match self {
            TrackCollection::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| track.track.album.name.clone())
            },
            TrackCollection::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| track.album.name.clone())
            },
            TrackCollection::SimplifiedTrack(tracks) => {
                collect_track_field!(tracks, |track: &SimplifiedTrack| {
                    match track.clone().album {
                        Some(album) => album.name.clone(),
                        None => String::from("unknown")
                    }
                })
            },
            TrackCollection::TrackLink(_) => None
        }
    }
    pub fn album_release_dates(&self) -> Option<Vec<String>> {
        match self {
            TrackCollection::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| {
                    track.track.album.release_date.clone().unwrap_or_else(|| String::from("unknown"))})
            },
            TrackCollection::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| {
                    track.album.release_date.clone().unwrap_or_else(|| String::from("unknown"))
                })
            },
            TrackCollection::SimplifiedTrack(tracks) => {
                collect_track_field!(tracks, |track: &SimplifiedTrack| {
                    match track.clone().album {
                        Some(album) => album.release_date.clone().unwrap_or_else(|| String::from("unknown")),
                        None => String::from("unknown")
                    }
                })
            },
            TrackCollection::TrackLink(_) => None
        }
    }
    pub fn album_release_date_precision(&self) -> Option<Vec<String>> {
        match self {
            TrackCollection::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| {
                    track.track.album.release_date_precision.clone().unwrap_or_else(|| String::from("unknown"))
                })
            },
            TrackCollection::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| {
                    track.album.release_date_precision.clone().unwrap_or_else(|| String::from("unknown"))
                })
            },
            TrackCollection::SimplifiedTrack(tracks) => {
                collect_track_field!(tracks, |track: &SimplifiedTrack| {
                    match track.clone().album {
                        Some(album) => album.release_date_precision.clone().unwrap_or_else(|| String::from("unknown")),
                        None => String::from("unknown")
                    }
                })
            },
            TrackCollection::TrackLink(_) => None
        }
    }
    pub fn album_restrictions(&self) -> Option<Vec<String>> {
        match self {
            TrackCollection::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| {
                    match track.track.album.restrictions.clone() {
                        Some(restriction) => {
                            Self::restrictions_to_string_helper(&Some(restriction))
                        },
                        None => String::from("none")
                    }
                })
            },
            TrackCollection::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| {
                    match track.album.restrictions.clone() {
                        Some(restriction) => {
                            Self::restrictions_to_string_helper(&Some(restriction))
                        },
                        None => String::from("none")
                    }
                })
            },
            TrackCollection::SimplifiedTrack(tracks) => {
                collect_track_field!(tracks, |track: &SimplifiedTrack| {
                    match track.clone().album {
                        Some(album) => match album.restrictions.clone() {
                            Some(restriction) => {
                                Self::restrictions_to_string_helper(&Some(restriction))
                            },
                            None => String::from("none")
                        },
                        None => String::from("none")
                    }
                })
            },
            TrackCollection::TrackLink(_) => None
        }
    }
    //End albums

    //Region artists
    pub fn artists(&self) -> Option<Vec<Vec<SimplifiedArtist>>> {
        match self {
            TrackCollection::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| track.track.artists.to_vec())
            },
            TrackCollection::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| track.artists.to_vec())
            },
            TrackCollection::SimplifiedTrack(tracks) => {
                collect_track_field!(tracks, |track: &SimplifiedTrack| track.artists.to_vec())
            },
            TrackCollection::TrackLink(_) => None
        }
    }
    pub fn artist_ids(&self) -> Option<Vec<Vec<ArtistId>>> {
        match self {
            TrackCollection::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| {
                    track.track.artists.iter().map(|artist| {
                        match artist.id.clone() {
                            Some(id) => id,
                            None => ArtistId::from_id("spotify:artist:unknown")
                                .expect("Failed to create ArtistId from unknown ID")
                        }
                    }).collect::<Vec<ArtistId>>()
                })
            },
            TrackCollection::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| {
                    track.artists.iter().map(|artist| {
                        match artist.id.clone() {
                            Some(id) => id,
                            None => ArtistId::from_id("spotify:artist:unknown")
                                .expect("Failed to create ArtistId from unknown ID")
                        }
                    }).collect::<Vec<ArtistId>>()
                })
            },
            TrackCollection::SimplifiedTrack(tracks) => {
                collect_track_field!(tracks, |track: &SimplifiedTrack| {
                    track.artists.iter().map(|artist| {
                        match artist.id.clone() {
                            Some(id) => id,
                            None => ArtistId::from_id("spotify:artist:unknown")
                                .expect("Failed to create ArtistId from unknown ID")
                        }
                    }).collect::<Vec<ArtistId>>()
                })
            },
            TrackCollection::TrackLink(_) => None
        }
    }
    pub fn artist_names(&self) -> Option<Vec<Vec<String>>> {
        match self {
            TrackCollection::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| {
                    track.track.artists.iter().map(|artist| {
                        artist.name.clone()
                    }).collect()
                })
            },
            TrackCollection::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| {
                    track.artists.iter().map(|artist| {
                        artist.name.clone()
                    }).collect()
                })
            },
            TrackCollection::SimplifiedTrack(tracks) => {
                collect_track_field!(tracks, |track: &SimplifiedTrack| {
                    track.artists.iter().map(|artist| {
                        artist.name.clone()
                    }).collect()
                })
            },
            TrackCollection::TrackLink(_) => None
        }
    }
    pub fn artist_hrefs(&self) -> Option<Vec<Vec<String>>> {
        match self {
            TrackCollection::SavedTracks(tracks) => {
                collect_track_field!(tracks, |track: &SavedTrack| {
                    track.track.artists.iter().map(|artist| {
                        artist.href.clone().unwrap_or_else(|| String::from("unknown"))
                    }).collect()
                })
            },
            TrackCollection::FullTrack(tracks) => {
                collect_track_field!(tracks, |track: &FullTrack| {
                    track.artists.iter().map(|artist| {
                        artist.href.clone().unwrap_or_else(|| String::from("unknown"))
                    }).collect()
                })
            },
            TrackCollection::SimplifiedTrack(tracks) => {
                collect_track_field!(tracks, |track: &SimplifiedTrack| {
                    track.artists.iter().map(|artist| {
                        artist.href.clone().unwrap_or_else(|| String::from("unknown"))
                    }).collect()
                })
            },
            TrackCollection::TrackLink(_) => None
        }
    }

    //End artists
}
