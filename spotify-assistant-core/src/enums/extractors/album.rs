use crate::collect_model_field;
use crate::enums::extractors::artist::ArtistsExtractor;
use crate::enums::extractors::helper::ExtractorHelper;
use crate::enums::extractors::track::TracksExtractor;
use crate::errors::SpotifyAssistantError;
use crate::errors::enums::EnumError::NotAvailableForVariant;
use rspotify::model::{AlbumId, FullAlbum, Image, SavedAlbum, SimplifiedAlbum};

#[derive(Debug, Clone)]
pub enum AlbumsExtractor {
    SavedAlbums(Vec<SavedAlbum>),
    FullAlbums(Vec<FullAlbum>),
    SimplifiedAlbums(Vec<SimplifiedAlbum>),
}

impl ExtractorHelper for AlbumsExtractor {
    fn is_empty(&self) -> bool {
        match self {
            AlbumsExtractor::SavedAlbums(albums) => albums.is_empty(),
            AlbumsExtractor::FullAlbums(albums) => albums.is_empty(),
            AlbumsExtractor::SimplifiedAlbums(albums) => albums.is_empty(),
        }
    }

    fn len(&self) -> usize {
        match self {
            AlbumsExtractor::SavedAlbums(albums) => albums.len(),
            AlbumsExtractor::FullAlbums(albums) => albums.len(),
            AlbumsExtractor::SimplifiedAlbums(albums) => albums.len(),
        }
    }
}
impl AlbumsExtractor {
    pub fn artists(&self) -> Vec<ArtistsExtractor> {
        let artists_vec = match self {
            AlbumsExtractor::SavedAlbums(albums) => {
                collect_model_field!(umap, albums, |album: &SavedAlbum| album
                    .album
                    .artists
                    .clone())
            }
            AlbumsExtractor::FullAlbums(albums) => {
                collect_model_field!(umap, albums, |album: &FullAlbum| album.artists.clone())
            }
            AlbumsExtractor::SimplifiedAlbums(albums) => {
                collect_model_field!(umap, albums, |album: &SimplifiedAlbum| album
                    .artists
                    .clone())
            }
        };
        artists_vec
            .into_iter()
            .map(|album_artists| ArtistsExtractor::SimplifiedArtists(album_artists))
            .collect()
    }

    // pub fn artists_by_album(&self) -> Option<Vec<Vec<SimplifiedArtist>>> {
    //     if let Some(artists_extractor) = self.artists() {
    //         artists_extractor.full()
    //     } else {
    //         None
    //     }
    // }

    pub fn available_markets(&self) -> Vec<Vec<String>> {
        match self {
            AlbumsExtractor::SavedAlbums(albums) => {
                collect_model_field!(
                    umap,
                    albums,
                    |album: &SavedAlbum| {
                        album
                            .album
                            .available_markets
                            .as_ref()
                            .map(|markets| markets.clone())
                    },
                    vec![String::new()]
                )
            }
            AlbumsExtractor::FullAlbums(albums) => {
                collect_model_field!(
                    umap,
                    albums,
                    |album: &FullAlbum| {
                        album
                            .available_markets
                            .as_ref()
                            .map(|markets| markets.clone())
                    },
                    vec![String::new()]
                )
            }
            AlbumsExtractor::SimplifiedAlbums(albums) => {
                collect_model_field!(umap, albums, |album: &SimplifiedAlbum| album
                    .available_markets
                    .clone())
            }
        }
    }

    pub fn groups(&self) -> Result<Vec<String>, SpotifyAssistantError> {
        match self {
            AlbumsExtractor::SavedAlbums(_) | AlbumsExtractor::FullAlbums(_) => return Err(NotAvailableForVariant {
                action: "return the album group for the spotify object",
                variant_label: "[AlbumsExtractor::SavedAlbums, AlbumsExtractor::FullAlbums] variants",
            }.into()),
            AlbumsExtractor::SimplifiedAlbums(albums) => {
                let groups = collect_model_field!(
                    umap,
                    albums,
                    |album: &SimplifiedAlbum| album.album_group.clone(),
                    String::from("none")
                );
                Ok(groups)
            }
        }
    }

    pub fn hrefs(&self) -> Vec<String> {
        match self {
            AlbumsExtractor::SavedAlbums(albums) => {
                collect_model_field!(umap, albums, |album: &SavedAlbum| album.album.href.clone())
            }
            AlbumsExtractor::FullAlbums(albums) => {
                collect_model_field!(umap, albums, |album: &FullAlbum| album.href.clone())
            }
            AlbumsExtractor::SimplifiedAlbums(albums) => {
                collect_model_field!(
                    umap,
                    albums,
                    |album: &SimplifiedAlbum| { album.href.clone() },
                    String::new()
                )
            }
        }
    }

