use anyhow::Result;
use std::rc::Rc;

use crate::{
    config::{Config, SharedConfig},
    core::map::course::Map,
};

#[derive(Default)]
pub struct Simulation {
    pub course: Option<Map>,
    pub done: bool,
    // environment config
    config: SharedConfig,
}

impl Simulation {
    pub fn new(course: Map) -> Self {
        Self {
            course: Some(course),
            done: false,
            config: Rc::new(Config::init()),
        }
    }

    pub fn step(&mut self) -> Result<()> {
        Ok(())
    }
}
