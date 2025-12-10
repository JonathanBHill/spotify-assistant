use rspotify::clients::pagination::Paginator;
use rspotify::ClientResult;

use crate::paginator::r#trait::PaginatorProcessor;

/// A thin, generic wrapper struct that owns a paginator and a processor strategy.
///
/// This illustrates the “struct approach” to handling paginated streams: you can inject
/// different processor implementations (that implement `PaginatorProcessor<T>`) to customize
/// how errors are handled, whether to limit items, etc.
pub struct PaginatorRunner<'a, T, P: PaginatorProcessor<T>> {
    paginator: Paginator<'a, ClientResult<T>>,
    processor: P,
}

impl<'a, T, P: PaginatorProcessor<T>> PaginatorRunner<'a, T, P> {
    /// Create a new runner from a paginator and a processing strategy.
    pub fn new(paginator: Paginator<'a, ClientResult<T>>, processor: P) -> Self {
        Self { paginator, processor }
    }

    /// Consume the runner and collect all items according to the processor's policy.
    pub async fn run(self) -> ClientResult<Vec<T>> {
        let paginator = self.paginator;
        let processor = self.processor;
        processor.process_all(paginator).await
    }
}

/// A convenience constructor that uses the unit type `()` as the processor, which has a
/// default fail-fast implementation.
pub fn run_with_default<'a, T: 'a>(paginator: Paginator<'a, ClientResult<T>>) -> impl std::future::Future<Output = ClientResult<Vec<T>>> + 'a {
    async move {
        let runner = PaginatorRunner::new(paginator, ());
        runner.run().await
    }
}
