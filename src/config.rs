use std::{convert::TryInto, rc::Rc};

use anyhow::Result;
use serde::__private::de;

pub type SharedConfig = Rc<Config>;

pub const STEPS: i64 = 10;

#[derive(Debug)]
pub struct Config {
    // stat bounds
// environment: Environment,
}

impl Default for Config {
    fn default() -> Self {
        Self {}
    }
}

impl Config {
    pub fn init() -> Self {
        Self::default()
    }
}
