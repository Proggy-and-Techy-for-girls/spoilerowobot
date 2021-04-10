use tbot::types::{
    message::Text, Animation, Audio, Contact, Dice, Document, Location, PhotoSize, Sticker, Video,
    VideoNote, Voice,
};

/// Spoiler.
/// todo doc
#[derive(Clone)]
pub(crate) struct Spoiler {
    /// The spoiler id. todo do i need this?
    pub(crate) id: String,
    /// The title of the Spoiler
    pub(crate) title: Option<String>,
    /// The Content.
    pub(crate) content: Content,
}

impl Spoiler {
    /// Create a new Spoiler Instance.
    pub(super) fn new(id: String, title: Option<String>, content: Content) -> Self {
        Spoiler { id, title, content }
    }
}

/// The type of the spoiled Content.
///
/// If it is a Text with less than max characters, the spoiled message can be shown in an alert.
/// Otherwise, the text needs to be shown in a private message of the requester. This is due to a
/// telegram limitation.
///
/// All other Types (images, videos, documents,…) always need to be shown in a private message with
/// the requester.
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
    Text(Text),

    /// This one is a workaround for created spoilers from inline queries since we have
    /// no matching Text struct available to save and creating an artificially is not permitted
    /// since `Text` is marked as *non-exhaustive*.
    String(String),
    Video(Video),
    VideoNote(VideoNote),
    Voice(Voice),
}

/// The spoiler creation state
///
/// These states mark points where the bot is expecting an input from the user.
#[derive(Eq, PartialEq, Debug)]
pub(crate) enum SpoilerCreationStatus {
    /// The bot is currently waiting for a message from the user of the content to be spoiled.
    WaitingForSpoiler,
    /// The bot is currently waiting for the title of the spoiler from the user.
    WaitingForTitle,
}
