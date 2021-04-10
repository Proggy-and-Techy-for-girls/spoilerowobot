use std::sync::Arc;

use tbot::{
    contexts::{fields::Context, methods::ChatMethods, Command, Text},
    types::input_file::{Animation, Audio, Document, Photo, Sticker, Video, VideoNote, Voice},
};

use crate::{
    bot::command::help,
    state::{spoiler::Content, State},
    strings::{
        CREATE_CUSTOM_SPOILER, ERROR_UNKNOWN_USER, INLINE_QUERY_SEPARATOR, PREPARING_A_SPOILER,
    },
    util::is_spoiler_id,
};

/// Handle the /start command sent from a private chat.
///
/// If the start parameter is empty or equals `CREATE_CUSTOM_SPOILER`, the bot will instruct the user
/// to create a spoiler.
/// Otherwise, it will send the requested spoiler (by the supplied spoiler id) to the user.
pub(crate) async fn start_from_pm(context: Arc<Command<Text>>, state: Arc<State>) {
    let user_id = context.from.as_ref().unwrap().id;

    // Create a new spoiler
    if context.text.value.is_empty() || context.text.value.eq(CREATE_CUSTOM_SPOILER) {
        let _status = state.set_waiting_for_spoiler(user_id).await;

        if let Err(e) = context
            .send_message_in_reply(PREPARING_A_SPOILER)
            .call()
            .await
        {
            dbg!(e.to_string());
        }
    } else {
        // Send an already created spoiler
        send_spoiler(context.clone(), state.clone()).await;
    }
}

/// Send the requested spoiler to the user
async fn send_spoiler(context: Arc<Command<Text>>, state: Arc<State>) {
    let spoiler_id = if is_spoiler_id(&context.text.value) {
        context
            .text
            .value
            .clone()
            .split(INLINE_QUERY_SEPARATOR)
            .collect::<String>()
    } else {
        context.text.value.clone()
    };
    if let Some(spoiler) = state.get_spoiler(spoiler_id).await {
        let user_id = context.from.as_ref().unwrap().id;

        match spoiler.content {
            Content::Animation(animation) => {
                if let Err(e) = context
                    .bot()
                    .send_animation(
                        user_id.to_owned(),
                        Animation::with_id(animation.file_id.as_ref()),
                    )
                    .call()
                    .await
                {
                    dbg!(e);
                }
            }
            Content::Audio(audio) => {
                if let Err(e) = context
                    .bot()
                    .send_audio(user_id.to_owned(), Audio::with_id(audio.file_id.as_ref()))
                    .call()
                    .await
                {
                    dbg!(e);
                }
            }
            Content::Contact(contact) => {
                if let Err(e) = context
                    .bot()
                    .send_contact(
                        user_id.to_owned(),
                        &contact.phone_number,
                        &contact.first_name,
                    )
                    .call()
                    .await
                {
                    dbg!(e);
                }
            }
            Content::Dice(_) => {}
            Content::Document(document) => {
                if let Err(e) = context
                    .bot()
                    .send_document(
                        user_id.to_owned(),
                        Document::with_id(document.file_id.as_ref()),
                    )
                    .call()
                    .await
                {
                    dbg!(e);
                }
            }
            Content::Location(location) => {
                if let Err(e) = context
                    .bot()
                    .send_location(user_id.to_owned(), (location.latitude, location.longitude))
                    .call()
                    .await
                {
                    dbg!(e);
                }
            }
            Content::Photo(photos) => {
                for photo in photos {
                    if let Err(e) = context
                        .bot()
                        .send_photo(user_id.to_owned(), Photo::with_id(photo.file_id.as_ref()))
                        .call()
                        .await
                    {
                        dbg!(e);
                    }
                }
            }
            Content::Sticker(sticker) => {
                if let Err(e) = context
                    .bot()
                    .send_sticker(
                        user_id.to_owned(),
                        Sticker::with_id(sticker.file_id.as_ref()),
                    )
                    .call()
                    .await
                {
                    dbg!(e);
                }
            }
            Content::Text(text) => {
                if let Err(e) = context
                    .bot()
                    .send_message(user_id.to_owned(), &text.value)
                    .call()
                    .await
                {
                    dbg!(e);
                }
            }
            Content::String(text) => {
                if let Err(e) = context
                    .bot()
                    .send_message(user_id.to_owned(), &text)
                    .call()
                    .await
                {
                    dbg!(e);
                }
            }
            Content::Video(video) => {
                if let Err(e) = context
                    .bot()
                    .send_video(user_id.to_owned(), Video::with_id(video.file_id.as_ref()))
                    .call()
                    .await
                {
                    dbg!(e);
                }
            }
            Content::VideoNote(video_note) => {
                if let Err(e) = context
                    .bot()
                    .send_video_note(
                        user_id.to_owned(),
                        VideoNote::with_id(video_note.file_id.as_ref()),
                    )
                    .call()
                    .await
                {
                    dbg!(e);
                }
            }
            Content::Voice(voice) => {
                if let Err(e) = context
                    .bot()
                    .send_voice(user_id.to_owned(), Voice::with_id(voice.file_id.as_ref()))
                    .call()
                    .await
                {
                    dbg!(e);
                }
            }
        }
    }
}

/// Handle the /start command if sent from within a group.
///
/// This will just post a message about usage.
pub(crate) async fn start_from_group(context: Arc<Command<Text>>, state: Arc<State>) {
    // todo probably just some help message maybe
    help::help(context, state).await;
}
