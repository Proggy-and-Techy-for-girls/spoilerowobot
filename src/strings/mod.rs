//! A module containing all static strings.
pub(crate) mod bot_replies;

/// Distinguishes between spoiler ids and the spoiler creation via inline query.
pub(crate) static INLINE_QUERY_SEPARATOR: &'static str = "id-_-";

/// Distinguish the spoiler title from the spoiler content.
pub(crate) static SPOILER_TITLE_SEPARATOR: &'static str = ":::";

/// Marks a spoiler as *major Spoiler*.
pub(crate) static MAJOR_SPOILER_IDENTIFIER: &'static str = "maj_";

/// Sent whenever a user switches from inline mode to a PM with the bot.
pub(crate) static CREATE_CUSTOM_SPOILER: &'static str = "create_custom_spoiler";

/// Indicates the content could not be found.
pub(crate) static ERROR_NO_CONTENT: &'static str = "No content?!?!?!";

/// Instructs the user to send it.
pub(crate) static SEND_IT: &'static str = "Send it";

/// Show spoiler
pub(crate) static SHOW_SPOILER: &'static str = "Show spoiler";
