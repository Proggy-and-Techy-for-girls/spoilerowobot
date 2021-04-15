//! A spoiler to be held in the bot's state.
use tbot::types::{
    Animation, Audio, Contact, Dice, Document, Location, message::Text, PhotoSize, Sticker, Video,
    VideoNote, Voice,
};
use tokio::time::Duration;

use crate::util::DAY_IN_SECS;

/// Information about a Spoiler.
#[derive(Clone)]
pub(crate) struct Spoiler {
    /// The spoiler id. todo do i need this?
    pub(crate) id: String,
    /// The title of the Spoiler. Setting a spoiler title is optional.
    pub(crate) title: Option<String>,
    /// The spoiled content.
    pub(crate) content: Content,
    /// The amount of time until the spoiler expires.
    pub(crate) expires_in: Duration,
}

impl Spoiler {
    /// Creates a new Spoiler.
    pub(super) fn new(
        id: String,
        title: Option<String>,
        content: Content,
        expires_in: Option<Duration>,
    ) -> Self {
        Spoiler {
            id,
            title,
            content,
            expires_in: expires_in.unwrap_or_else(|| Duration::from_secs(DAY_IN_SECS)),
        }
    }
}

/// An enum holding information about the spoiled content.
#[non_exhaustive]
#[derive(Clone)]
pub(crate) enum Content {
    Animation(Animation),
    Audio(Audio),
    Contact(Contact),
    Dice(Dice),
    Document(Document),
    Location(Location),
    Photo(Vec<PhotoSize>),
    Sticker(Sticker),

    /// This one is a workaround for created spoilers from inline queries since we have
    /// no matching Text message available to save.
    String(String),
    Text(Text),
    Video(Video),
    VideoNote(VideoNote),
    Voice(Voice),
}

/// Current status of the spoiler creation process.
///
/// These model the states where the bot is expecting an input from the user.
#[derive(Eq, PartialEq, Debug)]
pub(crate) enum SpoilerCreationStatus {
    /// The bot is currently waiting for a message from the user of the content to be spoiled.
    WaitingForSpoiler,
    /// The bot is currently waiting for the title of the spoiler from the user.
    WaitingForTitle,
}
