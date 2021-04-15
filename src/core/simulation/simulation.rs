use anyhow::Result;
use itertools::izip;
use rand::seq::IteratorRandom;
use rand::Rng;
use std::rc::Rc;

use crate::{
    config::{Config, SharedConfig},
    core::map::map::{Map, PhType, Pos, TileType},
};

use super::colony::{AntCollection, AntState, Colony};

#[derive(Default, Clone)]
pub struct SimData {
    pub step: Vec<i32>,
    // seeker returner follower noobs
    pub ants: Vec<[i32; 4]>,
    pub food_total: Vec<i32>,
    pub food_step: Vec<i32>,
}

impl SimData {
    pub fn new() -> Self {
        Self {
            step: Vec::new(),
            ants: Vec::new(),
            food_total: Vec::new(),
            food_step: Vec::new(),
        }
    }

    pub fn push(&mut self, step: i32, ants: [i32; 4], food_total: i32, food_step: i32) {
        self.step.push(step);
        self.ants.push(ants);
        self.food_total.push(food_total);
        self.food_step.push(food_step)
    }
}

#[derive(Default)]
pub struct Simulation {
    pub map: Map,
    pub done: bool,
    pub paused: bool,
    pub colony: Colony,
    pub history: SimData,
    // environment config
    config: Config,
}

impl Simulation {
    pub fn new(map: Map) -> Self {
        let mut colony = Colony::default();
        let (x_mid, y_mid) = (map.width() / 2, map.height() / 2);
        Self {
            map,
            done: false,
            paused: false,
            colony,
            history: SimData::new(),
            config: Config::init(),
        }
    }

    pub fn reset(&mut self, config: Config) {
        self.colony = Colony::default();
        self.map = Map::new(
            "Map",
            self.config.map_width.val(),
            self.config.map_height.val(),
        );
        self.config = config;
        self.done = false;
    }

    pub fn spwan_ant(&mut self) {
        let ant_state = Simulation::decide_ant_state(&self.map, &self.config);
        self.colony.add_ant(1, self.map.nest_pos, ant_state);
    }

    fn decide_ant_state(map: &Map, config: &Config) -> AntState {
        let neighbours = map.get_neightbours(map.nest_pos);
        let average_phc = neighbours
            .iter()
            .filter_map(|t| t.pheromone.as_ref())
            .map(|ph| ph.conc())
            .sum::<i32>();
        // / neighbours.len() as i32;

        if average_phc >= config.min_ph_c.val() {
            return AntState::Follower;
        } else {
            return AntState::Seeker;
        }
    }

    pub fn step(&mut self) -> Result<()> {
        // execute systems
        Simulation::ant_system(&mut self.colony, &mut self.map, &self.config);
        Simulation::ph_system(&mut self.map, &self.config);

        // history log
        let state_count = self.colony.ants.state_count();
        let ants: [i32; 4] = [
            *state_count.get(&AntState::Seeker).unwrap_or(&0),
            *state_count.get(&AntState::Returner).unwrap_or(&0),
            *state_count.get(&AntState::Follower).unwrap_or(&0),
            *state_count.get(&AntState::Noob).unwrap_or(&0),
        ];

        let food_rate =
            self.colony.food - self.history.food_total.last().cloned().unwrap_or_default();

        self.history.push(
            self.history.step.len() as i32,
            ants,
            self.colony.food,
            food_rate,
        );

        Ok(())
    }

    fn ant_system(colony: &mut Colony, map: &mut Map, config: &Config) {
        Simulation::seeker_system(colony, map, config);
        Simulation::returner_system(colony, map, config);
        Simulation::noob_system(colony, map, config);
        Simulation::follower_system(colony, map, config);
    }

    fn ph_system(map: &mut Map, config: &Config) {
        map.tile_matrix.iter_mut().for_each(|row| {
            row.iter_mut()
                .filter(|t| t.pheromone.is_some())
                .for_each(|t| t.evaporate(config.evaporation_rate.val()));
        })
    }

