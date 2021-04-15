//! Handles the `/help` command.
use std::sync::Arc;

use tbot::contexts::{fields::Context, methods::ChatMethods, Command, Text};

use crate::state::State;
use crate::strings::bot_replies::help_text;

/// Handle the `/help` command.
///
/// This will send a reply with a hint on how to use this bot.
pub(crate) async fn help(context: Arc<Command<Text>>, _state: Arc<State>) {
    let bot_username = match context.bot().get_me().call().await {
        Ok(me) => me.user.username.unwrap_or(me.user.first_name),
        Err(err) => {
            dbg!(err.to_string());
            "my bot username".to_string()
        }
    };
    if let Err(e) = context
        .send_message_in_reply(&help_text(bot_username))
        .call()
        .await
    {
        dbg!(e);
    }
}
