use spotify_assistant_cli::interface::TerminalApp;
use spotify_assistant_core::utilities::logging::init_tracing;

#[tokio::main]
async fn main() {
    init_tracing();
    let span = tracing::span!(tracing::Level::INFO, "main");
    let _enter = span.enter();
    
    let app = TerminalApp::new();
    app.run().await;
}
