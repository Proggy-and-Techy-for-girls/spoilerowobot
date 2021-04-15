//! Handles the `/spoiler` command.
use std::sync::Arc;

use tbot::contexts::{methods::ChatMethods, Command, Text};

use crate::state::State;
use crate::strings::bot_replies::TYPE_START;

/// Hint the user how to start the spoiler creation process.
///
/// In the future, this command will be used to hide messages from users within a group behind a
/// spoiler. This will require admin permissions for the bot to work.
pub(crate) async fn spoiler(context: Arc<Command<Text>>, _state: Arc<State>) {
    // todo implement feature to hide sensitive messages from others into a spoiler.
    // For now, just leave a hint how to start the spoiler creation process.
    if let Err(e) = context.send_message_in_reply(TYPE_START).call().await {
        dbg!(e);
    }
}
