//! Various utility functions needed throughout the project
extern crate regex;

use std::{sync::Arc, time::Duration};

use chrono::Utc;
use rand::{distributions::Alphanumeric, Rng};
use tbot::contexts::fields::Context;

use crate::strings::INLINE_QUERY_SEPARATOR;

use self::regex::Regex;

/// One minute in seconds.
pub(crate) static MINUTE_IN_SECS: u64 = 60;

/// One hour in seconds.
pub(crate) static HOUR_IN_SECS: u64 = 60 * MINUTE_IN_SECS;

/// One day in seconds.
pub(crate) static DAY_IN_SECS: u64 = 24 * HOUR_IN_SECS;

/// One week in seconds.
pub(crate) static WEEK_IN_SECS: u64 = 7 * DAY_IN_SECS;

/// One month in seconds.
///
/// A month is assumed to equal 30 days.
pub(crate) static MONTH_IN_SECS: u64 = 30 * DAY_IN_SECS;

/// One year in seconds.
///
/// A year is assumed to equal 365 days.
pub(crate) static YEAR_IN_SECS: u64 = 365 * DAY_IN_SECS;

/// Generates a random string that acts as an identifier, i.e. a spoiler id.
pub(crate) fn random_id() -> String {
    rand::thread_rng()
        .sample_iter(Alphanumeric)
        .take(48)
        .map(char::from)
        .collect::<String>()
}

/// Returns true if the query starts with [`INLINE_QUERY_SEPARATOR`].
pub(crate) fn is_spoiler_id(query: &String) -> bool {
    query.starts_with(INLINE_QUERY_SEPARATOR)
}

/// Generates a [Telegram start URL][tg docs] pointing to the bot with the provided start parameter.
///
/// [tg docs]: https://core.telegram.org/bots#deep-linking
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

/// Returns a Duration according to the user
///
/// The user specifies a duration by appending `/` followed by a number followed by one of the
/// following characters to generate a duration:
///
/// | Symbol   | Results in |
/// |----------|------------|
/// | `s`, `S` | Second     |
/// | `m`      | Minute     |
/// | `h`, `H` | Hour       |
/// | `d`, `D` | Day        |
/// | `M`      | Month. A Month is assumed to be 30 days long. |
/// | `y`, `Y` | Year. A year is assumed to be 365 days long.  |
///
/// # Examples
/// - `/7m` would result into a Duration of 7 minutes,
/// - `/6M` would result into a Duration of 6 months,
/// - `/1y` would result into a Duration of 1 year.
pub(crate) fn parse_duration(text: &String) -> Option<Duration> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"/(\d+)(s|S|m|M|h|H|d|D|y|Y)$").unwrap();
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

/// Removes the expiration suffix if present.
pub(crate) fn strip_expiration_suffix(text: &String) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"/\d+[[:alnum:]]$").unwrap();
    }
    RE.replace(text, "").to_string()
}

/// Returns a String representation of the future point in time (date + time in UTC)
/// when adding "now" + the specified duration.
pub(crate) fn expires_at(duration: Duration) -> String {
    Utc::now()
        .checked_add_signed(chrono::Duration::from_std(duration).unwrap())
        .unwrap()
        .format("%F %R %Z")
        .to_string()
}
