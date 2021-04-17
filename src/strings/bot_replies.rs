//! A module containing all possible replies the bot could send.
use super::SPOILER_TITLE_SEPARATOR;

/// Informs the user to send the content to be spoiled.
pub(crate) static PREPARING_A_SPOILER: &'static str =
    "Preparing a spoiler. To cancel, type /cancel.

First send the content to be spoiled. It can be text, photo, or any other media.";

/// Informs the user that the spoiler creation process has been cancelled.
pub(crate) static SPOILER_CREATION_CANCELLED: &'static str =
    "The spoiler creation has been cancelled.";

/// Informs the user that the spoiler is now ready.
pub(crate) static SPOILER_READY: &'static str = "Done! Your advanced spoiler is ready.";

/// Informs the user that the spoiler could not be found.
pub(crate) static SPOILER_NOT_FOUND: &'static str =
    "Spoiler not found! It might have expired already...";

/// Informs the user how to start the bot.
pub(crate) static TYPE_START: &'static str =
    "Type /start to prepare an advanced spoiler with a custom title.";

/// Informs the user to now send a title for the spoiler.
pub(crate) static NOW_SEND_A_TITLE: &'static str =
    "Now send a title for the spoiler (maximum 256 characters).
It will be immediately visible and can be used to add a small description for your spoiler.
Type a dash (-) now if you do not want a title for your spoiler.";

/// Informs the user to tap again to show the spoiler
pub(crate) static TAP_AGAIN_TO_SHOW_SPOILER: &'static str = "Please tap again to see the spoiler";

/// Sends information how to use this bot.
pub(crate) fn help_text(bot_username: String) -> String {
    format!(
        "Type /start to prepare an advanced spoiler with a custom title.

You can type quick spoilers by using @{} in inline mode:
@{} your spoiler message…

Custom titles can also be used from inline mode as follows:
@{} title for the spoiler{}contents of the spoiler
Note that the title will be immediately visible!",
        bot_username, bot_username, bot_username, SPOILER_TITLE_SEPARATOR
    )
}
