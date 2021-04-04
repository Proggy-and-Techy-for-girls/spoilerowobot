//! todo doc
use std::{collections::HashMap, sync::Arc};

use tbot::types::user;
use tokio::sync::Mutex;

use crate::{state::spoiler::Content, strings::ERROR_NO_CONTENT, util::random_id};

use self::spoiler::{Spoiler, SpoilerCreationStatus};

// todo convert to std::sync::Mutex

pub(crate) mod spoiler;

/// The bot's state.
#[derive(Default)]
pub(crate) struct State {
    /// A key-value store to track the progress of users creating custom spoilers.
    ///
    /// If a user starts the bot, a new key-value pair is inserted into the Hashmap.
    /// If a user cancels the operation or finishes creating their spoiler, the corresponding
    /// key-value pair gets deleted again.
    creation_status: Arc<Mutex<HashMap<user::Id, SpoilerCreationStatus>>>,

    /// A key-value store of spoilers that have not been fully created yet.
    new_spoilers: Arc<Mutex<HashMap<user::Id, Content>>>,

    /// A key-value store of all currently registered spoilers.
    spoilers: Arc<Mutex<HashMap<String, Spoiler>>>,
}

impl State {
    /// Wait for the user to send a spoiler
    pub(crate) async fn set_waiting_for_spoiler(
        &self,
        user: user::Id,
    ) -> Option<SpoilerCreationStatus> {
        self.creation_status
            .lock()
            .await
            .insert(user, SpoilerCreationStatus::WaitingForSpoiler)
    }

    /// Wait for the user to send a title for the spoiler
    pub(crate) async fn set_waiting_for_title(
        &self,
        user: user::Id,
    ) -> Option<SpoilerCreationStatus> {
        self.creation_status
            .lock()
            .await
            .insert(user, SpoilerCreationStatus::WaitingForTitle)
    }

    /// Cancel the spoiler creation and remove the corresponding value from the state.
    pub(crate) async fn cancel_spoiler_creation(
        &self,
        user: &user::Id,
    ) -> Option<SpoilerCreationStatus> {
        self.creation_status.lock().await.remove(user)
    }

    /// Return `true` if the bot is waiting for the user to specify a title.
    pub(crate) async fn waiting_for_title(&self, user: &user::Id) -> bool {
        match self.creation_status.lock().await.get(user) {
            Some(state) => state.eq(&SpoilerCreationStatus::WaitingForTitle),
            None => false,
        }
    }

    /// Return `true` if the bot is waiting for the content to be spoiled.
    pub(crate) async fn waiting_for_spoiler(&self, user: &user::Id) -> bool {
        match self.creation_status.lock().await.get(user) {
            Some(state) => state.eq(&SpoilerCreationStatus::WaitingForSpoiler),
            None => false,
        }
    }

    /// Create a new Spoiler and add it to the state.
    pub(crate) async fn new_spoiler(&self, user: user::Id, content: Content) {
        self.new_spoilers.lock().await.insert(user, content);
    }

    /// Get the title of the requested spoiler
    pub(crate) async fn get_spoiler_title(&self, spoiler_id: &String) -> String {
        match self.spoilers.lock().await.get(spoiler_id) {
            Some(spoiler) => spoiler.title.as_ref().unwrap_or(&"".to_string()).to_owned(),
            None => "".to_string(),
        }
    }

    /// Set the title of the spoiler
    ///
    /// If the user submits a single dash (-), the title creation will be skipped.
    ///
    /// Returns:
    /// the spoiler id
    pub(crate) async fn set_spoiler_title(&self, user_id: user::Id, title: String) -> String {
        let title = if title.eq("-") { None } else { Some(title) };

        let spoiler_id = random_id();

        let content = { self.new_spoilers.lock().await.remove(&user_id) };
        match content {
            Some(content) => {
                self.spoilers.lock().await.insert(
                    spoiler_id.to_owned(),
                    Spoiler::new(spoiler_id.to_owned(), title, content),
                );
            }
            None => {
                dbg!(ERROR_NO_CONTENT);
            }
        }

        return spoiler_id;
    }

    /// Return the spoiler by the specified spoiler id.
    pub(crate) async fn get_spoiler(&self, id: String) -> Option<Spoiler> {
        let spoiler: Option<Spoiler>;
        {
            spoiler = self.spoilers.lock().await.remove(&id);
        }
        match spoiler {
            Some(spoiler) => {
                self.spoilers.lock().await.insert(id, spoiler.clone());
                Some(spoiler)
            }
            None => None,
        }
    }
}
