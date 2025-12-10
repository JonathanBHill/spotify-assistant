use rspotify::ClientError;

/// A simple enum to illustrate how you might model events while walking a paginator.
///
/// In practice, rspotify's paginator yields `ClientResult<T>`; you can map each
/// result into one of these events to drive a state machine or UI.
#[derive(Debug)]
pub enum PaginatorEvent<T> {
    /// An item was successfully yielded.
    Item(T),
    /// An error occurred while fetching the next item.
    Error(ClientError),
    /// The stream is exhausted.
    Done,
}

impl<T> PaginatorEvent<T> {
    /// Convenience helpers to check the variant.
    pub fn is_item(&self) -> bool { matches!(self, Self::Item(_)) }
    pub fn is_error(&self) -> bool { matches!(self, Self::Error(_)) }
    pub fn is_done(&self) -> bool { matches!(self, Self::Done) }
}