    pub fn ids(&self) -> Vec<AlbumId<'static>> {
        let default_album_id =
            AlbumId::from_id("unknown").expect("Failed to create AlbumId from unknown ID");
        match self {
            AlbumsExtractor::SavedAlbums(albums) => {
                collect_model_field!(umap, albums, |album: &SavedAlbum| album.album.id.clone())
            }
            AlbumsExtractor::FullAlbums(albums) => {
                collect_model_field!(umap, albums, |album: &FullAlbum| album.id.clone())
            }
            AlbumsExtractor::SimplifiedAlbums(albums) => {
                collect_model_field!(
                    umap,
                    albums,
                    |album: &SimplifiedAlbum| album.id.clone(),
                    default_album_id.clone()
                )
            }
        }
    }

    pub fn images(&self) -> Vec<Image> {
        match self {
            AlbumsExtractor::SavedAlbums(albums) => {
                collect_model_field!(
                    umap,
                    albums,
                    |album: &SavedAlbum| { album.album.images.first().map(|image| image.clone()) },
                    Image::default()
                )
            }
            AlbumsExtractor::FullAlbums(albums) => {
                collect_model_field!(
                    umap,
                    albums,
                    |album: &FullAlbum| { album.images.first().map(|image| image.clone()) },
                    Image::default()
                )
            }
            AlbumsExtractor::SimplifiedAlbums(albums) => {
                collect_model_field!(
                    umap,
                    albums,
                    |album: &SimplifiedAlbum| { album.images.first().map(|image| image.clone()) },
                    Image::default()
                )
            }
        }
    }

    pub fn image_urls(&self) -> Vec<String> {
        self.images()
            .iter()
            .map(|image| image.url.clone())
            .collect()
    }

    pub fn image_dimensions(&self) -> Vec<(u32, u32)> {
        self.images()
            .iter()
            .map(|image| (image.width.unwrap_or(64), image.height.unwrap_or(64)))
            .collect()
    }

    pub fn names(&self) -> Vec<String> {
        match self {
            AlbumsExtractor::SavedAlbums(albums) => {
                collect_model_field!(umap, albums, |album: &SavedAlbum| album.album.name.clone())
            }
            AlbumsExtractor::FullAlbums(albums) => {
                collect_model_field!(umap, albums, |album: &FullAlbum| album.name.clone())
            }
            AlbumsExtractor::SimplifiedAlbums(albums) => {
                collect_model_field!(umap, albums, |album: &SimplifiedAlbum| album.name.clone())
            }
        }
    }

    pub fn release_dates(&self) -> Vec<String> {
        match self {
            AlbumsExtractor::SavedAlbums(albums) => {
                collect_model_field!(umap, albums, |album: &SavedAlbum| album
                    .album
                    .release_date
                    .clone())
            }
            AlbumsExtractor::FullAlbums(albums) => {
                collect_model_field!(umap, albums, |album: &FullAlbum| album.release_date.clone())
            }
            AlbumsExtractor::SimplifiedAlbums(albums) => {
                collect_model_field!(
                    umap,
                    albums,
                    |album: &SimplifiedAlbum| album.release_date.clone(),
                    String::from("unknown")
                )
            }
        }
    }

    pub fn release_date_precision(&self) -> Vec<String> {
        match self {
            AlbumsExtractor::SavedAlbums(albums) => {
                collect_model_field!(umap, albums, |album: &SavedAlbum| {
                    Self::date_precision_as_string(&album.album.release_date_precision)
                })
            }
            AlbumsExtractor::FullAlbums(albums) => {
                collect_model_field!(umap, albums, |album: &FullAlbum| {
                    Self::date_precision_as_string(&album.release_date_precision)
                })
            }
            AlbumsExtractor::SimplifiedAlbums(albums) => {
                collect_model_field!(
                    umap,
                    albums,
                    |album: &SimplifiedAlbum| album.release_date_precision.clone(),
                    String::from("day")
                )
            }
        }
    }

    pub fn restrictions(&self) -> Result<Vec<String>, SpotifyAssistantError> {
        match self {
            AlbumsExtractor::SavedAlbums(_) | AlbumsExtractor::FullAlbums(_) => return Err(NotAvailableForVariant {
                action: "return the album group for the spotify object",
                variant_label: "[AlbumsExtractor::SavedAlbums, AlbumsExtractor::FullAlbums] variants",
            }.into()),
            AlbumsExtractor::SimplifiedAlbums(albums) => {
                let restrictions = collect_model_field!(umap, albums, |album: &SimplifiedAlbum| {
                    Self::restrict_reason_as_string(&album.restrictions)
                });
                Ok(restrictions)
            }
        }
    }

    pub fn tracks(&self) -> Result<Vec<TracksExtractor>, SpotifyAssistantError> {
        match self {
            AlbumsExtractor::SavedAlbums(albums) => {
                let full_album = albums
                    .iter()
                    .map(|album| album.album.clone())
                    .collect::<Vec<FullAlbum>>();
                let album_track_extractor = full_album
                    .iter()
                    .map(|album| {
                        let tracks = album.tracks.clone().items;
                        TracksExtractor::SimplifiedTracks(tracks)
                    })
                    .collect();
                Ok(album_track_extractor)
            }
            AlbumsExtractor::FullAlbums(albums) => {
                let album_track_extractor = albums
                    .iter()
                    .map(|album| {
                        let tracks = album.tracks.clone().items;
                        TracksExtractor::SimplifiedTracks(tracks)
                    })
                    .collect();
                Ok(album_track_extractor)
            }
            AlbumsExtractor::SimplifiedAlbums(_) => Err(NotAvailableForVariant {
                action: "return a tracks extractor",
                variant_label: "AlbumsExtractor::SimplifiedAlbums",
            }
            .into()),
        }
    }

    pub fn types(&self) -> Vec<String> {
        match self {
            AlbumsExtractor::SavedAlbums(albums) => {
                collect_model_field!(umap, albums, |album: &SavedAlbum| {
                    Self::album_type_as_string(&album.album.album_type)
                })
            }
            AlbumsExtractor::FullAlbums(albums) => {
                collect_model_field!(umap, albums, |album: &FullAlbum| {
                    Self::album_type_as_string(&album.album_type)
                })
            }
            AlbumsExtractor::SimplifiedAlbums(albums) => {
                collect_model_field!(
                    umap,
                    albums,
                    |album: &SimplifiedAlbum| album.album_type.clone(),
                    String::from("none")
                )
            }
        }
    }
}
