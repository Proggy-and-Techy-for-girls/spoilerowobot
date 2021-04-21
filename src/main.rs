//! Spoilerowobot, a Telegram Bot for creating spoilers.
#![warn(missing_docs)]
#![warn(broken_intra_doc_links)]

#[macro_use]
extern crate lazy_static;

use tbot::predicates::{chat::is_private, without_state};

use crate::{
    bot::{
        callback,
        command::{cancel, help, spoiler, start},
        inline, spoiler_creation,
    },
    state::{periodic, State},
};

mod bot;
mod state;
mod strings;
mod util;

#[tokio::main]
async fn main() {
    let bot = tbot::from_env!("SPOILEROWO_BOT_TOKEN");
    let mut event_loop = bot.clone().stateful_event_loop(State::default());

    if let Err(msg) = event_loop.fetch_username().await {
        dbg!(msg);
    }

    // Listen to the following commands
    event_loop.start(start::start_from_pm);
    event_loop.command("spoiler", spoiler::spoiler);
    event_loop.command("cancel", cancel::cancel);
    event_loop.help(help::help);

    // Listen to inline queries
    event_loop.inline(inline::inline);

    // Listen to data callbacks
    event_loop.data_callback(callback::data_callback);

    // Listen to the following events for spoiler creation
    event_loop.animation_if(without_state(is_private), spoiler_creation::animation);
    event_loop.audio_if(without_state(is_private), spoiler_creation::audio);
    event_loop.contact_if(without_state(is_private), spoiler_creation::contact);
    event_loop.dice_if(without_state(is_private), spoiler_creation::dice);
    event_loop.document_if(without_state(is_private), spoiler_creation::document);
    event_loop.location_if(without_state(is_private), spoiler_creation::location);
    event_loop.photo_if(without_state(is_private), spoiler_creation::photo);
    event_loop.sticker_if(without_state(is_private), spoiler_creation::sticker);
    event_loop.text_if(without_state(is_private), spoiler_creation::text);
    event_loop.video_if(without_state(is_private), spoiler_creation::video);
    event_loop.video_note_if(without_state(is_private), spoiler_creation::video_note);
    event_loop.voice_if(without_state(is_private), spoiler_creation::voice);

    // A loop to check for expired spoilers that need to be cleared
    tokio::spawn(periodic::poll_for_expired_entries(event_loop.get_state()));

    // todo webhooks?
    event_loop.polling().start().await.unwrap();
}
