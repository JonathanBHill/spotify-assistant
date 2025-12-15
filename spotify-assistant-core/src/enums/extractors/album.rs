use crate::collect_model_field;
use crate::enums::extractors::artist::ArtistExtractor;
use crate::enums::extractors::helper::ExtractorHelper;
use crate::enums::extractors::track::TrackExtractor;
use rspotify::model::{
    AlbumId, FullAlbum, Image, Page, SavedAlbum, SimplifiedAlbum, SimplifiedTrack,
};

#[derive(Debug, Clone)]
pub enum AlbumExtractor {
    SavedAlbums(Vec<SavedAlbum>),
    FullAlbums(Vec<FullAlbum>),
    SimplifiedAlbums(Vec<SimplifiedAlbum>),
    PageSimplifiedAlbums(Page<SimplifiedAlbum>),
}

impl ExtractorHelper for AlbumExtractor {
    fn is_empty(&self) -> bool {
        match self {
            AlbumExtractor::SavedAlbums(albums) => albums.is_empty(),
            AlbumExtractor::FullAlbums(albums) => albums.is_empty(),
            AlbumExtractor::SimplifiedAlbums(albums) => albums.is_empty(),
            AlbumExtractor::PageSimplifiedAlbums(page) => page.items.is_empty(),
        }
    }

    fn len(&self) -> usize {
        match self {
            AlbumExtractor::SavedAlbums(albums) => albums.len(),
            AlbumExtractor::FullAlbums(albums) => albums.len(),
            AlbumExtractor::SimplifiedAlbums(albums) => albums.len(),
            AlbumExtractor::PageSimplifiedAlbums(page) => page.items.len(),
        }
    }
}
impl AlbumExtractor {
    pub fn artists(&self) -> Option<ArtistExtractor> {
        match self {
            AlbumExtractor::SavedAlbums(albums) => Some(ArtistExtractor::SimplifiedArtists(
                albums
                    .iter()
                    .map(|album| album.album.artists.clone())
                    .collect(),
            )),
            AlbumExtractor::FullAlbums(albums) => Some(ArtistExtractor::SimplifiedArtists(
                albums.iter().map(|album| album.artists.clone()).collect(),
            )),
            AlbumExtractor::SimplifiedAlbums(albums) => Some(ArtistExtractor::SimplifiedArtists(
                albums.iter().map(|album| album.artists.clone()).collect(),
            )),
            AlbumExtractor::PageSimplifiedAlbums(page) => Some(ArtistExtractor::SimplifiedArtists(
                page.items
                    .iter()
                    .map(|album| album.artists.clone())
                    .collect(),
            )),
        }
    }

    // pub fn artists_by_album(&self) -> Option<Vec<Vec<SimplifiedArtist>>> {
    //     if let Some(artists_extractor) = self.artists() {
    //         artists_extractor.full()
    //     } else {
    //         None
    //     }
    // }

