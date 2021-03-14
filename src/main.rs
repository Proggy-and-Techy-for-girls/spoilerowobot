use tbot::prelude::*;

use crate::{
    bot::{callback, command},
    state::State,
};

mod bot;
mod state;

#[tokio::main]
async fn main() {
    let bot = tbot::from_env!("SPOILEROWO_BOT_TOKEN");
    let mut event_loop = bot.clone().stateful_event_loop(State::default());

    if let Err(msg) = event_loop.fetch_username().await {
        dbg!(msg);
    }

    // Register bot commands
    event_loop.start(command::start);
    event_loop.help(command::help);
    event_loop.command("spoiler", command::spoiler);
    event_loop.inline(command::inline);
    event_loop.data_callback(callback::data_callback);

    // todo
    event_loop.polling().start().await.unwrap();
}
