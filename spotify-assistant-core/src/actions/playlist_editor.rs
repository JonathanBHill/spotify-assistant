use crate::actions::exploration::playlist::PlaylistXplr;
use crate::enums::pl::PlaylistType;
use crate::models::full_track_fingerprint::PlaylistFingerprints;
use crate::traits::apis::Api;
use rspotify::clients::OAuthClient;
use rspotify::model::{FullTrack, PlayableId, PlaylistId, TrackId};
use rspotify::{AuthCodeSpotify, scopes};
use std::collections::HashSet;
use tracing::{debug, debug_span, error, info, trace};

#[derive(Debug, Clone)]
pub struct Modifier {
    client: AuthCodeSpotify,
    src_pl_xplorer: PlaylistXplr,
    target_playlist_id: PlaylistId<'static>,
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
        src_playlist_id: PlaylistId<'static>,
        target_playlist_id: PlaylistId<'static>,
    ) -> Self {
        let client = Self::set_up_client(false, Some(Self::select_scopes())).await;
        let src_pl_xplorer = PlaylistXplr::new(src_playlist_id, true).await;
        let target_pl_xplorer = PlaylistXplr::new(target_playlist_id.clone(), true).await;

        Modifier {
            client,
            src_pl_xplorer,
            target_playlist_id,
            target_pl_xplorer,
        }
    }
    pub async fn new_rr() -> Self {
        let client = Self::set_up_client(false, Some(Self::select_scopes())).await;
        let stock_rr_id = PlaylistType::StockRR.get_id();
        let my_rr_id = PlaylistType::MyRR.get_id();
        let lagging_rr_id = PlaylistType::MyLaggingRR.get_id();
        let src_pl_xplorer = PlaylistXplr::new(stock_rr_id, true).await;
        let target_pl_xplorer = PlaylistXplr::new(my_rr_id, true).await;
        Modifier {
            client,
            src_pl_xplorer,
            target_playlist_id: lagging_rr_id,
            target_pl_xplorer,
        }
    }
    pub async fn run_rr(&mut self) {
        let _run_span = debug_span!("run").entered();
        self.check_if_stock_release_radar_id_was_used_as_target_id();

        // Update lagging RR playlist with current RR playlist's tracks.
        info!(
            "Backing up the current Release Radar playlist into the Lagging Release Radar playlist"
        );
        let ref_tracks = self.target_pl_xplorer.full_tracks_expanded().await;
        let ref_track_ids = ref_tracks
            .iter()
            .map(|track| track.id.clone().unwrap())
            .collect();
        self.update_playlist(&ref_track_ids).await;

        // Update current RR playlist with tracks from source playlist.
        info!("Updating Release Radar playlist");
        self.target_playlist_id = self.target_pl_xplorer.playlist_id.clone();
        let raw_src_tracks = self.src_pl_xplorer.full_tracks_expanded().await;
        let src_tracks = self.filter_ref(raw_src_tracks, ref_tracks).await;
        let track_ids_from_src = src_tracks
            .iter()
            .map(|track| track.id.clone().unwrap())
            .collect();
        self.update_playlist(&track_ids_from_src).await;

        self.wipe_reference_playlist().await;
    }
    pub async fn filter_ref(
        &self,
        src_tracks: Vec<FullTrack>,
        ref_tracks: Vec<FullTrack>,
    ) -> Vec<FullTrack> {
        let src_fps = PlaylistFingerprints::new(&src_tracks);
        let ref_fps = PlaylistFingerprints::new(&ref_tracks);
        let fps_mask = ref_fps.filter_duplicates(src_fps.clone());
        trace!(
            source_count = src_fps.distinct_fp.len(),
            ref_count = ref_fps.distinct_fp.len(),
            mask_count = fps_mask.len()
        );
        PlaylistFingerprints::filter_candidate_with_mask(&fps_mask, &src_tracks)
    }

    pub async fn update_playlist(&self, track_ids: &Vec<TrackId<'_>>) {
        let _update_pl_span = debug_span!("update-playlist").entered();
        let ids_vec = track_ids;
        let ids_len = ids_vec.len();
        let mut first_chunk = true;
        let mut count = 1;

        for chunk in ids_vec.chunks(20) {
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
    }

    async fn wipe_reference_playlist(&self) {
        let _wipe_pl_span = debug_span!("wipe-ref-pl").entered();
        let track_ids = self.src_pl_xplorer.playable_ids();

        for batch in track_ids.chunks(100) {
            match self
                .client
                .playlist_remove_all_occurrences_of_items(
                    self.src_pl_xplorer.playlist_id.clone(),
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
    fn check_if_stock_release_radar_id_was_used_as_target_id(&self) {
        assert_ne!(
            self.target_playlist_id.clone(),
            PlaylistType::StockRR.get_id(),
            "The target playlist ID cannot be set to the stock release radar playlist ID."
        );
    }
    fn generate_release_radar_description(&self) -> String {
        let local_time = chrono::Local::now();
        let local_time_string = local_time.format("%m/%d/%Y").to_string();
        format!(
            "Release Radar playlists with songs from albums included. Created on 11/02/2023. Updated on {local_time_string}."
        )
    }
    async fn update_playlist_from_chunk(&self, chunk: Vec<PlayableId<'_>>, is_first: bool) -> bool {
        let target_id = self.target_playlist_id.clone();
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
