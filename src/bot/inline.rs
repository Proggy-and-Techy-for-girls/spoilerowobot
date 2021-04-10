use std::sync::Arc;

use tbot::{
    contexts::{fields::Context, Inline},
    types::{inline_query, input_message_content, keyboard::inline, parameters},
};

use crate::{
    state::{spoiler::Content, State},
    strings::{INLINE_QUERY_SEPARATOR, MAJOR_SPOILER_IDENTIFIER},
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
    let spoiler_title = match state.get_spoiler_title(&context.query).await {
        Some(title) => title,
        None => {
            if context.query.contains(":::") {
                context.query.split(":::").collect::<Vec<&str>>()[0].to_string()
            } else {
                "".to_string()
            }
        }
    };
    let spoiler_content = if context.query.contains(":::") {
        context.query.split(":::").collect::<Vec<&str>>()[1].to_string()
    } else {
        context.query.clone()
    };

    let query = if util::is_spoiler_id(&context.query) {
        context.query.clone()
    } else {
        // create new spoiler, returns spoiler id
        state
            .new_spoiler(context.from.id.clone(), Content::String(spoiler_content))
            .await;
        format!(
            "{}{}",
            INLINE_QUERY_SEPARATOR,
            state
                .set_spoiler_title(context.from.id.clone(), spoiler_title.clone())
                .await
        )
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

    let minor_button_kind = inline::ButtonKind::CallbackData(&query);
    let minor_spoiler_keyboard_markup: inline::Markup =
        &[&[inline::Button::new("Show spoiler", minor_button_kind)]];

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

    let cd = format!("{}{}", MAJOR_SPOILER_IDENTIFIER, query);
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
