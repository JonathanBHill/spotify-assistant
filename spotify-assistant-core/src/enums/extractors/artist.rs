use crate::collect_model_field;
use crate::enums::extractors::helper::ExtractorHelper;
use rspotify::model::{ArtistId, CursorBasedPage, FullArtist, Image, SimplifiedArtist};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum ArtistExtractor {
    FullArtists(Vec<Vec<FullArtist>>),
    PageFullArtists(Vec<CursorBasedPage<FullArtist>>),
    SimplifiedArtists(Vec<Vec<SimplifiedArtist>>),
}

impl ExtractorHelper for ArtistExtractor {
    fn is_empty(&self) -> bool {
        match self {
            ArtistExtractor::FullArtists(artists) => artists.is_empty(),
            ArtistExtractor::PageFullArtists(artists) => artists.is_empty(),
            ArtistExtractor::SimplifiedArtists(artists) => artists.is_empty(),
        }
    }

    fn len(&self) -> usize {
        match self {
            ArtistExtractor::FullArtists(artists) => artists.len(),
            ArtistExtractor::PageFullArtists(artists) => artists.len(),
            ArtistExtractor::SimplifiedArtists(artists) => artists.len(),
        }
    }
}
impl ArtistExtractor {
    pub fn full(&self) -> Option<Vec<Vec<SimplifiedArtist>>> {
        match self {
            ArtistExtractor::SimplifiedArtists(artists_vec) => Some(artists_vec.clone()),
            _ => None,
        }
    }

    pub fn external_urls(&self) -> Option<Vec<Vec<HashMap<String, String>>>> {
        match self {
            ArtistExtractor::FullArtists(artists_vec) => {
                collect_model_field!(map, artists_vec, |artists: &Vec<FullArtist>| {
                    artists
                        .iter()
                        .map(|artist| artist.external_urls.clone())
                        .collect::<Vec<HashMap<String, String>>>()
                })
            }
            ArtistExtractor::PageFullArtists(artists_vec) => {
                collect_model_field!(map, artists_vec, |artists: &CursorBasedPage<FullArtist>| {
                    artists
                        .items
                        .iter()
                        .map(|artist| artist.external_urls.clone())
                        .collect::<Vec<HashMap<String, String>>>()
                })
            }
            ArtistExtractor::SimplifiedArtists(artists_vec) => {
                collect_model_field!(map, artists_vec, |artists: &Vec<SimplifiedArtist>| {
                    artists
                        .iter()
                        .map(|artist| artist.external_urls.clone())
                        .collect::<Vec<HashMap<String, String>>>()
                })
            }
        }
    }

    pub fn followers(&self) -> Option<Vec<Vec<u32>>> {
        match self {
            ArtistExtractor::FullArtists(artists_vec) => {
                collect_model_field!(map, artists_vec, |artists: &Vec<FullArtist>| {
                    artists
                        .iter()
                        .map(|artist| artist.followers.total)
                        .collect::<Vec<u32>>()
                })
            }
            ArtistExtractor::PageFullArtists(artists_vec) => {
                collect_model_field!(map, artists_vec, |artists: &CursorBasedPage<FullArtist>| {
                    artists
                        .items
                        .iter()
                        .map(|artist| artist.followers.total)
                        .collect::<Vec<u32>>()
                })
            }
            ArtistExtractor::SimplifiedArtists(_) => None,
        }
    }

    pub fn genres(&self) -> Option<Vec<Vec<Vec<String>>>> {
        match self {
            ArtistExtractor::FullArtists(artists_vec) => {
                collect_model_field!(map, artists_vec, |artists: &Vec<FullArtist>| {
                    artists
                        .iter()
                        .map(|artist| artist.genres.clone())
                        .collect::<Vec<Vec<String>>>()
                })
            }
            ArtistExtractor::PageFullArtists(artists_vec) => {
                collect_model_field!(map, artists_vec, |artists: &CursorBasedPage<FullArtist>| {
                    artists
                        .items
                        .iter()
                        .map(|artist| artist.genres.clone())
                        .collect::<Vec<Vec<String>>>()
                })
            }
            ArtistExtractor::SimplifiedArtists(_) => None,
        }
    }

    pub fn hrefs(&self) -> Option<Vec<Vec<String>>> {
        match self {
            ArtistExtractor::FullArtists(artists_vec) => {
                collect_model_field!(map, artists_vec, |artists: &Vec<FullArtist>| {
                    artists
                        .iter()
                        .map(|artist| artist.href.clone())
                        .collect::<Vec<String>>()
                })
            }
            ArtistExtractor::PageFullArtists(artists_vec) => {
                collect_model_field!(map, artists_vec, |artists: &CursorBasedPage<FullArtist>| {
                    artists
                        .items
                        .iter()
                        .map(|artist| artist.href.clone())
                        .collect::<Vec<String>>()
                })
            }
            ArtistExtractor::SimplifiedArtists(artists_vec) => {
                collect_model_field!(map, artists_vec, |artists: &Vec<SimplifiedArtist>| {
                    artists
                        .iter()
                        .map(|artist| artist.href.clone().unwrap_or_else(|| String::from("none")))
                        .collect::<Vec<String>>()
                })
            }
        }
    }

