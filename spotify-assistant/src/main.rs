use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use spotify_assistant_cli::interface::TerminalApp;
// use spotify_assistant_core::actions::update::PlaylistCreator;
// use spotify_assistant_core::traits::apis::Querying;
use spotify_assistant_core::utilities::configurations::CustomFormatter;

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    {
        let subscriber = FmtSubscriber::builder()
            .event_format(CustomFormatter)
            .with_max_level(Level::TRACE)
            .finish();
        // Initialize the global tracing subscriber
        tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
        let span = tracing::span!(Level::INFO, "main");
        let _enter = span.enter();
    }
    let app = TerminalApp::new();
    app.run().await;
}
