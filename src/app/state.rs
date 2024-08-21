use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

// In this reference impementation we only store the infoID and seqID of the messages.
// This is the minimum we need to store in order to return this information to ICS as
// part of the message sync process, where ICS will ask us the current messages we hold.
// Furthermore, we only store the latest version of every message, i.e. the highest
// seqID for every infoID. Our implementation doesn't do any safety checks to ensure
// that new versions of a message are actually coming in with a higher seqID. We simply
// store the latest seqID we've received and blindly assume that the server side sends
// them in the correct order. A sophisticated implementation should do safety checks.
#[derive(Clone, Debug, Default)]
pub struct AppState {
    // We store the infoID and seqID of a message in a hashmap, such that we can easily
    // access the current seqID for a given message and replace it when an update occurs.
    // The hashmap sits behind a read/write lock such that at all times there's either
    // one writer or an arbitrary amount of readers. We wrap the lock in an Arc such that
    // all threads may have shared ownership of the lock and request it when necessary.
    pub messages: Arc<RwLock<HashMap<String, u32>>>,
}
