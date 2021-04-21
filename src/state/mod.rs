//! A State that get attached to the bot's event loop.
use std::{
    collections::HashMap,
    ops::Deref,
    sync::{Arc, Mutex},
};

use tbot::types::user;
use tokio::time::{delay_queue, DelayQueue, Duration};

use crate::{state::spoiler::Content, strings::ERROR_NO_CONTENT, util};

use self::spoiler::{Spoiler, SpoilerCreationStatus};

pub(crate) mod periodic;
pub(crate) mod spoiler;

/// The bot's state.
///
/// The state holds information about
/// - users currently creating a new spoiler,
/// - A map of all registered spoilers and
/// - A queue that yields the next spoiler that is going to expire.
#[derive(Default)]
pub(crate) struct State {
    /// A key-value store to track the progress of users creating a custom spoiler.
    ///
    /// Once a user starts the bot, a new key-value pair is inserted into the map.
    /// If a user cancels the operation or finishes creating their spoiler, the corresponding
    /// key-value pair gets deleted again.
    pub(self) creation_status: Arc<Mutex<HashMap<user::Id, SpoilerCreationStatus>>>,

    /// A key-value store of spoilers that have not been fully created yet.
    pub(self) new_spoilers: Arc<Mutex<HashMap<user::Id, Content>>>,

    /// A key-value store of users currently trying to open a major spoiler (which requires a double tap).
    pub(self) open_major_spoiler: Arc<Mutex<HashMap<(user::Id, String), ()>>>,

    /// A queue that holds information about which spoiler is going to expire next.
    pub(self) expirations: Mutex<DelayQueue<String>>,

    /// A key-value store of all currently registered spoilers with information about when the entry shall be yielded back.
    pub(self) spoilers: Mutex<HashMap<String, (Spoiler, delay_queue::Key)>>,
}

impl State {
    /// Waits for the user to send a spoiler.
    pub(crate) fn set_waiting_for_spoiler(&self, user: user::Id) -> Option<SpoilerCreationStatus> {
        self.creation_status
            .lock()
            .unwrap()
            .insert(user, SpoilerCreationStatus::WaitingForSpoiler)
    }

    /// Waits for the user to send a title for the spoiler.
    pub(crate) fn set_waiting_for_title(&self, user: user::Id) -> Option<SpoilerCreationStatus> {
        self.creation_status
            .lock()
            .unwrap()
            .insert(user, SpoilerCreationStatus::WaitingForTitle)
    }

    /// Cancels the spoiler creation and remove the corresponding value from the state.
    pub(crate) fn cancel_spoiler_creation(&self, user: &user::Id) -> Option<SpoilerCreationStatus> {
        self.creation_status.lock().unwrap().remove(user)
    }

    /// Returns `true` if the bot is waiting for the user to specify a title.
    pub(crate) fn waiting_for_title(&self, user: &user::Id) -> bool {
        match self.creation_status.lock().unwrap().get(user) {
            Some(state) => state.eq(&SpoilerCreationStatus::WaitingForTitle),
            None => false,
        }
    }

    /// Returns `true` if the bot is waiting for the content to be spoiled.
    pub(crate) fn waiting_for_spoiler(&self, user: &user::Id) -> bool {
        match self.creation_status.lock().unwrap().get(user) {
            Some(state) => state.eq(&SpoilerCreationStatus::WaitingForSpoiler),
            None => false,
        }
    }

    /// Creates a new Spoiler and add it to the state.
    pub(crate) fn new_spoiler(&self, user: user::Id, content: Content) {
        self.new_spoilers.lock().unwrap().insert(user, content);
    }

    /// Gets the title of the requested spoiler
    pub(crate) fn get_spoiler_title(&self, spoiler_id: &String) -> Option<String> {
        match self.spoilers.lock().unwrap().get(spoiler_id) {
            Some(spoiler) => Some(
                spoiler
                    .0
                    .title
                    .as_ref()
                    .unwrap_or(&"".to_string())
                    .to_owned(),
            ),
            None => None,
        }
    }

    /// Sets the title of the spoiler and the time after which it should expire.
    ///
    /// If the user submits a single dash (-), the title creation will be skipped.
    ///
    /// The default expiration time set to 1 day.
    ///
    /// # Returns
    /// The id of the newly created spoiler.
    pub(crate) fn set_spoiler_title_and_expiration(
        &self,
        user_id: user::Id,
        title: String,
        expires_in: Option<Duration>,
    ) -> String {
        let title = util::strip_expiration_suffix(&title);
        let title = if title.eq("-") { None } else { Some(title) };

        let spoiler_id = util::random_id();

        let content = { self.new_spoilers.lock().unwrap().remove(&user_id) };
        match content {
            Some(content) => {
                let spoiler = Spoiler::new(spoiler_id.to_owned(), title, content, expires_in);
                self.add_spoiler_to_queue(spoiler);
            }
            None => {
                dbg!(ERROR_NO_CONTENT);
            }
        }

        return spoiler_id;
    }

    /// Returns the spoiler by the specified spoiler id.
    pub(crate) fn get_spoiler(&self, id: &String) -> Option<Spoiler> {
        match self.spoilers.lock().unwrap().get(id) {
            Some(val) => Some(val.deref().0.clone()),
            None => None,
        }
    }

    /// Return true if the user needs to tap once more to the spoiler button
    ///
    /// This internally checks how often the user has tried to open a given spoiler.
    pub(crate) fn needs_to_tap_once_more(&self, user: &user::Id, spoiler_id: &String) -> bool {
        let mut open_major_spoiler = self.open_major_spoiler.lock().unwrap();

        match open_major_spoiler.remove(&(user.clone(), spoiler_id.clone())) {
            Some(()) => false,
            None => {
                open_major_spoiler.insert((*user, spoiler_id.clone()), ());
                true
            }
        }
    }

    /// Add a Spoiler to the DelayQueue
    fn add_spoiler_to_queue(&self, spoiler: Spoiler) {
        let delay_key;
        {
            delay_key = self
                .expirations
                .lock()
                .unwrap()
                .insert(spoiler.id.clone(), spoiler.expires_in);
        }
        {
            self.spoilers
                .lock()
                .unwrap()
                .insert(spoiler.id.clone(), (spoiler, delay_key));
        }
    }
}
