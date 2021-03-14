use std::sync::Arc;

use rand::{distributions::Alphanumeric, Rng};
use tbot::{
    contexts::{
        Command,
        fields::Context,
        Inline,
        methods::ChatMethods,
        Text
    },
    types::{
        inline_query,
        inline_query::result::Thumb,
        input_message_content,
        keyboard::inline::{
            Button,
            ButtonKind,
            Keyboard,
            Markup,
        },
        parameters::Text as ParseMode,
    },
};

use crate::state::State;

/// Handle inline queries
///
/// Upon typing the text to be spoiled, the bot shows three options to create a spoiler:
///
/// The first option is to create a *minor Spoiler*, where the message is hidden behind a dialog
/// thingie that requires only a single tap to be opened.
///
/// The second option is to create a *major Spoiler*, where the message is hidden behind a dialog
/// thingie that requires two taps to be opened.
///
/// The third option is to create an advanced spoiler, where the user can upload Texts, images,
/// videos etc. that should be hidden behind a spoiler message.
pub(crate) async fn inline(context: Arc<Inline>, state: Arc<State>) {
    // do I actually need this?
    let _id = {
        let mut id = state.inline_query_ids.lock().await;
        *id += 1;
        id.to_string()
    };

    // minor spoiler
    let generated_minor_spoiler =
        input_message_content::Text::new(ParseMode::with_markdown_v2("_Minor spoiler_"));

    let minor_spoiler =
        inline_query::result::Article::new("Minor Spoiler", generated_minor_spoiler)
            .description("Text, single tap")
            .thumb(
                Thumb::new("https://i.imgur.com/csh5H5O.png")
                    .width(512)
                    .height(512),
            );
    let rid = random_id();
    let minor_spoiler_keyboard_markup: Markup = &[&[Button::new(
        "Show spoiler",
        ButtonKind::CallbackData(&context.query),
    )]];

    let minor_spoiler_result = inline_query::Result::new(&rid, minor_spoiler)
        .reply_markup(Keyboard::new(minor_spoiler_keyboard_markup));

    // major spoiler
    let generated_major_spoiler =
        input_message_content::Text::new(ParseMode::with_markdown_v2("*Major spoiler\\!*"));

    let major_spoiler =
        inline_query::result::Article::new("Major Spoiler", generated_major_spoiler)
            .description("Text, double tap")
            .thumb(
                Thumb::new("https://i.imgur.com/3qqCZZk.png")
                    .width(512)
                    .height(512),
            );
    let rid: String = random_id();

    let major_spoiler_keyboard_markup: Markup = &[&[Button::new(
        "Double tap to show spoiler",
        ButtonKind::CallbackData(&context.query),
    )]];
    let major_spoiler_result = inline_query::result::Result::new(&rid, major_spoiler)
        .reply_markup(Keyboard::new(major_spoiler_keyboard_markup));

    let rid = random_id();
    if let Err(e) = context
        .answer(&[minor_spoiler_result, major_spoiler_result])
        .is_personal(true)
        .switch_pm("Advanced spoiler (media etc.)…", &rid)
        .call()
        .await
    {
        dbg!(e.to_string());
    }
}

/// Command to display information on usage
pub(crate) async fn start(context: Arc<Command<Text>>, state: Arc<State>) {
    help(context, state).await;
}

/// Command to display information on usage
pub(crate) async fn spoiler(context: Arc<Command<Text>>, state: Arc<State>) {
    help(context, state).await;
}

/// Command to display information on usage
pub(crate) async fn help(context: Arc<Command<Text>>, _state: Arc<State>) {
    let bot_username = match context.bot().get_me().call().await {
        Ok(me) => format!("@{}", me.user.username.unwrap_or(me.user.first_name)),
        Err(err) => {
            dbg!(err.to_string());
            "my bot username".to_string()
        }
    };
    if let Err(e) = context.send_message_in_reply(&format!("To create a spoiler, start your message with {}", bot_username)).call().await {
        dbg!(e);
    }
}

fn random_id() -> String {
    rand::thread_rng()
        .sample_iter(Alphanumeric)
        .take(60)
        .map(char::from)
        .collect::<String>()
}
