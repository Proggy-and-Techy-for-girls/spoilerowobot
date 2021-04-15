//! Methods related to spoiler creation
use std::sync::Arc;

use tbot::{
    contexts::{
        fields::Message, methods::ChatMethods, Animation, Audio, Contact, Dice, Document, Location,
        Photo, Sticker, Text, Video, VideoNote, Voice,
    },
    types::keyboard::inline::{Button, ButtonKind, Markup},
};
use tokio::time::Duration;

use crate::strings::bot_replies::{NOW_SEND_A_TITLE, SPOILER_READY};
use crate::strings::{INLINE_QUERY_SEPARATOR, SEND_IT};
use crate::{
    state::{spoiler::Content, State},
    util,
};

/// Handle text messages.
pub(crate) async fn text(context: Arc<Text>, state: Arc<State>) {
    let user_id = context.from.as_ref().unwrap().id;

    if state.waiting_for_spoiler(&user_id) {
        // check if we are in the spoiler creation process
        new_spoiler(context.clone(), state.clone()).await;
    } else if state.waiting_for_title(&user_id) {
        set_spoiler_title(context.clone(), state.clone()).await;
    }
}

/// Create a new state and notify the user what to do next
async fn new_spoiler(context: Arc<Text>, state: Arc<State>) {
    let user_id = context.from().as_ref().unwrap().id;

    state.new_spoiler(user_id.to_owned(), Content::Text(context.text.to_owned()));
    let _ = state.set_waiting_for_title(user_id);

    if let Err(e) = context.send_message_in_reply(NOW_SEND_A_TITLE).call().await {
        dbg!(e);
    }
}

/// Set the spoiler title and return the created spoiler to the user
async fn set_spoiler_title(context: Arc<Text>, state: Arc<State>) {
    let user_id = context.from.as_ref().unwrap().id;

    let expires_in: Option<Duration> = util::parse_duration(&context.text.value);

    let mut spoiler_id = String::from(INLINE_QUERY_SEPARATOR);
    spoiler_id.push_str(&*state.set_spoiler_title_and_expiration(
        user_id,
        context.text.value.to_owned(),
        expires_in,
    ));
    let reply_markup: Markup = &[&[Button::new(
        SEND_IT,
        ButtonKind::SwitchInlineQuery(&spoiler_id),
    )]];

    if let Err(e) = context
        .send_message_in_reply(SPOILER_READY)
        .reply_markup(reply_markup)
        .call()
        .await
    {
        dbg!(e);
    }
}

/// Handle Animation messages.
pub(crate) async fn animation(context: Arc<Animation>, state: Arc<State>) {
    let user_id = context.from.as_ref().unwrap().id;

    if !state.waiting_for_spoiler(&user_id) {
        return;
    }

    let _ = state.set_waiting_for_title(user_id);
    state.new_spoiler(
        user_id.to_owned(),
        Content::Animation(context.animation.to_owned()),
    );

    if let Err(e) = context.send_message_in_reply(NOW_SEND_A_TITLE).call().await {
        dbg!(e.to_string());
    }
}

/// Handle Audio Messages
pub(crate) async fn audio(context: Arc<Audio>, state: Arc<State>) {
    let user_id = context.from.as_ref().unwrap().id;

    if !state.waiting_for_spoiler(&user_id) {
        return;
    }

    let _ = state.set_waiting_for_title(user_id);
    state.new_spoiler(user_id.to_owned(), Content::Audio(context.audio.to_owned()));

    if let Err(e) = context.send_message_in_reply(NOW_SEND_A_TITLE).call().await {
        dbg!(e.to_string());
    }
}

/// Handle Contact Messages
pub(crate) async fn contact(context: Arc<Contact>, state: Arc<State>) {
    let user_id = context.from.as_ref().unwrap().id;

    if !state.waiting_for_spoiler(&user_id) {
        return;
    }

    let _ = state.set_waiting_for_title(user_id);
    state.new_spoiler(
        user_id.to_owned(),
        Content::Contact(context.contact.to_owned()),
    );

    if let Err(e) = context.send_message_in_reply(NOW_SEND_A_TITLE).call().await {
        dbg!(e.to_string());
    }
}