    pub fn ids(&self) -> Option<Vec<Vec<ArtistId<'_>>>> {
        match self {
            ArtistExtractor::FullArtists(artists_vec) => {
                collect_model_field!(map, artists_vec, |artists: &Vec<FullArtist>| {
                    artists
                        .iter()
                        .map(|artist| artist.id.clone())
                        .collect::<Vec<ArtistId<'_>>>()
                })
            }
            ArtistExtractor::PageFullArtists(artists_vec) => {
                collect_model_field!(map, artists_vec, |artists: &CursorBasedPage<FullArtist>| {
                    artists
                        .items
                        .iter()
                        .map(|artist| artist.id.clone())
                        .collect::<Vec<ArtistId<'_>>>()
                })
            }
            ArtistExtractor::SimplifiedArtists(artists_vec) => {
                collect_model_field!(map, artists_vec, |artists: &Vec<SimplifiedArtist>| {
                    artists
                        .iter()
                        .map(|artist: &SimplifiedArtist| {
                            artist.id.clone().unwrap_or_else(|| {
                                ArtistId::from_id("unknown")
                                    .expect("Failed to create ArtistId from unknown ID")
                            })
                        })
                        .collect::<Vec<ArtistId<'_>>>()
                })
            }
        }
    }

    pub fn images(&self) -> Option<Vec<Vec<Vec<Image>>>> {
        match self {
            ArtistExtractor::FullArtists(artists_vec) => {
                collect_model_field!(map, artists_vec, |artists: &Vec<FullArtist>| {
                    artists
                        .iter()
                        .map(|artist| artist.images.clone())
                        .collect::<Vec<Vec<Image>>>()
                })
            }
            ArtistExtractor::PageFullArtists(artists_vec) => {
                collect_model_field!(map, artists_vec, |artists: &CursorBasedPage<FullArtist>| {
                    artists
                        .items
                        .iter()
                        .map(|artist| artist.images.clone())
                        .collect::<Vec<Vec<Image>>>()
                })
            }
            ArtistExtractor::SimplifiedArtists(_) => None,
        }
    }

    pub fn images_urls(&self) -> Option<Vec<Vec<Vec<String>>>> {
        match self.images() {
            Some(all_images) => Some(
                all_images
                    .iter()
                    .map(|unit_images| {
                        unit_images
                            .iter()
                            .map(|artist_images| {
                                artist_images
                                    .iter()
                                    .map(|image| image.url.clone())
                                    .collect::<Vec<String>>()
                            })
                            .collect()
                    })
                    .collect(),
            ),
            None => None,
        }
    }

    pub fn images_dimensions(&self) -> Option<Vec<Vec<Vec<(u32, u32)>>>> {
        match self.images() {
            Some(all_images) => Some(
                all_images
                    .iter()
                    .map(|unit_images| {
                        unit_images
                            .iter()
                            .map(|artist_images| {
                                artist_images
                                    .iter()
                                    .map(|image| {
                                        (image.width.unwrap_or(64), image.height.unwrap_or(64))
                                    })
                                    .collect::<Vec<(u32, u32)>>()
                            })
                            .collect()
                    })
                    .collect(),
            ),
            None => None,
        }
    }

    pub fn names(&self) -> Option<Vec<Vec<String>>> {
        match self {
            ArtistExtractor::FullArtists(artists_vec) => {
                collect_model_field!(map, artists_vec, |artists: &Vec<FullArtist>| artists
                    .iter()
                    .map(|artist| artist.name.clone())
                    .collect::<Vec<String>>())
            }
            ArtistExtractor::PageFullArtists(artists_vec) => {
                collect_model_field!(map, artists_vec, |artists: &CursorBasedPage<FullArtist>| {
                    artists
                        .items
                        .iter()
                        .map(|artist| artist.name.clone())
                        .collect::<Vec<String>>()
                })
            }
            ArtistExtractor::SimplifiedArtists(artists_vec) => {
                collect_model_field!(map, artists_vec, |artists: &Vec<SimplifiedArtist>| artists
                    .iter()
                    .map(|artist| artist.name.clone())
                    .collect::<Vec<String>>())
            }
        }
    }

    pub fn popularity(&self) -> Option<Vec<Vec<u32>>> {
        match self {
            ArtistExtractor::FullArtists(artists_vec) => {
                collect_model_field!(map, artists_vec, |artists: &Vec<FullArtist>| artists
                    .iter()
                    .map(|artist| artist.popularity)
                    .collect::<Vec<u32>>())
            }
            ArtistExtractor::PageFullArtists(artists_vec) => {
                collect_model_field!(map, artists_vec, |artists: &CursorBasedPage<FullArtist>| {
                    artists
                        .items
                        .iter()
                        .map(|artist| artist.popularity)
                        .collect::<Vec<u32>>()
                })
            }
            ArtistExtractor::SimplifiedArtists(_) => None,
        }
    }
}
