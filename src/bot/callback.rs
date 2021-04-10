//! This module implements the user interaction via [callbacks].
//!
//! [callbacks]: https://core.telegram.org/bots/2-0-intro#callback-buttons
use std::sync::Arc;

use tbot::contexts::{methods::Callback, DataCallback};

use crate::strings::bot_replies::{SPOILER_NOT_FOUND, TAP_AGAIN_TO_SHOW_SPOILER};
use crate::strings::{INLINE_QUERY_SEPARATOR, MAJOR_SPOILER_IDENTIFIER};
use crate::{state::spoiler::Content, util::start_url, State};

/// The maximum length of an Telegram alert.
///
/// A telegram alert [can only be up to 200 characters long][tg docs]. If the content longer than
/// that, it needs to be sent in a private message instead.
///
/// [tg docs]: https://core.telegram.org/bots/api#answercallbackquery
static MAX_ALERT_LENGTH: usize = 200;

/// Data callback handler
///
/// Receives a spoiler_id as argument.
/// If the id starts with `maj_`, instruct the handler to open the spoiler in a major fashion.
pub(crate) async fn data_callback(context: Arc<DataCallback>, state: Arc<State>) {
    if !context.data.contains(INLINE_QUERY_SEPARATOR) {
        // useless data callback. We only process queries that have an id
        return;
    }

    let spoiler_id = context
        .data
        .clone()
        .rsplit(MAJOR_SPOILER_IDENTIFIER)
        .collect::<Vec<&str>>()[0]
        .to_string()
        .split(INLINE_QUERY_SEPARATOR)
        .collect::<String>();

    if context.data.starts_with(MAJOR_SPOILER_IDENTIFIER)
        && state.needs_to_tap_once_more(context.from.id.clone(), spoiler_id.to_string())
    {
        if let Err(e) = context.notify(TAP_AGAIN_TO_SHOW_SPOILER).call().await {
            dbg!(e);
            return;
        }
    }

    match state.get_spoiler(spoiler_id.clone()) {
        Some(spoiler) => {
            match &spoiler.content {
                Content::Text(text) => {
                    if text.value.chars().count() <= MAX_ALERT_LENGTH {
                        // 200 is the max limit for an alert
                        if let Err(e) = context.alert(&text.value).call().await {
                            dbg!(e);
                        }
                        return;
                    }
                }
                Content::String(text) => {
                    if text.chars().count() <= MAX_ALERT_LENGTH {
                        if let Err(e) = context.alert(&text).call().await {
                            dbg!(e);
                        }
                        return;
                    }
                }
                _ => {}
            }

            if let Err(e) = context
                .open_url(
                    &start_url(
                        context.clone(),
                        &format!("{}{}", INLINE_QUERY_SEPARATOR, spoiler_id),
                    )
                    .await,
                )
                .call()
                .await
            {
                dbg!(e);
            }
        }
        None => {
            if let Err(e) = context.notify(SPOILER_NOT_FOUND).call().await {
                dbg!(e);
                return;
            }
        }
    }
}
