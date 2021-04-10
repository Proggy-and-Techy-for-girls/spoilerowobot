//! Periodically polls for expired entries.
//!
//! Expired spoilers are going to be removed from the state.
use std::sync::Arc;

use futures_util::stream::poll_fn;
use tokio::{
    stream::StreamExt,
    time::{delay_for, Duration},
};

use crate::state::State;

/// Periodically polls for expired entries from the [DelayQueue].
///
/// This function periodically checks for entries in the [DelayQueue] that are going to expire next
/// and removes these instances from the [expirations].
///
/// [DelayQueue]: tokio::time::delay_queue::DelayQueue
/// [expirations]: crate::state::State#structfield.expirations
pub(crate) async fn poll_for_expired_entries(state: Arc<State>) {
    // There might be a better way to poll new expirations, but this should be fine for now...
    #[allow(irrefutable_let_patterns)]
    while let item = poll_fn(|cx| state.expirations.lock().unwrap().poll_expired(cx))
        .next()
        .await
    {
        if let Some(Ok(result)) = item {
            let cache_key = result.into_inner();
            let _entry;
            {
                _entry = state.spoilers.lock().unwrap().remove(&cache_key)
            }
        } else {
            delay_for(Duration::from_secs(1)).await;
        }
    }
}