    fn returner_system(colony: &mut Colony, map: &mut Map, config: &Config) {
        for (id, speed, position, direction, state) in izip!(
            &colony.ants.id,
            &colony.ants.speed,
            &mut colony.ants.position,
            &mut colony.ants.direction,
            &mut colony.ants.state,
        )
        .filter(|(_id, _speed, _position, _direction, state)| **state == AntState::Returner)
        {
            let dir_x = (map.nest_pos.x - position.x).signum();
            let dir_y = (map.nest_pos.y - position.y).signum();

            let neighbours = map.get_neightbours(*position);
            if let Some(tile) = neighbours
                .iter()
                .filter(|t| t.pheromone.is_some())
                .find(|t| Pos::dist(*position, map.nest_pos) >= Pos::dist(t.pos, map.nest_pos))
            {
                *position = tile.pos;
            } else {
                if rand::thread_rng().gen::<f32>() <= 0.5 {
                    *position = Pos::new(position.x + dir_x, position.y);
                } else {
                    *position = Pos::new(position.x, position.y + dir_y);
                }
            }

            // drop ph
            map.drop_ph(PhType::FoodTrail, *position, config.ph_drop.val());

            if map.reached_nest(*position) {
                *state = Simulation::decide_ant_state(map, config);
                colony.food += 1;
            }
        }
    }

    fn follower_system(colony: &mut Colony, map: &mut Map, config: &Config) {
        for (id, speed, position, direction, state) in izip!(
            &colony.ants.id,
            &colony.ants.speed,
            &mut colony.ants.position,
            &mut colony.ants.direction,
            &mut colony.ants.state,
        )
        .filter(|(_id, _speed, _position, _direction, state)| **state == AntState::Follower)
        {
            let neighbours = map.get_neightbours(*position);
            let max_ph_tile = neighbours
                .iter()
                .filter(|t| t.pheromone.is_some())
                .filter(|t| Pos::dist(map.nest_pos, t.pos) >= Pos::dist(*position, map.nest_pos))
                .choose(&mut rand::thread_rng());
            // .max_by_key(|t| OrderedFloat(t.pheromone.unwrap().conc()));

            if let Some(tile) = max_ph_tile {
                *position = tile.pos;
            } else {
                *state = AntState::Seeker;
            }

            if let Some(tile) = neighbours.iter().find(|t| t.tile_type == TileType::Food) {
                *state = AntState::Returner;
                map.clear(tile.pos);
            }
        }
    }

    fn noob_system(colony: &mut Colony, map: &mut Map, config: &Config) {
        for (id, speed, position, direction, state) in izip!(
            &colony.ants.id,
            &colony.ants.speed,
            &mut colony.ants.position,
            &mut colony.ants.direction,
            &mut colony.ants.state,
        )
        .filter(|(_id, _speed, _position, _direction, state)| **state == AntState::Noob)
        {
            let dir_x = (map.nest_pos.x - position.x).signum();
            let dir_y = (map.nest_pos.y - position.y).signum();

            let neighbours = map.get_neightbours(*position);
            if let Some(tile) = neighbours
                .iter()
                .find(|t| Pos::dist(*position, map.nest_pos) >= Pos::dist(t.pos, map.nest_pos))
            {
                *position = tile.pos;
            } else {
                if rand::thread_rng().gen::<f32>() <= 0.5 {
                    *position = Pos::new(position.x + dir_x, position.y);
                } else {
                    *position = Pos::new(position.x, position.y + dir_y);
                }
            }

            if map.reached_nest(*position) {
                *state = Simulation::decide_ant_state(map, config);
            }
        }
    }

    fn seeker_system(colony: &mut Colony, map: &mut Map, config: &Config) {
        for (id, speed, position, direction, state, steps) in izip!(
            &colony.ants.id,
            &colony.ants.speed,
            &mut colony.ants.position,
            &mut colony.ants.direction,
            &mut colony.ants.state,
            &mut colony.ants.steps,
        )
        .filter(|(_id, _speed, _position, _direction, state, _steps)| **state == AntState::Seeker)
        {
            // add steps counter
            *steps += 1;

            let mut rng = rand::thread_rng();
            // rotate random
            if rng.gen_ratio(1, 5) {
                *direction = direction.random_turn(&mut rng);
            };

            let neighbours = map.get_neightbours(*position);

            if let Some(tile) = neighbours.iter().find(|t| t.tile_type == TileType::Food) {
                *state = AntState::Returner;
                map.clear(tile.pos);
                *steps = 0;
            } else if *steps > config.max_steps.val() {
                *state = AntState::Noob;
                *steps = 0;
            }

            // calculate new position
            let x_new = position.x + direction.vec().x * speed;
            let y_new = position.y + direction.vec().y * speed;
            let new_pos = Pos::new(
                x_new.clamp(0, *map.width() - 1),
                y_new.clamp(0, *map.height() - 1),
            );
            *position = new_pos;
        }
    }
}
