use std::{fs, path::Path, rc::Rc, thread};

use anyhow::{bail, Result};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use fs::read_to_string;
use serde::__private::de;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem, Tabs},
    Frame,
};

use crate::{
    config::{self, Config, SharedConfig},
    core::{map::map::Map, simulation::simulation::Simulation},
    draw,
    keys::{KeyConfig, SharedKeyConfig},
};

pub struct App {
    pub simulation: Simulation,
    pub should_quit: bool,
    pub config: SharedConfig,
    pub key_config: SharedKeyConfig,
}

impl App {
    pub fn new(initialize: bool) -> App {
        let mut app = App {
            simulation: Simulation::default(),
            should_quit: false,
            config: Rc::new(Config::init()),
            key_config: Rc::new(KeyConfig::init()),
        };

        if initialize {
            app.load_player();
            app.load_discs();
        };

        let simulation = Simulation::new(Map::new("Map", 100, 40));
        app.simulation = simulation;

        return app;
    }

    pub fn start(&mut self) -> Result<()> {
        self.simulation.step()?;
        Ok(())
    }

    pub fn step(&mut self) -> Result<()> {
        self.simulation.step()?;
        Ok(())
    }
}

// private impls
impl App {
    // TODO: Error handling
    pub fn load_player(&mut self) {
        // let contents = include_str!("../assets/player.ron");
        // let roaster: PlayerRoaster = ron::from_str(&contents).unwrap();
        // self.player_roaster = roaster;
    }

    pub fn load_discs(&mut self) {
        // let contents = include_str!("../assets/discs.ron");
        // let storage: DiscStorage = ron::from_str(&contents).unwrap();
        // self.disc_storage = storage;
    }

    pub fn load_course() {
        // let contents = include_str!("../assets/course_1.ron");
        // let course_result: Result<Course, ron::Error> = ron::from_str(&contents);
        // return course_result;
    }
}
