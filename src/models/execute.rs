use super::{config::RunType, storage::JsonStorage};

pub struct Actor {
    storage: JsonStorage,
    run_type: RunType,
}
