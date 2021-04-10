use std::sync::Arc;

use rand::{distributions::Alphanumeric, Rng};
use tbot::contexts::fields::Context;

use crate::strings::INLINE_QUERY_SEPARATOR;

/// Create a random string that acts as an identifier.
pub(crate) fn random_id() -> String {
    rand::thread_rng()
        .sample_iter(Alphanumeric)
        .take(48)
        .map(char::from)
        .collect::<String>()
}

/// Return true if the query starts with `INLINE_QUERY_SEPARATOR`.
pub(crate) fn is_spoiler_id(query: &String) -> bool {
    query.starts_with(INLINE_QUERY_SEPARATOR)
}

/// Generate a start URL pointing to the bot with a start parameter.
pub(crate) async fn start_url(context: Arc<impl Context>, start_param: &String) -> String {
    format!(
        "https://t.me/{}?start={}",
        context
            .bot()
            .get_me()
            .call()
            .await
            .unwrap()
            .user
            .username
            .unwrap(),
        start_param
    )
}
