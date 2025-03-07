//! Methods related to handling [inline queries][tg doc].
//!
//! [tg doc]: https://core.telegram.org/bots/api#inline-mode
use std::sync::Arc;

use tbot::{
    contexts::Inline,
    types::{inline_query, input_message_content, keyboard::inline, parameters},
};

use crate::{
    state::{spoiler::Content, State},
    strings::{
        INLINE_QUERY_SEPARATOR, MAJOR_SPOILER_IDENTIFIER, SHOW_SPOILER, SPOILER_TITLE_SEPARATOR,
    },
    util,
};

/// Handle [inline queries]
///
/// Upon typing the text to be spoiled, the bot will display three options to create a spoiler:
/// - A *minor Spoiler*, where the message is hidden behind an [alert]
///   that requires a single tap to open.
/// - A *major Spoiler*, where the message is hidden behind an [alert]
///   that requires two taps to open.
/// - An advanced spoiler, where the user can upload images,
///   videos etc. and optionally set a title for the spoiler
///
/// [inline queries]: https://core.telegram.org/bots/api#inline-mode
/// [alert]: https://core.telegram.org/bots/api#answercallbackquery
pub(crate) async fn inline(context: Arc<Inline>, state: Arc<State>) {
    let spoiler_title = parse_spoiler_title(context.clone(), state.clone()).await;
    let spoiler_id = parse_spoiler_id(context.clone(), state.clone()).await;
    let expires_in = expires_in(&spoiler_id, state.clone());

    // Minor spoiler
    let minor_spoiler = &format!(
        "<i>Minor spoiler!</i>{}{}",
        format!("\n<code>{}</code>", &spoiler_title),
        &expires_in,
    );
    let generated_minor_spoiler =
        input_message_content::Text::new(parameters::Text::with_html(minor_spoiler));

    let minor_spoiler =
        inline_query::result::Article::new("Minor Spoiler", generated_minor_spoiler)
            .description("Text, single tap")
            .thumb(
                inline_query::result::Thumb::new("https://i.imgur.com/csh5H5O.png")
                    .width(512)
                    .height(512),
            );
    let rid = util::random_id();

    let minor_button_kind = inline::ButtonKind::CallbackData(&spoiler_id);
    let minor_spoiler_keyboard_markup: inline::Markup =
        &[&[inline::Button::new(SHOW_SPOILER, minor_button_kind)]];

    let minor_spoiler_result = inline_query::Result::new(&rid, minor_spoiler)
        .reply_markup(inline::Keyboard::new(minor_spoiler_keyboard_markup));

    // Major spoiler
    let major_spoiler = &*format!(
        "<b>Major spoiler!</b>{}{}",
        format!("\n<code>{}</code>", spoiler_title),
        &expires_in
    );
    let generated_major_spoiler =
        input_message_content::Text::new(parameters::Text::with_html(major_spoiler));

    let major_spoiler =
        inline_query::result::Article::new("Major Spoiler", generated_major_spoiler)
            .description("Text, double tap")
            .thumb(
                inline_query::result::Thumb::new("https://i.imgur.com/3qqCZZk.png")
                    .width(512)
                    .height(512),
            );

    let cd = format!("{}{}", MAJOR_SPOILER_IDENTIFIER, spoiler_id);
    let major_button_kind = inline::ButtonKind::CallbackData(&cd);
    let major_spoiler_keyboard_markup: inline::Markup = &[&[inline::Button::new(
        "Double tap to show spoiler",
        major_button_kind,
    )]];

    let rid: String = util::random_id();
    let major_spoiler_result = inline_query::Result::new(&rid, major_spoiler)
        .reply_markup(inline::Keyboard::new(major_spoiler_keyboard_markup));

    if let Err(e) = context
        .answer(&[minor_spoiler_result, major_spoiler_result])
        .is_personal(true)
        .switch_pm("Advanced spoiler (media etc.)…", "create_custom_spoiler")
        .call()
        .await
    {
        dbg!(e.to_string());
    }
}

/// Parse the (optional) spoiler title from an [inline query]
///
/// The spoiler title can either be
/// - provided by the user in the inline query,
/// - already defined while creating a custom spoiler or
/// - not provided at all.
///
/// The bot user can provide a spoiler title by formatting the spoiler message as follows:
/// ```
/// spoiler title:::message to be spoiled
/// ```
///
/// [inline query]: https://core.telegram.org/bots/api#inline-mode
async fn parse_spoiler_title(context: Arc<Inline>, state: Arc<State>) -> String {
    let spoiler_id = if util::is_spoiler_id(&context.query) {
        context
            .query
            .clone()
            .rsplit(INLINE_QUERY_SEPARATOR)
            .take(1)
            .collect::<String>()
    } else {
        context.query.clone()
    };

    match state.get_spoiler_title(&spoiler_id) {
        Some(title) => title,
        None => {
            if context.query.contains(SPOILER_TITLE_SEPARATOR) {
                context
                    .query
                    .split(SPOILER_TITLE_SEPARATOR)
                    .collect::<Vec<&str>>()[0]
                    .to_string()
            } else {
                "".to_string()
            }
        }
    }
}

/// Parses the spoiler content from an [inline query]
///
/// [inline query]: https://core.telegram.org/bots/api#inline-mode
async fn parse_spoiler_content(context: Arc<Inline>) -> String {
    if context.query.contains(SPOILER_TITLE_SEPARATOR) {
        context
            .query
            .split(SPOILER_TITLE_SEPARATOR)
            .collect::<Vec<&str>>()[1]
            .to_string()
    } else {
        context.query.clone()
    }
}

/// Parses the spoiler id from the given query.
///
/// If a spoiler id is provided, this id will be returned.
/// Otherwise, it creates a new spoiler from the user input and returns that id.
async fn parse_spoiler_id(context: Arc<Inline>, state: Arc<State>) -> String {
    if util::is_spoiler_id(&context.query) {
        context.query.clone()
    } else {
        // Create a new spoiler from the inline query and return the spoiler id
        let spoiler_content = parse_spoiler_content(context.clone()).await;
        let spoiler_content = util::strip_expiration_suffix(&spoiler_content);
        let spoiler_title = parse_spoiler_title(context.clone(), state.clone()).await;
        let duration = util::parse_duration(&context.query);
        dbg!(duration);

        state.new_spoiler(context.from.id.clone(), Content::String(spoiler_content));

        format!(
            "{}{}",
            INLINE_QUERY_SEPARATOR,
            state.set_spoiler_title_and_expiration(
                context.from.id.clone(),
                spoiler_title.clone(),
                duration
            )
        )
    }
}

/// Returns a string representation of when the specified spoiler will expire.
fn expires_in(spoiler_id: &String, state: Arc<State>) -> String {
    let id = if util::is_spoiler_id(spoiler_id) {
        spoiler_id
            .clone()
            .split(INLINE_QUERY_SEPARATOR)
            .collect::<String>()
    } else {
        spoiler_id.clone()
    };

    match state.get_spoiler(&id) {
        None => "".to_string(),
        Some(spoiler) => format!("\n\n(Expires at {})", util::expires_at(spoiler.expires_in)),
    }
}
