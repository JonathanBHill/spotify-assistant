use crate::collect_track_field;
use rspotify::model::{
    AlbumId, AlbumType, DatePrecision, FullAlbum, Image, Page, RestrictionReason, SavedAlbum,
    SimplifiedAlbum, SimplifiedArtist,
};

#[derive(Debug, Clone)]
pub enum AlbumExtractor {
    SavedAlbums(Vec<SavedAlbum>),
    FullAlbums(Vec<FullAlbum>),
    SimplifiedAlbums(Vec<SimplifiedAlbum>),
    PageSimplifiedAlbums(Page<SimplifiedAlbum>),
}
impl AlbumExtractor {
    pub fn is_empty(&self) -> bool {
        match self {
            AlbumExtractor::SavedAlbums(albums) => albums.is_empty(),
            AlbumExtractor::FullAlbums(albums) => albums.is_empty(),
            AlbumExtractor::SimplifiedAlbums(albums) => albums.is_empty(),
            AlbumExtractor::PageSimplifiedAlbums(page) => page.items.is_empty(),
        }
    }
    pub fn len(&self) -> usize {
        match self {
            AlbumExtractor::SavedAlbums(albums) => albums.len(),
            AlbumExtractor::FullAlbums(albums) => albums.len(),
            AlbumExtractor::SimplifiedAlbums(albums) => albums.len(),
            AlbumExtractor::PageSimplifiedAlbums(page) => page.items.len(),
        }
    }
    pub fn groups(&self) -> Option<Vec<String>> {
        match self {
            AlbumExtractor::SavedAlbums(_) | AlbumExtractor::FullAlbums(_) => None,
            AlbumExtractor::SimplifiedAlbums(albums) => {
                collect_track_field!(albums, |album: &SimplifiedAlbum| album
                    .album_group
                    .clone()
                    .unwrap_or_else(|| String::from("none")))
            }
            AlbumExtractor::PageSimplifiedAlbums(page) => {
                let page_items = page.items.clone();
                collect_track_field!(page_items, |album: &SimplifiedAlbum| {
                    album
                        .album_group
                        .clone()
                        .unwrap_or_else(|| String::from("none"))
                })
            }
        }
    }
    pub fn types(&self) -> Option<Vec<String>> {
        match self {
            AlbumExtractor::SavedAlbums(albums) => {
                collect_track_field!(albums, |album: &SavedAlbum| {
                    match album.album.album_type.clone() {
                        AlbumType::Album => "album".to_string(),
                        AlbumType::Single => "single".to_string(),
                        AlbumType::Compilation => "compilation".to_string(),
                        AlbumType::AppearsOn => "appears_on".to_string(),
                    }
                })
            }
            AlbumExtractor::FullAlbums(albums) => {
                collect_track_field!(albums, |album: &FullAlbum| {
                    match album.album_type.clone() {
                        AlbumType::Album => "album".to_string(),
                        AlbumType::Single => "single".to_string(),
                        AlbumType::Compilation => "compilation".to_string(),
                        AlbumType::AppearsOn => "appears_on".to_string(),
                    }
                })
            }
            AlbumExtractor::SimplifiedAlbums(albums) => {
                collect_track_field!(albums, |album: &SimplifiedAlbum| {
                    album
                        .album_type
                        .clone()
                        .unwrap_or_else(|| String::from("none"))
                })
            }
            AlbumExtractor::PageSimplifiedAlbums(page) => {
                let page_items = page.items.clone();
                collect_track_field!(page_items, |album: &SimplifiedAlbum| {
                    album
                        .album_type
                        .clone()
                        .unwrap_or_else(|| String::from("none"))
                })
            }
        }
    }
    pub fn artists(&self) -> Option<Vec<Vec<SimplifiedArtist>>> {
        match self {
            AlbumExtractor::SavedAlbums(albums) => {
                collect_track_field!(albums, |album: &SavedAlbum| album.album.artists.clone())
            }
            AlbumExtractor::FullAlbums(albums) => {
                collect_track_field!(albums, |album: &FullAlbum| album.artists.clone())
            }
            AlbumExtractor::SimplifiedAlbums(albums) => {
                collect_track_field!(albums, |album: &SimplifiedAlbum| album.artists.clone())
            }
            AlbumExtractor::PageSimplifiedAlbums(page) => {
                let page_items = page.items.clone();
                collect_track_field!(page_items, |album: &SimplifiedAlbum| album.artists.clone())
            }
        }
    }
    pub fn available_markets(&self) -> Option<Vec<Vec<String>>> {
        match self {
            AlbumExtractor::SavedAlbums(albums) => {
                collect_track_field!(albums, |album: &SavedAlbum| {
                    match album.album.available_markets.clone() {
                        Some(markets) => markets.clone(),
                        None => vec![String::new()],
                    }
                })
            }
            AlbumExtractor::FullAlbums(albums) => {
                collect_track_field!(albums, |album: &FullAlbum| {
                    match album.available_markets.clone() {
                        Some(markets) => markets.clone(),
                        None => vec![String::new()],
                    }
                })
            }
            AlbumExtractor::SimplifiedAlbums(albums) => {
                collect_track_field!(albums, |album: &SimplifiedAlbum| album
                    .available_markets
                    .clone())
            }
            AlbumExtractor::PageSimplifiedAlbums(page) => {
                let page_items = page.items.clone();
                collect_track_field!(page_items, |album: &SimplifiedAlbum| album
                    .available_markets
                    .clone())
            }
        }
    }
    pub fn hrefs(&self) -> Option<Vec<String>> {
        match self {
            AlbumExtractor::SavedAlbums(albums) => {
                collect_track_field!(albums, |album: &SavedAlbum| album.album.href.clone())
            }
            AlbumExtractor::FullAlbums(albums) => {
                collect_track_field!(albums, |album: &FullAlbum| album.href.clone())
            }
            AlbumExtractor::SimplifiedAlbums(albums) => {
                collect_track_field!(albums, |album: &SimplifiedAlbum| {
                    album.href.clone().unwrap_or_else(|| String::new())
                })
            }
            AlbumExtractor::PageSimplifiedAlbums(page) => {
                let page_items = page.items.clone();
                collect_track_field!(page_items, |album: &SimplifiedAlbum| {
                    album.href.clone().unwrap_or_else(|| String::new())
                })
            }
        }
    }
    pub fn images(&self) -> Option<Vec<Image>> {
        match self {
            AlbumExtractor::SavedAlbums(albums) => {
                collect_track_field!(albums, |album: &SavedAlbum| {
                    match album.album.images.first() {
                        Some(image) => image.clone(),
                        None => Image::default(),
                    }
                })
            }
            AlbumExtractor::FullAlbums(albums) => {
                collect_track_field!(albums, |album: &FullAlbum| {
                    match album.images.first() {
                        Some(image) => image.clone(),
                        None => Image::default(),
                    }
                })
            }
            AlbumExtractor::SimplifiedAlbums(albums) => {
                collect_track_field!(albums, |album: &SimplifiedAlbum| {
                    match album.images.first() {
                        Some(image) => image.clone(),
                        None => Image::default(),
                    }
                })
            }
            AlbumExtractor::PageSimplifiedAlbums(page) => {
                let page_items = page.items.clone();
                collect_track_field!(page_items, |album: &SimplifiedAlbum| {
                    match album.images.first() {
                        Some(image) => image.clone(),
                        None => Image::default(),
                    }
                })
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
    pub fn ids(&self) -> Option<Vec<AlbumId<'static>>> {
        let default_album_id =
            AlbumId::from_id("unknown").expect("Failed to create AlbumId from unknown ID");
        match self {
            AlbumExtractor::SavedAlbums(albums) => {
                collect_track_field!(albums, |album: &SavedAlbum| album.album.id.clone())
            }
            AlbumExtractor::FullAlbums(albums) => {
                collect_track_field!(albums, |album: &FullAlbum| album.id.clone())
            }
            AlbumExtractor::SimplifiedAlbums(albums) => {
                collect_track_field!(
                    albums,
                    |album: &SimplifiedAlbum| album.id.clone(),
                    default_album_id.clone()
                )
            }
            AlbumExtractor::PageSimplifiedAlbums(page) => {
                let page_items = page.items.clone();
                collect_track_field!(
                    page_items,
                    |album: &SimplifiedAlbum| album.id.clone(),
                    default_album_id.clone()
                )
            }
        }
    }
    pub fn names(&self) -> Option<Vec<String>> {
        match self {
            AlbumExtractor::SavedAlbums(albums) => {
                collect_track_field!(albums, |album: &SavedAlbum| album.album.name.clone())
            }
            AlbumExtractor::FullAlbums(albums) => {
                collect_track_field!(albums, |album: &FullAlbum| album.name.clone())
            }
            AlbumExtractor::SimplifiedAlbums(albums) => {
                collect_track_field!(albums, |album: &SimplifiedAlbum| album.name.clone())
            }
            AlbumExtractor::PageSimplifiedAlbums(page) => {
                let page_items = page.items.clone();
                collect_track_field!(page_items, |album: &SimplifiedAlbum| album.name.clone())
            }
        }
    }
    pub fn release_dates(&self) -> Option<Vec<String>> {
        match self {
            AlbumExtractor::SavedAlbums(albums) => {
                collect_track_field!(albums, |album: &SavedAlbum| album
                    .album
                    .release_date
                    .clone())
            }
            AlbumExtractor::FullAlbums(albums) => {
                collect_track_field!(albums, |album: &FullAlbum| album.release_date.clone())
            }
            AlbumExtractor::SimplifiedAlbums(albums) => {
                collect_track_field!(albums, |album: &SimplifiedAlbum| album
                    .release_date
                    .clone()
                    .unwrap_or_else(|| String::from("unknown")))
            }
            AlbumExtractor::PageSimplifiedAlbums(page) => {
                collect_track_field!(page.items, |album: &SimplifiedAlbum| album
                    .release_date
                    .clone()
                    .unwrap_or_else(|| String::from("unknown")))
            }
        }
    }
    pub fn reelease_date_precision(&self) -> Option<Vec<String>> {
        match self {
            AlbumExtractor::SavedAlbums(albums) => {
                collect_track_field!(albums, |album: &SavedAlbum| {
                    match album.album.release_date_precision.clone() {
                        DatePrecision::Year => "year".to_string(),
                        DatePrecision::Month => "month".to_string(),
                        DatePrecision::Day => "day".to_string(),
                    }
                })
            }
            AlbumExtractor::FullAlbums(albums) => {
                collect_track_field!(albums, |album: &FullAlbum| {
                    match album.release_date_precision.clone() {
                        DatePrecision::Year => "year".to_string(),
                        DatePrecision::Month => "month".to_string(),
                        DatePrecision::Day => "day".to_string(),
                    }
                })
            }
            AlbumExtractor::SimplifiedAlbums(albums) => {
                collect_track_field!(albums, |album: &SimplifiedAlbum| album
                    .release_date_precision
                    .clone()
                    .unwrap_or_else(|| String::from("day")))
            }
            AlbumExtractor::PageSimplifiedAlbums(page) => {
                collect_track_field!(page.items, |album: &SimplifiedAlbum| album
                    .release_date_precision
                    .clone()
                    .unwrap_or_else(|| String::from("day")))
            }
        }
    }
    pub fn restrictions(&self) -> Option<Vec<String>> {
        match self {
            // AlbumExtractor::SavedAlbums(albums) => {
            //     collect_track_field!(albums, |album: &SavedAlbum| album.album)
            // },
            AlbumExtractor::FullAlbums(_) | AlbumExtractor::SavedAlbums(_) => None,
            AlbumExtractor::SimplifiedAlbums(albums) => {
                collect_track_field!(albums, |album: &SimplifiedAlbum| {
                    match album.restrictions.clone() {
                        Some(restriction) => match restriction.reason.clone() {
                            RestrictionReason::Explicit => "explicit".to_string(),
                            RestrictionReason::Market => "market".to_string(),
                            RestrictionReason::Product => "product".to_string(),
                        },
                        None => String::from("none"),
                    }
                })
            }
            AlbumExtractor::PageSimplifiedAlbums(page) => {
                collect_track_field!(page.items, |album: &SimplifiedAlbum| {
                    match album.restrictions.clone() {
                        Some(restriction) => match restriction.reason.clone() {
                            RestrictionReason::Explicit => "explicit".to_string(),
                            RestrictionReason::Market => "market".to_string(),
                            RestrictionReason::Product => "product".to_string(),
                        },
                        None => String::from("none"),
                    }
                })
            }
        }
    }
}
