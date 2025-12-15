use crate::collect_model_field;
use crate::enums::extractors::helper::ExtractorHelper;
use crate::errors::SpotifyAssistantError;
use crate::errors::enums::EnumError::NotAvailableForVariant;
use rspotify::model::{ArtistId, FullArtist, Image, SimplifiedArtist};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum ArtistsExtractor {
    FullArtists(Vec<FullArtist>),
    SimplifiedArtists(Vec<SimplifiedArtist>),
}

impl ExtractorHelper for ArtistsExtractor {
    fn is_empty(&self) -> bool {
        match self {
            ArtistsExtractor::FullArtists(artists) => artists.is_empty(),
            ArtistsExtractor::SimplifiedArtists(artists) => artists.is_empty(),
        }
    }

    fn len(&self) -> usize {
        match self {
            ArtistsExtractor::FullArtists(artists) => artists.len(),
            ArtistsExtractor::SimplifiedArtists(artists) => artists.len(),
        }
    }
}
impl ArtistsExtractor {
    pub fn full(&self) -> Result<Vec<SimplifiedArtist>, SpotifyAssistantError> {
        match self {
            ArtistsExtractor::SimplifiedArtists(artists_vec) => Ok(artists_vec.clone()),
            ArtistsExtractor::FullArtists(_) => Err(NotAvailableForVariant {
                action: "return the artists associated with the spotify object",
                variant_label: "ArtistsExtractor::FullArtists",
            }
            .into()),
        }
    }

    pub fn external_urls(&self) -> Result<Vec<HashMap<String, String>>, SpotifyAssistantError> {
        let urls = match self {
            ArtistsExtractor::FullArtists(artists_vec) => {
                collect_model_field!(umap, artists_vec, |artist: &FullArtist| {
                    artist.external_urls.clone()
                })
            }
            ArtistsExtractor::SimplifiedArtists(artists_vec) => {
                collect_model_field!(umap, artists_vec, |artist: &SimplifiedArtist| {
                    artist.external_urls.clone()
                })
            }
        };
        Ok(urls)
    }

    pub fn followers(&self) -> Result<Vec<u32>, SpotifyAssistantError> {
        let followers = match self {
            ArtistsExtractor::FullArtists(artists_vec) => {
                collect_model_field!(umap, artists_vec, |artist: &FullArtist| {
                    artist.followers.total
                })
            }
            ArtistsExtractor::SimplifiedArtists(_) => {
                return Err(NotAvailableForVariant {
                    action: "return the follower count for the artists",
                    variant_label: "ArtistsExtractor::SimplifiedArtists variant",
                }
                .into());
            }
        };
        Ok(followers)
    }

    pub fn genres(&self) -> Result<Vec<Vec<String>>, SpotifyAssistantError> {
        let genres = match self {
            ArtistsExtractor::FullArtists(artists_vec) => {
                collect_model_field!(umap, artists_vec, |artist: &FullArtist| {
                    artist.genres.clone()
                })
            }
            ArtistsExtractor::SimplifiedArtists(_) => {
                return Err(NotAvailableForVariant {
                    action: "return the genres for the artists",
                    variant_label: "ArtistsExtractor::SimplifiedArtists variant",
                }
                .into());
            }
        };
        Ok(genres)
    }

    pub fn hrefs(&self) -> Vec<String> {
        match self {
            ArtistsExtractor::FullArtists(artists_vec) => {
                collect_model_field!(umap, artists_vec, |artist: &FullArtist| {
                    artist.href.clone()
                })
            }
            ArtistsExtractor::SimplifiedArtists(artists_vec) => {
                collect_model_field!(umap, artists_vec, |artist: &SimplifiedArtist| {
                    artist.href.clone().unwrap_or_else(|| String::from("none"))
                })
            }
        }
    }

    pub fn ids(&self) -> Vec<ArtistId<'_>> {
        match self {
            ArtistsExtractor::FullArtists(artists_vec) => {
                collect_model_field!(umap, artists_vec, |artist: &FullArtist| {
                    artist.id.clone()
                })
            }
            ArtistsExtractor::SimplifiedArtists(artists_vec) => {
                collect_model_field!(umap, artists_vec, |artist: &SimplifiedArtist| {
                    artist.id.clone().unwrap_or_else(|| {
                        ArtistId::from_id("unknown")
                            .expect("Failed to create ArtistId from unknown ID")
                    })
                })
            }
        }
    }
    //
    pub fn images(&self) -> Result<Vec<Image>, SpotifyAssistantError> {
        let images = match self {
            ArtistsExtractor::FullArtists(artists_vec) => {
                collect_model_field!(umap, artists_vec, |artist: &FullArtist| {
                    artist
                        .images
                        .first()
                        .cloned()
                        .unwrap_or_else(|| Image::default())
                })
                .into_iter()
                .collect()
            }
            ArtistsExtractor::SimplifiedArtists(_) => {
                return Err(NotAvailableForVariant {
                    action: "return the popularity of the spotify object",
                    variant_label: "ArtistsExtractor::SimplifiedArtists variant",
                }
                .into());
            }
        };
        Ok(images)
    }

    pub fn images_urls(&self) -> Result<Vec<String>, SpotifyAssistantError> {
        let urls = match self.images() {
            Ok(images) => images
                .iter()
                .map(|image: &Image| image.url.clone())
                .collect(),
            Err(err) => return Err(err),
        };
        Ok(urls)
    }

    pub fn images_dimensions(&self) -> Result<Vec<(u32, u32)>, SpotifyAssistantError> {
        let dimensions = match self.images() {
            Ok(images) => images
                .iter()
                .map(|image: &Image| (image.width.unwrap_or(64), image.height.unwrap_or(64)))
                .collect(),
            Err(err) => return Err(err),
        };
        Ok(dimensions)
    }
    //
    pub fn names(&self) -> Vec<String> {
        match self {
            ArtistsExtractor::FullArtists(artists_vec) => {
                collect_model_field!(umap, artists_vec, |artist: &FullArtist| {
                    artist.name.clone()
                })
            }
            ArtistsExtractor::SimplifiedArtists(artists_vec) => {
                collect_model_field!(umap, artists_vec, |artist: &SimplifiedArtist| {
                    artist.name.clone()
                })
            }
        }
    }

    pub fn popularity(&self) -> Result<Vec<u32>, SpotifyAssistantError> {
        let popularity = match self {
            ArtistsExtractor::FullArtists(artists_vec) => {
                collect_model_field!(umap, artists_vec, |artist: &FullArtist| {
                    artist.popularity
                })
            }
            ArtistsExtractor::SimplifiedArtists(_) => {
                return Err(NotAvailableForVariant {
                    action: "return the popularity of the spotify object",
                    variant_label: "ArtistsExtractor::SimplifiedArtists variant",
                }
                .into());
            }
        };
        Ok(popularity)
    }
}
