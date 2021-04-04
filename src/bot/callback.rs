use std::sync::Arc;

use tbot::contexts::{fields::Context, methods::Callback, DataCallback};

use crate::{state::spoiler::Content, strings::INLINE_QUERY_SEPARATOR, util::is_spoiler_id, State};

/// Data callback handler
pub(crate) async fn data_callback(context: Arc<DataCallback>, state: Arc<State>) {
    if is_spoiler_id(&context.data) {
        let spoiler_id = context
            .data
            .clone()
            .split(INLINE_QUERY_SEPARATOR)
            .collect::<String>();

        if let Some(spoiler) = state.get_spoiler(spoiler_id).await {
            match &spoiler.content {
                Content::Text(text) => {
                    if text.value.chars().count() <= 200 {
                        if let Err(e) = context.alert(&text.value).call().await {
                            dbg!(e);
                        }
                    } else {
                        if let Err(e) = context
                            .bot()
                            .send_message(context.from.id, &text.value)
                            .call()
                            .await
                        {
                            dbg!(e);
                        }
                    }
                    return;
                }
                /*
                Content::Audio(audio) => {
                    if let Err(e) = context
                        .bot()
                        .send_audio(context.from.id, Audio::with_id(audio.file_id.as_ref()))
                        .call()
                        .await
                    {
                        dbg!(e);
                    }
                    return;
                }

                */
                _ => {
                    return;
                }
            }
        }
    } else {
        if context.data.chars().count() <= 200 {
            if let Err(e) = context.alert(&context.data).call().await {
                dbg!(e);
            }
        } else {
            if let Err(e) = context
                .bot()
                .send_message(context.from.id, &context.data)
                .call()
                .await
            {
                dbg!(e);
            }
        }
    }
}
