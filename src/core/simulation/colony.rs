use std::{collections::HashMap, ptr::hash};

use nalgebra::{Point2, Vector2};

use crate::{core::map::map::Pos, utils::AntDirection};

#[derive(Default)]
pub struct Colony {
    pub ants: AntCollection,
    pub food: i32,
}

impl Colony {
    pub fn add_ant(&mut self, speed: i32, position: Pos, state: AntState) {
        let direction: AntDirection = rand::random();
        self.ants.push(speed, position, direction, state);
    }
}
#[derive(Default, Clone)]
pub struct AntCollection {
    pub id: Vec<i32>,
    pub speed: Vec<i32>,
    pub position: Vec<Pos>,
    pub direction: Vec<AntDirection>,
    pub state: Vec<AntState>,
    pub travel: Vec<Vec<Pos>>,
    pub steps: Vec<i32>,
}

impl AntCollection {
    pub fn push(&mut self, speed: i32, position: Pos, direction: AntDirection, state: AntState) {
        self.id.push(self.id.len() as i32);
        self.speed.push(speed);
        self.position.push(position);
        self.direction.push(direction);
        self.state.push(state);
        self.travel.push(Vec::new());
        self.steps.push(0);
    }

    pub fn state_count(&self) -> HashMap<AntState, i32> {
        let seeker = self
            .state
            .iter()
            .filter(|s| **s == AntState::Seeker)
            .count();

        let returner = self
            .state
            .iter()
            .filter(|s| **s == AntState::Returner)
            .count();

        let noobs = self.state.iter().filter(|s| **s == AntState::Noob).count();

        let follower = self
            .state
            .iter()
            .filter(|s| **s == AntState::Follower)
            .count();

        let mut hash_map = HashMap::new();
        hash_map.insert(AntState::Seeker, seeker as i32);
        hash_map.insert(AntState::Returner, returner as i32);
        hash_map.insert(AntState::Noob, noobs as i32);
        hash_map.insert(AntState::Follower, follower as i32);

        return hash_map;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AntState {
    Seeker,
    Returner,
    Noob,
    Follower,
}
