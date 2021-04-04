use rand::{distributions::Alphanumeric, Rng};

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
