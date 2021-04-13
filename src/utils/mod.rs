use nalgebra::Vector2;
use rand::{distributions::Standard, prelude::Distribution, Rng};
use std::{f64::consts::PI, path::Ancestors};

pub mod func;
pub mod renderer;

#[derive(Clone)]
pub enum AntDirection {
    Up,
    Down,
    Right,
    Left,
}

impl AntDirection {
    pub fn vec(&self) -> Vector2<i32> {
        match *self {
            AntDirection::Up => Vector2::new(0, 1),
            AntDirection::Down => Vector2::new(0, -1),
            AntDirection::Right => Vector2::new(1, 0),
            AntDirection::Left => Vector2::new(-1, 0),
        }
    }
}

impl Distribution<AntDirection> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> AntDirection {
        match rng.gen_range(0..4) {
            0 => AntDirection::Up,
            1 => AntDirection::Down,
            2 => AntDirection::Left,
            _ => AntDirection::Right,
        }
    }
}
