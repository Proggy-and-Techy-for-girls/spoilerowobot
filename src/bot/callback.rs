use std::sync::Arc;

use tbot::contexts::{methods::Callback, DataCallback};

use crate::state::State;

/// Data callback handler
pub(crate) async fn data_callback(context: Arc<DataCallback>, _state: Arc<State>) {
    if let Err(e) = context.alert(&context.data).call().await {
        dbg!(e);
    }
}
