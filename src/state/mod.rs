use std::sync::Arc;
use tokio::sync::Mutex;

/// The bot's state. will be later expanded to support writing to databases.
#[derive(Default)]
pub(crate) struct State {
    /// A store of IDs used to interact with Telegram inline queries.
    pub inline_query_ids: Arc<Mutex<u128>>,
}