/// Handle Dice Messages
pub(crate) async fn dice(context: Arc<Dice>, state: Arc<State>) {
    let user_id = context.from.as_ref().unwrap().id;

    if !state.waiting_for_spoiler(&user_id) {
        return;
    }

    let _ = state.set_waiting_for_title(user_id.to_owned());
    state.new_spoiler(user_id, Content::Dice(context.dice.to_owned()));

    if let Err(e) = context.send_message_in_reply(NOW_SEND_A_TITLE).call().await {
        dbg!(e.to_string());
    }
}

/// Handle Document Messages
pub(crate) async fn document(context: Arc<Document>, state: Arc<State>) {
    let user_id = context.from.as_ref().unwrap().id;

    if !state.waiting_for_spoiler(&user_id) {
        return;
    }

    let _ = state.set_waiting_for_title(user_id.to_owned());
    state.new_spoiler(user_id, Content::Document(context.document.to_owned()));

    if let Err(e) = context.send_message_in_reply(NOW_SEND_A_TITLE).call().await {
        dbg!(e.to_string());
    }
}

/// Handle Location Messages
pub(crate) async fn location(context: Arc<Location>, state: Arc<State>) {
    let user_id = context.from.as_ref().unwrap().id;

    if !state.waiting_for_spoiler(&user_id) {
        return;
    }
    let _ = state.set_waiting_for_title(user_id.to_owned());
    state.new_spoiler(user_id, Content::Location(context.location.to_owned()));

    if let Err(e) = context.send_message_in_reply(NOW_SEND_A_TITLE).call().await {
        dbg!(e.to_string());
    }
}

/// Handle Photo Messages
pub(crate) async fn photo(context: Arc<Photo>, state: Arc<State>) {
    let user_id = context.from.as_ref().unwrap().id;

    if !state.waiting_for_spoiler(&user_id) {
        return;
    }

    let _ = state.set_waiting_for_title(user_id.to_owned());
    state.new_spoiler(user_id, Content::Photo(context.photo.to_owned()));

    if let Err(e) = context.send_message_in_reply(NOW_SEND_A_TITLE).call().await {
        dbg!(e.to_string());
    }
}

/// Handle Sticker Messages
pub(crate) async fn sticker(context: Arc<Sticker>, state: Arc<State>) {
    let user_id = context.from.as_ref().unwrap().id;

    if !state.waiting_for_spoiler(&user_id) {
        return;
    }

    let _ = state.set_waiting_for_title(user_id.to_owned());
    state.new_spoiler(user_id, Content::Sticker(context.sticker.to_owned()));

    if let Err(e) = context.send_message_in_reply(NOW_SEND_A_TITLE).call().await {
        dbg!(e.to_string());
    }
}

/// Handle Video Messages
pub(crate) async fn video(context: Arc<Video>, state: Arc<State>) {
    let user_id = context.from.as_ref().unwrap().id;

    if !state.waiting_for_spoiler(&user_id) {
        return;
    }

    let _ = state.set_waiting_for_title(user_id);
    state.new_spoiler(user_id, Content::Video(context.video.to_owned()));

    if let Err(e) = context.send_message_in_reply(NOW_SEND_A_TITLE).call().await {
        dbg!(e.to_string());
    }
}

/// Handle Video Note Messages
pub(crate) async fn video_note(context: Arc<VideoNote>, state: Arc<State>) {
    let user_id = context.from.as_ref().unwrap().id;

    if !state.waiting_for_spoiler(&user_id) {
        return;
    }

    let _ = state.set_waiting_for_title(user_id.to_owned());
    state.new_spoiler(user_id, Content::VideoNote(context.video_note.to_owned()));

    if let Err(e) = context.send_message_in_reply(NOW_SEND_A_TITLE).call().await {
        dbg!(e.to_string());
    }
}

/// Handle Voice Messages
pub(crate) async fn voice(context: Arc<Voice>, state: Arc<State>) {
    let user_id = context.from.as_ref().unwrap().id;
    if !state.waiting_for_spoiler(&user_id) {
        return;
    }

    let _ = state.set_waiting_for_title(user_id.to_owned());
    state.new_spoiler(user_id, Content::Voice(context.voice.to_owned()));

    if let Err(e) = context.send_message_in_reply(NOW_SEND_A_TITLE).call().await {
        dbg!(e.to_string());
    }
}
