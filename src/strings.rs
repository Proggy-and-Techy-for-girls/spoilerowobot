//! todo doc
pub static CREATE_CUSTOM_SPOILER: &'static str = "create_custom_spoiler";
pub static PREPARING_A_SPOILER: &'static str = "Preparing a spoiler. To cancel, type /cancel.

First send the content to be spoiled. It can be text, photo, or any other media.";
pub static SPOILER_CREATION_CANCELLED: &'static str = "The spoiler creation has been cancelled.";
pub static SPOILER_READY: &'static str = "Done! Your advanced spoiler is ready.";
pub static TYPE_START: &'static str =
    "Type /start to prepare an advanced spoiler with a custom title.";
pub static NOW_SEND_A_TITLE: &'static str =
    "Now send a title for the spoiler (maximum 256 characters).
It will be immediately visible and can be used to add a small description for your spoiler.
Type a dash (-) now if you do not want a title for your spoiler.";

pub(crate) fn help_text(bot_username: String) -> String {
    format!(
        "Type /start to prepare an advanced spoiler with a custom title.

You can type quick spoilers by using @{} in inline mode:
@{} your spoiler message…

Custom titles can also be used from inline mode as follows:
@{} title for the spoiler:::contents of the spoiler
Note that the title will be immediately visible!",
        bot_username, bot_username, bot_username
    )
}

pub static ERROR_UNKNOWN_USER: &'static str =
    "ERROR: Could not determine who sent me that message!";
pub static ERROR_NO_CONTENT: &'static str = "No content?!?!?!";
pub static SEND_IT: &'static str = "Send it";

/// This is needed to distinguish between users creating spoilers using the inline mode
/// and spoiler ids passed as a result from creating a custom spoiler.
pub(crate) static INLINE_QUERY_SEPARATOR: &'static str = "id-_-";
