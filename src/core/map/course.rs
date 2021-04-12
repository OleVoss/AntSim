use getset::{Getters, MutGetters, Setters};
use serde::{Deserialize, Serialize};

#[derive(Getters, MutGetters, Setters, Deserialize)]
pub struct Map {
    name: String,
}

impl Map {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
