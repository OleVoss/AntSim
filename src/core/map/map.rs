use crate::config::ANTHILL_RADIUS;
use getset::{Getters, MutGetters, Setters};
use rand::Rng;
use serde::{Deserialize, Serialize};

use super::{area::Area, map_objects::Circle};

#[derive(Getters, MutGetters, Setters, Deserialize, Default)]
pub struct Map {
    #[getset(get = "pub")]
    name: String,
    #[getset(get = "pub")]
    food: Vec<Area>,
    #[getset(get = "pub")]
    width: i32,
    #[getset(get = "pub")]
    height: i32,
    pub tile_matrix: Vec<Vec<Tile>>,
}

impl Map {
    pub fn new(name: &str, width: i32, height: i32) -> Self {
        let tile_matrix = Map::init_map(width, height);
        Self {
            name: String::from(name),
            food: Vec::new(),
            width,
            height,
            tile_matrix,
        }
    }

    fn init_map(width: i32, height: i32) -> Vec<Vec<Tile>> {
        let mut tile_matrix: Vec<Vec<Tile>> =
            vec![vec![Tile::empty(); width as usize]; height as usize];
        let x_offset: i32 = (width as f32 / 2.) as i32;
        let y_offset: i32 = (height as f32 / 2.) as i32;

        // anthill
        for y in 0..=2 {
            for x in 0..=4 {
                tile_matrix[(y + y_offset) as usize][(x + x_offset) as usize] = Tile::nest();
            }
        }

        // random food; TEMPORARY
        let food_piles = 7;
        for i in 0..food_piles {
            let mut rng = rand::thread_rng();

            let food_block_x = 9;
            let food_block_y = (food_block_x as f32 * 0.3) as i32;

            let rand_x = rng.gen_range(0..width - food_block_x);
            let rand_y = rng.gen_range(0..height - food_block_y);
            for y in rand_y..rand_y + food_block_y {
                for x in rand_x..rand_x + food_block_x {
                    tile_matrix[(y) as usize][(x) as usize] = Tile::food();
                }
            }
        }

        return tile_matrix;
    }

    pub fn min_height(&self) -> i32 {
        -self.height / 2
    }

    pub fn max_height(&self) -> i32 {
        self.height / 2
    }

    pub fn min_width(&self) -> i32 {
        -self.width / 2
    }

    pub fn max_width(&self) -> i32 {
        self.width / 2
    }
}

#[derive(Deserialize, Clone)]
pub struct Tile {
    pub tile_type: TileType,
    pub pheromone: Option<Pheromone>,
}

impl Tile {
    pub fn empty() -> Self {
        Self {
            tile_type: TileType::Empty,
            pheromone: None,
        }
    }

    pub fn nest() -> Self {
        Self {
            tile_type: TileType::Nest,
            pheromone: None,
        }
    }

    pub fn border() -> Self {
        Self {
            tile_type: TileType::Border,
            pheromone: None,
        }
    }

    pub fn food() -> Self {
        Self {
            tile_type: TileType::Food,
            pheromone: None,
        }
    }

    pub fn char(&self) -> char {
        self.tile_type.char()
    }
}

#[derive(Deserialize, Clone)]
pub enum TileType {
    Border,
    Empty,
    Food,
    Nest,
}

impl TileType {
    pub fn char(&self) -> char {
        match &self {
            TileType::Border => ' ',
            TileType::Empty => ' ',
            TileType::Food => '@',
            TileType::Nest => 'H',
        }
    }
}

#[derive(Deserialize, Clone)]
pub struct Pheromone {
    pub ph_type: PhType,
    pub concentration: f32,
}

impl Pheromone {
    pub fn expl() -> Self {
        Self {
            ph_type: PhType::Exploration,
            concentration: 0.0,
        }
    }

    pub fn food() -> Self {
        Self {
            ph_type: PhType::FoodTrail,
            concentration: 0.0,
        }
    }
}

#[derive(Deserialize, Clone)]
pub enum PhType {
    Exploration,
    FoodTrail,
}