    pub fn available_markets(&self) -> Option<Vec<Vec<String>>> {
        match self {
            AlbumExtractor::SavedAlbums(albums) => {
                collect_model_field!(
                    map,
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
            AlbumExtractor::FullAlbums(albums) => {
                collect_model_field!(
                    map,
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
            AlbumExtractor::SimplifiedAlbums(albums) => {
                collect_model_field!(map, albums, |album: &SimplifiedAlbum| album
                    .available_markets
                    .clone())
            }
            AlbumExtractor::PageSimplifiedAlbums(page) => {
                collect_model_field!(map, page.items, |album: &SimplifiedAlbum| album
                    .available_markets
                    .clone())
            }
        }
    }

    pub fn groups(&self) -> Option<Vec<String>> {
        match self {
            AlbumExtractor::SavedAlbums(_) | AlbumExtractor::FullAlbums(_) => None,
            AlbumExtractor::SimplifiedAlbums(albums) => {
                collect_model_field!(
                    map,
                    albums,
                    |album: &SimplifiedAlbum| album.album_group.clone(),
                    String::from("none")
                )
            }
            AlbumExtractor::PageSimplifiedAlbums(page) => {
                collect_model_field!(
                    map,
                    page.items,
                    |album: &SimplifiedAlbum| { album.album_group.clone() },
                    String::from("none")
                )
            }
        }
    }

    pub fn hrefs(&self) -> Option<Vec<String>> {
        match self {
            AlbumExtractor::SavedAlbums(albums) => {
                collect_model_field!(map, albums, |album: &SavedAlbum| album.album.href.clone())
            }
            AlbumExtractor::FullAlbums(albums) => {
                collect_model_field!(map, albums, |album: &FullAlbum| album.href.clone())
            }
            AlbumExtractor::SimplifiedAlbums(albums) => {
                collect_model_field!(
                    map,
                    albums,
                    |album: &SimplifiedAlbum| { album.href.clone() },
                    String::new()
                )
            }
            AlbumExtractor::PageSimplifiedAlbums(page) => {
                collect_model_field!(
                    map,
                    page.items,
                    |album: &SimplifiedAlbum| { album.href.clone() },
                    String::new()
                )
            }
        }
    }

    pub fn ids(&self) -> Option<Vec<AlbumId<'static>>> {
        let default_album_id =
            AlbumId::from_id("unknown").expect("Failed to create AlbumId from unknown ID");
        match self {
            AlbumExtractor::SavedAlbums(albums) => {
                collect_model_field!(map, albums, |album: &SavedAlbum| album.album.id.clone())
            }
            AlbumExtractor::FullAlbums(albums) => {
                collect_model_field!(map, albums, |album: &FullAlbum| album.id.clone())
            }
            AlbumExtractor::SimplifiedAlbums(albums) => {
                collect_model_field!(
                    map,
                    albums,
                    |album: &SimplifiedAlbum| album.id.clone(),
                    default_album_id.clone()
                )
            }
            AlbumExtractor::PageSimplifiedAlbums(page) => {
                collect_model_field!(
                    map,
                    page.items,
                    |album: &SimplifiedAlbum| album.id.clone(),
                    default_album_id.clone()
                )
            }
        }
    }

    pub fn images(&self) -> Option<Vec<Image>> {
        match self {
            AlbumExtractor::SavedAlbums(albums) => {
                collect_model_field!(
                    map,
                    albums,
                    |album: &SavedAlbum| { album.album.images.first().map(|image| image.clone()) },
                    Image::default()
                )
            }
            AlbumExtractor::FullAlbums(albums) => {
                collect_model_field!(
                    map,
                    albums,
                    |album: &FullAlbum| { album.images.first().map(|image| image.clone()) },
                    Image::default()
                )
            }
            AlbumExtractor::SimplifiedAlbums(albums) => {
                collect_model_field!(
                    map,
                    albums,
                    |album: &SimplifiedAlbum| { album.images.first().map(|image| image.clone()) },
                    Image::default()
                )
            }
            AlbumExtractor::PageSimplifiedAlbums(page) => {
                collect_model_field!(
                    map,
                    page.items,
                    |album: &SimplifiedAlbum| { album.images.first().map(|image| image.clone()) },
                    Image::default()
                )
            }
        }
    }

    pub fn image_urls(&self) -> Option<Vec<String>> {
        match self.images() {
            Some(images) => Some(images.iter().map(|image| image.url.clone()).collect()),
            None => None,
        }
    }

    pub fn image_dimensions(&self) -> Option<Vec<(u32, u32)>> {
        match self.images() {
            Some(images) => Some(
                images
                    .iter()
                    .map(|image| (image.width.unwrap_or(64), image.height.unwrap_or(64)))
                    .collect(),
            ),
            None => None,
        }
    }

    pub fn names(&self) -> Option<Vec<String>> {
        match self {
            AlbumExtractor::SavedAlbums(albums) => {
                collect_model_field!(map, albums, |album: &SavedAlbum| album.album.name.clone())
            }
            AlbumExtractor::FullAlbums(albums) => {
                collect_model_field!(map, albums, |album: &FullAlbum| album.name.clone())
            }
            AlbumExtractor::SimplifiedAlbums(albums) => {
                collect_model_field!(map, albums, |album: &SimplifiedAlbum| album.name.clone())
            }
            AlbumExtractor::PageSimplifiedAlbums(page) => {
                collect_model_field!(map, page.items, |album: &SimplifiedAlbum| album
                    .name
                    .clone())
            }
        }
    }

    pub fn release_dates(&self) -> Option<Vec<String>> {
        match self {
            AlbumExtractor::SavedAlbums(albums) => {
                collect_model_field!(map, albums, |album: &SavedAlbum| album
                    .album
                    .release_date
                    .clone())
            }
            AlbumExtractor::FullAlbums(albums) => {
                collect_model_field!(map, albums, |album: &FullAlbum| album.release_date.clone())
            }
            AlbumExtractor::SimplifiedAlbums(albums) => {
                collect_model_field!(
                    map,
                    albums,
                    |album: &SimplifiedAlbum| album.release_date.clone(),
                    String::from("unknown")
                )
            }
            AlbumExtractor::PageSimplifiedAlbums(page) => {
                collect_model_field!(
                    map,
                    page.items,
                    |album: &SimplifiedAlbum| album.release_date.clone(),
                    String::from("unknown")
                )
            }
        }
    }

    pub fn release_date_precision(&self) -> Option<Vec<String>> {
        match self {
            AlbumExtractor::SavedAlbums(albums) => {
                collect_model_field!(map, albums, |album: &SavedAlbum| {
                    Self::date_precision_as_string(&album.album.release_date_precision)
                })
            }
            AlbumExtractor::FullAlbums(albums) => {
                collect_model_field!(map, albums, |album: &FullAlbum| {
                    Self::date_precision_as_string(&album.release_date_precision)
                })
            }
            AlbumExtractor::SimplifiedAlbums(albums) => {
                collect_model_field!(
                    map,
                    albums,
                    |album: &SimplifiedAlbum| album.release_date_precision.clone(),
                    String::from("day")
                )
            }
            AlbumExtractor::PageSimplifiedAlbums(page) => {
                collect_model_field!(
                    map,
                    page.items,
                    |album: &SimplifiedAlbum| album.release_date_precision.clone(),
                    String::from("day")
                )
            }
        }
    }

    pub fn restrictions(&self) -> Option<Vec<String>> {
        match self {
            AlbumExtractor::FullAlbums(_) | AlbumExtractor::SavedAlbums(_) => None,
            AlbumExtractor::SimplifiedAlbums(albums) => {
                collect_model_field!(map, albums, |album: &SimplifiedAlbum| {
                    Self::restrict_reason_as_string(&album.restrictions)
                })
            }
            AlbumExtractor::PageSimplifiedAlbums(page) => {
                collect_model_field!(map, page.items, |album: &SimplifiedAlbum| {
                    Self::restrict_reason_as_string(&album.restrictions)
                })
            }
        }
    }

    pub fn tracks(&self) -> Option<TrackExtractor> {
        match self {
            AlbumExtractor::SavedAlbums(albums) => {
                // let albums = albums.iter().map(|album| album.album).collect::<Vec<FullAlbum>>();
                // let albums_tracks = collect_model_field!(map, albums, |album: &FullAlbum| { album.tracks.clone().items }).unwrap();
                let tracks = collect_model_field!(map, albums, |album: &SavedAlbum| {
                    album.album.tracks.clone().items
                })
                .unwrap()
                .iter()
                .flatten()
                .cloned()
                .collect();
                Some(TrackExtractor::SimplifiedTracks(tracks))
            }
            AlbumExtractor::FullAlbums(albums) => {
                let tracks = collect_model_field!(map, albums, |album: &FullAlbum| {
                    album.tracks.clone().items
                })
                .unwrap()
                .iter()
                .flatten()
                .cloned()
                .collect();
                Some(TrackExtractor::SimplifiedTracks(tracks))
            }
            AlbumExtractor::SimplifiedAlbums(_) | AlbumExtractor::PageSimplifiedAlbums(_) => None,
        }
    }
    pub fn list_tracks(&self) -> Option<Vec<Page<SimplifiedTrack>>> {
        match self {
            AlbumExtractor::SavedAlbums(albums) => {
                collect_model_field!(map, albums, |album: &SavedAlbum| {
                    album.album.tracks.clone()
                })
            }
            AlbumExtractor::FullAlbums(albums) => {
                collect_model_field!(map, albums, |album: &FullAlbum| { album.tracks.clone() })
            }
            AlbumExtractor::SimplifiedAlbums(_) | AlbumExtractor::PageSimplifiedAlbums(_) => None,
        }
    }

    pub fn types(&self) -> Option<Vec<String>> {
        match self {
            AlbumExtractor::SavedAlbums(albums) => {
                collect_model_field!(map, albums, |album: &SavedAlbum| {
                    Self::album_type_as_string(&album.album.album_type)
                })
            }
            AlbumExtractor::FullAlbums(albums) => {
                collect_model_field!(map, albums, |album: &FullAlbum| {
                    Self::album_type_as_string(&album.album_type)
                })
            }
            AlbumExtractor::SimplifiedAlbums(albums) => {
                collect_model_field!(
                    map,
                    albums,
                    |album: &SimplifiedAlbum| album.album_type.clone(),
                    String::from("none")
                )
            }
            AlbumExtractor::PageSimplifiedAlbums(page) => {
                collect_model_field!(
                    map,
                    page.items,
                    |album: &SimplifiedAlbum| album.album_type.clone(),
                    String::from("none")
                )
            }
        }
    }
}
