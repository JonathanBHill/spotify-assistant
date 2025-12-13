use spotify_assistant_core::actions::playlist_editor::Modifier;
use spotify_assistant_core::utilities::logging::init_tracing;
use tracing::Level;

#[tokio::main]
async fn main() {
    init_tracing();
    let span = tracing::span!(Level::INFO, "main");
    let _enter = span.enter();

    let mut modifier = Modifier::new_rr().await;
    modifier.run_rr().await;
}
