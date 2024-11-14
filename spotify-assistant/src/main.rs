use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use spotify_assistant_core::utilities::configurations::CustomFormatter;

fn main() {
    #[cfg(debug_assertions)]
    {
        let subscriber = FmtSubscriber::builder()
            .event_format(CustomFormatter)
            .with_max_level(Level::INFO)
            .finish();
        // Initialize the global tracing subscriber
        tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
        let span = tracing::span!(Level::INFO, "main");
        let _enter = span.enter();
    }
    println!("Thanks for checking out the Spotify Assistant!");
}
