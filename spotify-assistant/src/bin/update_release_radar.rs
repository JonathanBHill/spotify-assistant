use tracing::Level;

use spotify_assistant_core::actions::update::Editor;
use spotify_assistant_core::utilities::logging::init_tracing;

#[tokio::main]
async fn main() {
    init_tracing();
    let span = tracing::span!(Level::INFO, "main");
    let _enter = span.enter();
    let my_release_radar = Editor::release_radar().await;
    my_release_radar.update_playlist().await
}
