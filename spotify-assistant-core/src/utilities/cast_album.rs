use rspotify::model::{AlbumType, DatePrecision, FullAlbum, FullArtist, SimplifiedAlbum, SimplifiedArtist};

pub trait ToSimplifiedAlbum {
    fn to_simplified_album(&self) -> SimplifiedAlbum;
}

impl ToSimplifiedAlbum for FullAlbum {
    fn to_simplified_album(&self) -> SimplifiedAlbum {
        let album_type = match self.album_type {
            AlbumType::Album => "album",
            AlbumType::Single => "single",
            AlbumType::Compilation => "compilation",
            AlbumType::AppearsOn => "appears_on",
        };

        let release_date_precision = match self.release_date_precision {
            DatePrecision::Year => "year",
            DatePrecision::Month => "month",
            DatePrecision::Day => "day",
        };

        SimplifiedAlbum {
            album_type: Some(album_type.to_string()),
            artists: self.artists.clone(),
            external_urls: self.external_urls.clone(),
            href: Some(self.href.clone()),
            id: Some(self.id.clone()),
            images: self.images.clone(),
            name: self.name.clone(),
            release_date: Some(self.release_date.clone()),
            release_date_precision: Some(release_date_precision.to_string()),
            ..Default::default()
        }
    }
}

pub trait ToSimplifiedArtist {
    fn to_simplified_artist(&self) -> SimplifiedArtist;
}

impl ToSimplifiedArtist for FullArtist {
    fn to_simplified_artist(&self) -> SimplifiedArtist {
        SimplifiedArtist {
            external_urls: self.external_urls.clone(),
            href: Option::from(self.href.clone()),
            id: Some(self.id.clone()),
            name: self.name.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::offline::OfflineObjects;

    #[test]
    fn full_album_to_simplified_preserves_supported_fields() {
        let (_, full_album) = OfflineObjects::sample_full_album();

        let simplified = full_album.to_simplified_album();

        assert_eq!(simplified.id, Some(full_album.id.clone()));
        assert_eq!(simplified.name, full_album.name);
        assert_eq!(simplified.artists, full_album.artists);
        assert_eq!(simplified.album_type.as_deref(), Some("album"));
        assert_eq!(simplified.release_date_precision.as_deref(), Some("day"));
    }

    #[test]
    fn full_album_without_available_markets_still_converts() {
        let (_, full_album) = OfflineObjects::sample_full_album();

        let mut payload = serde_json::to_value(full_album).unwrap();

        payload.as_object_mut().unwrap().remove("available_markets");

        let album: FullAlbum = serde_json::from_value(payload).unwrap();
        let simplified = album.to_simplified_album();

        assert_eq!(simplified.name, album.name);
        assert_eq!(simplified.id, Some(album.id));
    }
}
