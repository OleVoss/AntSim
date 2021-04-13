use anyhow::Result;
use na::{Point2, Rotation2};
use nalgebra as na;
use rand::Rng;
use std::{ops::Mul, rc::Rc};

use crate::{
    config::{self, Config, SharedConfig},
    core::map::map::Map,
    utils::AntDirection,
};

use super::colony::{AntCollection, Colony};

#[derive(Default)]
pub struct Simulation {
    pub map: Map,
    pub done: bool,
    pub colony: Colony,
    // environment config
    config: SharedConfig,
}

impl Simulation {
    pub fn new(map: Map) -> Self {
        let mut colony = Colony::default();
        for x in -10..10 {
            colony.add_ant(2, Point2::new(x, 0));
        }
        Self {
            map,
            done: false,
            colony,
            config: Rc::new(Config::init()),
        }
    }

    pub fn step(&mut self) -> Result<()> {
        Simulation::ant_system(&mut self.colony.ants, &self.map);

        Ok(())
    }

    fn ant_system(ants: &mut AntCollection, map: &Map) {
        for (id, (speed, (position, direction))) in ants.id.iter().zip(
            ants.speed
                .iter()
                .zip(ants.position.iter_mut().zip(ants.direction.iter_mut())),
        ) {
            let mut rng = rand::thread_rng();
            // rotate random
            if rng.gen_ratio(1, 3) {
                *direction = rand::random();
            };
            // calculate new position
            let x_new = position.x + direction.vec().x * speed;
            let y_new = position.y + direction.vec().y * speed;
            let new_pos = Point2::new(
                x_new.clamp(map.min_width(), map.max_width()),
                y_new.clamp(map.min_height(), map.max_height()),
            );
            *position = new_pos;
        }
    }
}
