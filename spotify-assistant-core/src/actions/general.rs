use crate::traits::apis::Api;
use rspotify::clients::BaseClient;
use rspotify::model::{AlbumId, ArtistId, FullAlbum, FullArtist, FullPlaylist, FullTrack, PlaylistId, TrackId};
use rspotify::{scopes, ClientError};

#[derive(Debug)]
pub struct FullProfiles {
    client: rspotify::AuthCodeSpotify,
}
impl Api for FullProfiles {
    fn select_scopes() -> std::collections::HashSet<std::string::String> {
        scopes!("user-library-read", "user-library-modify")
    }
}
impl FullProfiles {
    pub async fn new() -> Self {
        FullProfiles {
            client: Self::set_up_client(false, Some(Self::select_scopes())).await,
        }
    }
    pub async fn artist(&self, artist_id: String) -> FullArtist {
        let artist_id = match ArtistId::from_id(artist_id) {
            Ok(id) => { id }
            Err(err) => { panic!("Error: {:?}", err) }
        };
        match self.client.artist(artist_id).await {
            Ok(artist) => { artist }
            Err(err) => { panic!("Error: {:?}", err) }
        }
    }
    pub async fn album(&self, album_id: AlbumId<'static>) -> Result<FullAlbum, ClientError> {
        self.client.album(album_id, Some(Self::market())).await
    }
    pub async fn track(&self, track_id: TrackId<'static>) -> Result<FullTrack, ClientError> {
        self.client.track(track_id, Some(Self::market())).await
    }
    pub async fn playlist(&self, playlist_id: PlaylistId<'static>) -> Result<FullPlaylist, ClientError> {
        self.client.playlist(playlist_id, None, Some(Self::market())).await
    }
}
