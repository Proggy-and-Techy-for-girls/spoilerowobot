use std::sync::Arc;

use tbot::{
    contexts::{fields::Context, Inline},
    types::{inline_query, input_message_content, keyboard::inline, parameters},
};

use crate::{
    state::{
        spoiler::{Content, Spoiler},
        State,
    },
    strings::INLINE_QUERY_SEPARATOR,
    util,
};

/// Handle inline queries
///
/// Upon typing the text to be spoiled, the bot will display three options to create a spoiler:
/// - A *minor Spoiler*, where the message is hidden behind an alert
///   that requires a single tap to open.
/// - A *major Spoiler*, where the message is hidden behind an alert
///   that requires two taps to open.
/// - An advanced spoiler, where the user can upload images,
///   videos etc. and opitonally set a title for the spoiler
pub(crate) async fn inline(context: Arc<Inline>, state: Arc<State>) {
    let query = if util::is_spoiler_id(&context.query) {
        context
            .query
            .clone()
            .split(INLINE_QUERY_SEPARATOR)
            .collect()
    } else {
        context.query.clone()
    };

    let spoiler_title = state.get_spoiler_title(&query).await;
    let start_url = format!(
        "https://t.me/{}?start={}",
        context
            .bot()
            .get_me()
            .call()
            .await
            .unwrap()
            .user
            .username
            .unwrap(),
        &context.query
    );

    let spoiler_text_value = if util::is_spoiler_id(&context.query) {
        match state.get_spoiler(query.clone()).await {
            None => "".to_string(),
            Some(spoiler) => match spoiler.content {
                Content::Text(text) => text.value,
                _ => "".to_string(),
            },
        }
    } else {
        "".to_string()
    };
    let button_kind = if util::is_spoiler_id(&context.query) {
        match state.get_spoiler(query.clone()).await {
            None => inline::ButtonKind::Url(&start_url),
            Some(spoiler) => match spoiler.content {
                Content::Text(text) => {
                    if text.value.chars().count() <= 200 {
                        inline::ButtonKind::CallbackData(&*spoiler_text_value)
                    } else {
                        inline::ButtonKind::Url(&start_url)
                    }
                }
                _ => inline::ButtonKind::Url(&start_url),
            },
        }
    } else {
        inline::ButtonKind::CallbackData(&query)
    };

    // Minor spoiler
    let minor_spoiler = &format!(
        "<i>Minor spoiler!</i>{}",
        format!("\n<code>{}</code>", &spoiler_title)
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

    let minor_spoiler_keyboard_markup: inline::Markup =
        &[&[inline::Button::new("Show spoiler", button_kind.clone())]];

    let minor_spoiler_result = inline_query::Result::new(&rid, minor_spoiler)
        .reply_markup(inline::Keyboard::new(minor_spoiler_keyboard_markup));

    // Major spoiler
    let major_spoiler = &*format!(
        "<b>Major spoiler!</b>{}",
        format!("\n<code>{}</code>", spoiler_title)
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

    let major_spoiler_keyboard_markup: inline::Markup = &[&[inline::Button::new(
        "Double tap to show spoiler",
        button_kind,
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
