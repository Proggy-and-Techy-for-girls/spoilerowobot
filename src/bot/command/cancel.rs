//! Handles the `/cancel` command.
use std::sync::Arc;

use tbot::contexts::{methods::ChatMethods, Command, Text};

use crate::state::State;
use crate::strings::bot_replies::SPOILER_CREATION_CANCELLED;

/// Handle the `/cancel` command.
///
/// This will cancel the spoiler creation process.
pub(crate) async fn cancel(context: Arc<Command<Text>>, state: Arc<State>) {
    let user = &context.from.as_ref().unwrap().id;

    let message = match state.cancel_spoiler_creation(&user.id) {
        None => "You were not creating a spoiler.",
        Some(..) => SPOILER_CREATION_CANCELLED,
    };

    if let Err(e) = context.send_message_in_reply(message).call().await {
        dbg!(e.to_string());
    }
}
