use tracing::Level;

use spotify_assistant_core::actions::user::UserData;
use spotify_assistant_core::utilities::logging::init_tracing;

#[tokio::main]
async fn main() {
    init_tracing();
    let span = tracing::span!(Level::INFO, "main");
    let _enter = span.enter();

    let usr_data = UserData::new().await;
    let top_tracks = usr_data.top_tracks("short").await;
    println!("{:?}", top_tracks);
}
