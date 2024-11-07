use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use spotify_assistant_core::actions::user::UserData;
use spotify_assistant_core::utilities::configurations::CustomFormatter;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .event_format(CustomFormatter)
        .with_max_level(Level::TRACE)
        .finish();
    // Initialize the global tracing subscriber
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    {
        let span = tracing::span!(Level::INFO, "main");
        let _enter = span.enter();
        let usr_data = UserData::new().await;
        let top_tracks = usr_data.top_tracks().await;
        println!("{:?}", top_tracks);
    }
}
