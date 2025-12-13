use crate::actions::exploration::playlist::PlaylistXplr;
use crate::enums::pl::PlaylistType;
use crate::traits::apis::Api;
use rspotify::clients::OAuthClient;
use rspotify::model::{Id, PlayableId, PlaylistId};
use rspotify::{AuthCodeSpotify, scopes};
use std::collections::HashSet;
use tracing::{debug, debug_span, error, info};

#[derive(Debug, Clone)]
pub struct Modifier {
    client: AuthCodeSpotify,
    ref_pl_xplorer: PlaylistXplr,
    target_pl_xplorer: PlaylistXplr,
}

impl Api for Modifier {
    fn select_scopes() -> HashSet<String> {
        scopes!(
            "playlist-read-private",
            "playlist-read-collaborative",
            "playlist-modify-public",
            "playlist-modify-private"
        )
    }
}

impl Modifier {
    pub async fn new(
        ref_playlist_id: PlaylistId<'static>,
        target_playlist_id: PlaylistId<'static>,
    ) -> Self {
        let client = Self::set_up_client(false, Some(Self::select_scopes())).await;
        let ref_pl_xplorer = PlaylistXplr::new(ref_playlist_id, true).await;
        let target_pl_xplorer = PlaylistXplr::new(target_playlist_id, true).await;
        Modifier {
            client,
            ref_pl_xplorer,
            target_pl_xplorer,
        }
    }
    pub async fn release_radar() -> Self {
        let client = Self::set_up_client(false, Some(Self::select_scopes())).await;
        let stock_rr_id = PlaylistType::StockRR.get_id();
        let my_rr_id = PlaylistType::MyRR.get_id();
        let ref_pl_xplorer = PlaylistXplr::new(stock_rr_id, true).await;
        let target_pl_xplorer = PlaylistXplr::new(my_rr_id, true).await;
        Modifier {
            client,
            ref_pl_xplorer,
            target_pl_xplorer,
        }
    }
    pub async fn lagging_release_radar() -> Self {
        let client = Self::set_up_client(false, Some(Self::select_scopes())).await;
        let my_rr_id = PlaylistType::MyRR.get_id();
        let lagging_rr_id = PlaylistType::MyLaggingRR.get_id();
        let ref_pl_xplorer = PlaylistXplr::new(my_rr_id, true).await;
        let target_pl_xplorer = PlaylistXplr::new(lagging_rr_id, true).await;
        Modifier {
            client,
            ref_pl_xplorer,
            target_pl_xplorer,
        }
    }
    pub async fn update_playlist(&self) {
        let _update_pl_span = debug_span!("update-playlist").entered();
        let track_ids = self.ref_pl_xplorer.track_ids_expanded().await;
        let ids_len = track_ids.len();
        self.check_if_stock_release_radar_id_was_used(ids_len);
        let mut first_chunk = true;
        let mut count = 1;

        for chunk in track_ids.chunks(20) {
            debug!(
                "On chunk {:?}/{:?}",
                count,
                (ids_len as f32 / 20.0).ceil() as usize
            );
            let chunk_iterated = chunk
                .iter()
                .map(|track| PlayableId::Track(track.as_ref()))
                .collect();
            first_chunk = self
                .update_playlist_from_chunk(chunk_iterated, first_chunk)
                .await;
            debug!(
                total_added = chunk.len() + count * 20,
                "Added {} tracks to playlist.",
                chunk.len()
            );
            count += 1;
        }
        self.wipe_reference_playlist().await;
    }
    async fn wipe_reference_playlist(&self) {
        let _wipe_pl_span = debug_span!("wipe-ref-pl").entered();
        let track_ids = self.ref_pl_xplorer.playable_ids();

        for batch in track_ids.chunks(100) {
            match self
                .client
                .playlist_remove_all_occurrences_of_items(
                    self.ref_pl_xplorer.playlist_id.clone(),
                    batch.to_vec(),
                    None,
                )
                .await
            {
                Ok(_) => {
                    info!("Removed tracks from reference playlist.");
                }
                Err(err) => {
                    error!("Error: {:?}", err);
                    panic!("Could not remove tracks from reference playlist");
                }
            }
        }
    }
    fn check_if_stock_release_radar_id_was_used(&self, number_of_ids: usize) {
        if self.target_pl_xplorer.playlist_id.clone() == PlaylistType::StockRR.get_id() {
            error!(
                "Your Stock Release Radar ID was used: {playlist_id}",
                playlist_id = self.target_pl_xplorer.playlist_id.id()
            );
            panic!(
                "You must ensure that you are calling the update method with your full version release radar ID instead of your stock version's."
            )
        } else {
            info!("Your Full Release Radar playlists will be updated with {number_of_ids} songs",);
        }
    }
    fn generate_release_radar_description(&self) -> String {
        let local_time = chrono::Local::now();
        let local_time_string = local_time.format("%m/%d/%Y").to_string();
        format!(
            "Release Radar playlists with songs from albums included. Created on 11/02/2023. Updated on {local_time_string}."
        )
    }
    async fn update_playlist_from_chunk(&self, chunk: Vec<PlayableId<'_>>, is_first: bool) -> bool {
        let target_id = self.target_pl_xplorer.playlist_id.clone();
        let _upd_pl_from_chunk_span = debug_span!("upd-chunking").entered();

        if is_first {
            let description = self.generate_release_radar_description();
            self.client
                .playlist_change_detail(
                    target_id.clone(),
                    None,
                    None,
                    Some(description.as_str()),
                    None,
                )
                .await
                .expect("Couldn't update description");
            debug!("Replacing playlist items");
            self.client
                .playlist_replace_items(target_id.clone(), chunk.to_vec())
                .await
                .expect("Track IDs should be assigned to chunk_iterated as type TrackID");
        } else {
            debug!("Adding {} tracks to playlist.", chunk.len());
            self.client
                .playlist_add_items(target_id.clone(), chunk.to_vec(), None)
                .await
                .expect("Track IDs should be assigned to chunk_iterated as type TrackID");
        }
        false
    }
}
