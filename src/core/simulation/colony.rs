use nalgebra::{Point2, Vector2};

use crate::utils::AntDirection;

#[derive(Default)]
pub struct Colony {
    pub ants: AntCollection,
}

impl Colony {
    pub fn add_ant(&mut self, speed: i32, position: Point2<i32>) {
        let direction: AntDirection = rand::random();
        self.ants.push(speed, position, direction);
    }
}
#[derive(Default, Clone)]
pub struct AntCollection {
    pub id: Vec<i32>,
    pub speed: Vec<i32>,
    pub position: Vec<Point2<i32>>,
    pub direction: Vec<AntDirection>,
    pub state: Vec<AntState>,
}

impl AntCollection {
    pub fn push(&mut self, speed: i32, position: Point2<i32>, direction: AntDirection) {
        self.id.push(self.id.len() as i32);
        self.speed.push(speed);
        self.position.push(position);
        self.direction.push(direction);
        self.state.push(AntState::Seeker);
    }
}

#[derive(Debug, Clone)]
pub enum AntState {
    Seeker,
    Returner,
}
