extern crate regex;

use std::sync::Arc;
use std::time::Duration;

use rand::{distributions::Alphanumeric, Rng};
use tbot::contexts::fields::Context;

use crate::strings::INLINE_QUERY_SEPARATOR;

use self::regex::Regex;

static MINUTE_IN_SECS: u64 = 60;
static HOUR_IN_SECS: u64 = 60 * MINUTE_IN_SECS;
static DAY_IN_SECS: u64 = 24 * HOUR_IN_SECS;
static WEEK_IN_SECS: u64 = 7 * DAY_IN_SECS;
static MONTH_IN_SECS: u64 = 30 * DAY_IN_SECS;
static YEAR_IN_SECS: u64 = 365 * DAY_IN_SECS;


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

pub(crate) fn parse_duration(text: &String) -> Option<Duration> {
    // parse the expiration
    lazy_static! {
        static ref RE: Regex = Regex::new(r"/(\d+)(s|m|h|d|M|Y)$").unwrap();
    }

    if RE.is_match(text) {
        let captures = RE.captures(text).unwrap();
        let amount = captures.get(1).map_or("", |m| m.as_str());
        let amount = amount.parse::<u64>().unwrap();

        let unit = captures.get(2).map_or("", |m| m.as_str());

        match unit {
            "m" => Some(Duration::from_secs(amount * MINUTE_IN_SECS)),
            "M" => Some(Duration::from_secs(amount * MONTH_IN_SECS)),
            "s" | "S" => Some(Duration::from_secs(amount)),
            "h" | "H" => Some(Duration::from_secs(amount * HOUR_IN_SECS)),
            "d" | "D" => Some(Duration::from_secs(amount * DAY_IN_SECS)),
            "w" | "W" => Some(Duration::from_secs(amount * WEEK_IN_SECS)),
            "y" | "Y" => Some(Duration::from_secs(amount * YEAR_IN_SECS)),
            _ => None,
        }
    } else {
        None
    }
}

pub(crate) fn strip_expiration_suffix(text: &String) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"/\d+[[:alnum:]]$").unwrap();
    }
    RE.replace(text, "").to_string()
}