use rspotify::model::{FullAlbum, FullArtist, SimplifiedAlbum, SimplifiedArtist};

pub trait ToSimplifiedAlbum {
    fn to_simplified_album(&self) -> SimplifiedAlbum;
}

impl ToSimplifiedAlbum for FullAlbum {
    fn to_simplified_album(&self) -> SimplifiedAlbum {
        SimplifiedAlbum {
            // Not present on FullAlbum
            album_group: None,
            restrictions: None,

            // Type differences (FullAlbum uses enums; SimplifiedAlbum uses Option<String>)
            album_type: Some(format!("{:?}", self.album_type)),
            release_date_precision: Some(format!("{:?}", self.release_date_precision)),

            // Straightforward copies
            artists: self.artists.clone(),
            external_urls: self.external_urls.clone(),
            images: self.images.clone(),
            name: self.name.clone(),

            // Present on FullAlbum but optional on SimplifiedAlbum
            href: Some(self.href.clone()),
            id: Some(self.id.clone()),
            release_date: Some(self.release_date.clone()),

            // Optional on FullAlbum but required on SimplifiedAlbum
            available_markets: self.available_markets.clone().unwrap_or_default(),
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
