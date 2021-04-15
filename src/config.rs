use std::{cmp, convert::TryInto, rc::Rc};

use anyhow::Result;
use ordered_float::OrderedFloat;
use serde::__private::de;

pub type SharedConfig = Rc<Config>;

#[derive(Debug, Clone, Copy)]
pub struct ConfigVar {
    name: &'static str,
    val: i32,
    min: i32,
    max: i32,
}

impl ConfigVar {
    pub fn new(name: &'static str, val: i32, min: i32, max: i32) -> Self {
        Self {
            name,
            val,
            min,
            max,
        }
    }

    pub fn set(&mut self, val: i32) {
        if val <= self.max && val >= self.min {
            self.val = val;
        } else if val <= self.min {
            self.val = self.min;
        } else {
            self.val = self.max;
        }
    }

    pub fn incr(&mut self) {
        self.set(self.val + 1);
    }

    pub fn decr(&mut self) {
        let amount = (self.max - self.min) / 10;
        self.set(self.val - 1);
    }

    pub fn val(&self) -> i32 {
        self.val
    }

    pub fn min(&self) -> i32 {
        self.min
    }

    pub fn max(&self) -> i32 {
        self.max
    }

    pub fn name(&self) -> &str {
        self.name
    }
}

pub const parameter_desc: [[&str; 2]; 8] = [
    ["Anthill height", "Sets the anthill height. This parameter does not affect the simulation."],
    ["Anthill width", "Sets the anthill width. This parameter does not affect the simulation."],
    ["Max seeker steps", "Maximal steps a seeker ant wanders around, searching for food. After the max is reached the and returns back home."],
    ["Minimal pheromone concentration", "Pheromone concentratin mandatory (on the tiles surrounding the nest) to spawn follower ants."],
    ["Map width", "Sets the map width."],
    ["Map height", "Sets the map height."],
    ["Pheromone evaporation amount (per step)", "Amount of pheromone that evaporates from each tile each timestep."],
    ["Pheromone drop", "Amount of pheromone, droped by an returning and (with food)."],
];
#[derive(Debug, Clone, Copy)]
pub struct Config {
    // stat bounds
    // environment: Environment,
    pub anthill_height: ConfigVar,
    pub anthill_width: ConfigVar,
    pub max_steps: ConfigVar,
    pub min_ph_c: ConfigVar,
    pub map_width: ConfigVar,
    pub map_height: ConfigVar,
    pub evaporation_rate: ConfigVar,
    pub ph_drop: ConfigVar,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            anthill_height: ConfigVar::new("Anthill height", 3, 1, 7),
            anthill_width: ConfigVar::new("Anthill width", 5, 3, 15),
            max_steps: ConfigVar::new("Max steps", 100, 0, 1000),
            min_ph_c: ConfigVar::new("Min Ph C", 10, 0, 90),
            map_width: ConfigVar::new("Map width", 115, 50, 350),
            map_height: ConfigVar::new("Map height", 46, 25, 200),
            evaporation_rate: ConfigVar::new("Evaporation rate (in %)", 2, 0, 25),
            ph_drop: ConfigVar::new("Ph drop", 79, 0, 100),
        }
    }
}

impl Config {
    pub fn init() -> Self {
        Self::default()
    }

    pub fn vars(&self) -> Vec<ConfigVar> {
        vec![
            self.anthill_height,
            self.anthill_width,
            self.max_steps,
            self.min_ph_c,
            self.map_width,
            self.map_height,
            self.evaporation_rate,
            self.ph_drop,
        ]
    }

    pub fn vars_mut(&mut self) -> Vec<&mut ConfigVar> {
        vec![
            &mut self.anthill_height,
            &mut self.anthill_width,
            &mut self.max_steps,
            &mut self.min_ph_c,
            &mut self.map_width,
            &mut self.map_height,
            &mut self.evaporation_rate,
            &mut self.ph_drop,
        ]
    }
}
