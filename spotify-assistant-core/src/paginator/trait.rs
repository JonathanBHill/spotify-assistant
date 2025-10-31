use futures::future::LocalBoxFuture;
use futures::StreamExt;
use rspotify::clients::pagination::Paginator;
use rspotify::model::FullTrack;
use rspotify::ClientResult;
use tracing::Level;

/// A trait that abstracts processing a paginator into a concrete collection.
///
/// This default implementation collects all items and returns `ClientResult<Vec<T>>`,
/// propagating the first error it encounters (fail-fast). You can override the behavior
/// in your own implementations if you want to keep going on errors instead.
pub trait PaginatorProcessor<T> {
    /// Process all pages and return the collected items or an error.
    fn process_all<'a>(&self, paginator: Paginator<'a, ClientResult<T>>) -> LocalBoxFuture<'a, ClientResult<Vec<T>>>
    where
        T: 'a;
}

impl<T> PaginatorProcessor<T> for FullTrack {
    fn process_all<'a>(&self, paginator: Paginator<'a, ClientResult<T>>) -> LocalBoxFuture<'a, ClientResult<Vec<T>>>
    where
        T: 'a
    {
        let span = tracing::span!(Level::INFO, "Paginator");
        let _enter = span.enter();
        let fut = async move {
            let mut paginator = paginator;
            let mut items = Vec::new();
            while let Some(page) = paginator.next().await {
                match page {
                    Ok(saved_track) => items.push(saved_track),
                    Err(err) => return Err(err),
                }
            }
            Ok(items)
        };
        Box::pin(fut)
    }
}
impl<T> PaginatorProcessor<T> for () {
    fn process_all<'a>(&self, paginator: Paginator<'a, ClientResult<T>>) -> LocalBoxFuture<'a, ClientResult<Vec<T>>>
    where
        T: 'a
    {
        let fut = async move {
            let mut paginator = paginator;
            let mut items = Vec::new();
            while let Some(page) = paginator.next().await {
                match page {
                    Ok(item) => items.push(item),
                    Err(err) => return Err(err),
                }
            }
            Ok(items)
        };
        Box::pin(fut)
    }
}

/// A convenience adapter to map `ClientResult<T>` into an enum for UIs or logging.
pub fn map_event<T>(res: ClientResult<T>) -> crate::paginator::PaginatorEvent<T> {
    match res {
        Ok(v) => crate::paginator::PaginatorEvent::Item(v),
        Err(e) => crate::paginator::PaginatorEvent::Error(e),
    }
}
