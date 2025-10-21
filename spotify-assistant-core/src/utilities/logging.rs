use crate::utilities::configurations::CustomFormatter;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
pub fn init_tracing() {
    #[cfg(debug_assertions)]
    {
        let subscriber = FmtSubscriber::builder()
            .event_format(CustomFormatter)
            .with_max_level(Level::TRACE)
            .finish();
        tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    }

    #[cfg(not(debug_assertions))]
    {
        let subscriber = FmtSubscriber::builder()
            .event_format(CustomFormatter)
            .with_max_level(Level::INFO)
            .finish();
        tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    }
    tracing::trace!("Subscriber built");
}
