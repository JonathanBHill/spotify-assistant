use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use spotify_assistant::core::functionality::update::ReleaseRadar;
use spotify_assistant::core::utilities::configurations::CustomFormatter;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .event_format(CustomFormatter)
        .with_max_level(Level::TRACE)
        .finish();
    // Initialize the global tracing subscriber
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    {
        let span = tracing::span!(Level::INFO, "main");
        let _enter = span.enter();
        let my_release_radar = ReleaseRadar::new_personal().await;
        my_release_radar.update_rr(true).await
    }
}
