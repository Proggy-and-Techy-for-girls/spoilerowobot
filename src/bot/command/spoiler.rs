//! Handles the `/spoiler` command.
use std::sync::Arc;

use tbot::{
    contexts::{methods::ChatMethods, Command, Text},
    types::{
        chat::member::Status,
        keyboard::inline::{Button, ButtonKind, Markup},
        message::Kind,
        parameters,
    },
};

use crate::{
    state::{spoiler::Content, State},
    strings::{
        bot_replies::{NOT_AN_ADMIN, NO_DELETE_PERMISSION},
        INLINE_QUERY_SEPARATOR, SHOW_SPOILER,
    },
};

/// Handles the `/spoiler` command.
///
/// When replying to someone else's message with `/spoiler`, it will create a new spoiler with the
/// message in reply to as content and finally delete the original message from the group.
///
/// This requires permission for the bot to delete messages in the group.
pub(crate) async fn spoiler(context: Arc<Command<Text>>, state: Arc<State>) {
    let bot_id = context.bot.get_me().call().await.unwrap().user.id;

    if !can_delete_messages(context.clone()).await {
        return;
    }

    if let Some(message) = context.reply_to.clone() {
        match message.kind {
            Kind::Text(text) => {
                state.new_spoiler(bot_id, Content::Text(text));
            }
            Kind::Audio(audio, caption) => {
                state.new_spoiler(bot_id, Content::Audio(audio, caption));
            }
            Kind::Document(document, caption) => {
                state.new_spoiler(bot_id, Content::Document(document, caption))
            }
            Kind::Dice(dice) => {
                state.new_spoiler(bot_id, Content::Dice(dice));
            }
            Kind::Photo(photo, caption, media_group_id) => {
                state.new_spoiler(bot_id, Content::Photo(photo, caption, media_group_id));
            }
            Kind::Sticker(sticker) => {
                state.new_spoiler(bot_id, Content::Sticker(sticker));
            }
            Kind::Video(video, caption, media_group_id) => {
                state.new_spoiler(bot_id, Content::Video(video, caption, media_group_id));
            }
            Kind::Voice(voice, caption) => {
                state.new_spoiler(bot_id, Content::Voice(voice, caption));
            }
            Kind::VideoNote(video_note) => {
                state.new_spoiler(bot_id, Content::VideoNote(video_note));
            }
            Kind::Contact(contact) => {
                state.new_spoiler(bot_id, Content::Contact(contact));
            }
            Kind::Location(location) => {
                state.new_spoiler(bot_id, Content::Location(location));
            }
            Kind::Animation(animation, caption) => {
                state.new_spoiler(bot_id, Content::Animation(animation, caption));
            }
            _ => {}
        }

        // and post the created spoiler in the group
        let mut spoiler_id = String::from(INLINE_QUERY_SEPARATOR);

        let title = format!(
            "Bad message from {}:",
            &context
                .reply_to
                .as_ref()
                .unwrap()
                .from
                .as_ref()
                .unwrap()
                .first_name
        );
        spoiler_id.push_str(&*state.set_spoiler_title_and_expiration(bot_id, title.clone(), None));
        let reply_markup: Markup = &[&[Button::new(
            SHOW_SPOILER,
            ButtonKind::CallbackData(&spoiler_id),
        )]];

        let spoiler = format!(
            "<b>Spoiler!</b>{}{}",
            format!("\n<code>{}</code>", &context.text.value),
            format!("\n\n{}", &title)
        );
        if let Err(e) = context
            .bot
            .send_message(context.chat.id, parameters::Text::with_html(&spoiler))
            .reply_markup(reply_markup)
            .call()
            .await
        {
            dbg!(e);
        }

        // and finally delete the message in reply to
        if let Err(e) = context
            .bot
            .delete_message(context.chat.id, message.id)
            .call()
            .await
        {
            dbg!(e);
        }
    }
}

/// Returns `true` if the bot is allowed to delete messages from other users
/// in the group where the request came from.
async fn can_delete_messages(context: Arc<Command<Text>>) -> bool {
    match context
        .bot
        .get_chat_administrators(context.chat.id)
        .call()
        .await
    {
        Ok(users) => {
            for member in users {
                if member
                    .user
                    .id
                    .eq(&context.bot.get_me().call().await.unwrap().user.id)
                {
                    match member.status {
                        Status::Administrator {
                            can_delete_messages,
                            ..
                        } => {
                            if !can_delete_messages {
                                if let Err(e) = context
                                    .send_message_in_reply(NO_DELETE_PERMISSION)
                                    .call()
                                    .await
                                {
                                    dbg!(e);
                                }
                            }
                            return can_delete_messages;
                        }
                        _ => {
                            if let Err(e) = context.send_message_in_reply(NOT_AN_ADMIN).call().await
                            {
                                dbg!(e);
                            }
                        }
                    }
                }
            }
        }
        Err(err) => panic!("{}", err),
    }
    false
}
