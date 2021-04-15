use getset::{Getters, MutGetters, Setters};
use rand::Rng;
use serde::{Deserialize, Serialize};

use super::area::Area;

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
    pub nest_pos: Pos,
}

impl Map {
    pub fn new(name: &str, width: i32, height: i32) -> Self {
        let tile_matrix = Vec::new();
        let mut map = Self {
            name: String::from(name),
            food: Vec::new(),
            width,
            height,
            tile_matrix,
            nest_pos: Pos::new(width / 2, height / 2),
        };
        map.tile_matrix = map.init_map();
        return map;
    }

    fn init_map(&self) -> Vec<Vec<Tile>> {
        let mut tile_matrix: Vec<Vec<Tile>> = Vec::new();
        for y in 0..self.height {
            let mut row: Vec<Tile> = Vec::new();
            for x in 0..self.width {
                row.push(Tile::empty(x, y));
            }
            tile_matrix.push(row);
        }

        // anthill
        tile_matrix[self.nest_pos.y as usize][self.nest_pos.x as usize] = Tile::nest(self.nest_pos);
        // for y in self.nest_pos.y - ANTHILL_HEIGHT / 2..self.nest_pos.y + ANTHILL_HEIGHT / 2 {
        //     for x in self.nest_pos.x - ANTHILL_WIDTH / 2..self.nest_pos.x + ANTHILL_WIDTH / 2 {
        //         tile_matrix[(y) as usize][(x) as usize] = Tile::nest(x, y);
        //     }
        // }

        // random food; TEMPORARY
        let mut food_piles = 0;
        while food_piles < 8 {
            let mut rng = rand::thread_rng();

            let food_block_x = 20;
            let food_block_y = (food_block_x as f32 * 0.3) as i32;

            let rand_x = rng.gen_range(0..self.width - food_block_x);
            let rand_y = rng.gen_range(0..self.height - food_block_y);

            if (rand_x < self.nest_pos.x - 10 || rand_x > self.nest_pos.x + 10)
                && (rand_y < self.nest_pos.y - 10 || rand_y > self.nest_pos.y + 10)
            {
                for y in rand_y..rand_y + food_block_y {
                    for x in rand_x..rand_x + food_block_x {
                        tile_matrix[y as usize][x as usize] = Tile::food(x, y);
                    }
                }
                food_piles += 1;
            }
        }

        return tile_matrix;
    }

    pub fn get(&self, pos: Pos) -> &Tile {
        return &self.tile_matrix[pos.y as usize][pos.x as usize];
    }

    pub fn get_xy(&self, x: i32, y: i32) -> &Tile {
        let pos = Pos::new(x, y);
        return self.get(pos);
    }

    pub fn get_mut(&mut self, pos: Pos) -> &mut Tile {
        return &mut self.tile_matrix[pos.y as usize][pos.x as usize];
    }

    pub fn clear(&mut self, pos: Pos) {
        self.get_mut(pos).tile_type = TileType::Empty;
    }

    pub fn drop_ph(&mut self, ph_type: PhType, pos: Pos, amount: i32) {
        if let Some(ph) = &mut self.get_mut(pos).pheromone {
            ph.concentration += amount;
        } else {
            self.get_mut(pos).pheromone = Some(Pheromone::food(amount));
        }
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

    pub fn get_neightbours(&self, pos: Pos) -> Vec<Tile> {
        let mut neighbours: Vec<Tile> = Vec::new();

        for y in self.clamp_height(pos.y - 1)..=self.clamp_height(pos.y + 1) {
            for x in self.clamp_width(pos.x - 1)..=self.clamp_width(pos.x + 1) {
                if !((x == pos.x) && (y == pos.y)) {
                    neighbours.push(*self.get_xy(x, y));
                }
            }
        }

        return neighbours;
    }

    pub fn reached_nest(&self, pos: Pos) -> bool {
        let neighbours = self.get_neightbours(pos);
        if neighbours.into_iter().any(|nt| nt.pos == self.nest_pos) {
            return true;
        } else {
            return false;
        }
    }

    pub fn nest_ph(&self) -> f32 {
        todo!();
    }

    fn clamp_height(&self, num: i32) -> i32 {
        num.clamp(0, self.height - 1)
    }

    fn clamp_width(&self, num: i32) -> i32 {
        num.clamp(0, self.width - 1)
    }
}

#[derive(Deserialize, Clone, Copy, Default, PartialEq, Eq)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn dist(pos1: Pos, pos2: Pos) -> f32 {
        let a: f32 = (pos1.x - pos2.x).pow(2) as f32;
        let b: f32 = (pos1.y - pos2.y).pow(2) as f32;
        let c: f32 = (a + b).sqrt();
        return c;
    }
}

#[derive(Deserialize, Clone, Copy)]
pub struct Tile {
    pub tile_type: TileType,
    pub pheromone: Option<Pheromone>,
    pub pos: Pos,
}

impl Tile {
    pub fn empty(x: i32, y: i32) -> Self {
        Self {
            tile_type: TileType::Empty,
            pheromone: None,
            pos: Pos::new(x, y),
        }
    }

    pub fn nest(pos: Pos) -> Self {
        Self {
            tile_type: TileType::Nest,
            pheromone: None,
            pos,
        }
    }

    pub fn border(x: i32, y: i32) -> Self {
        Self {
            tile_type: TileType::Border,
            pheromone: None,
            pos: Pos::new(x, y),
        }
    }

    pub fn food(x: i32, y: i32) -> Self {
        Self {
            tile_type: TileType::Food,
            pheromone: None,
            pos: Pos::new(x, y),
        }
    }

    pub fn char(&self) -> char {
        self.tile_type.char()
    }

    pub fn evaporate(&mut self, rate: i32) {
        match &mut self.pheromone {
            Some(ph) => {
                let mut conc: f32 = ph.conc() as f32;
                conc = conc * (1. - (rate as f32 / 100.));
                if conc < 30. {
                    self.pheromone = None;
                } else {
                    ph.concentration = conc as i32;
                }
            }
            None => {}
        }
    }
}

#[derive(Deserialize, Clone, Copy, PartialEq, Eq)]
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

#[derive(Deserialize, Clone, Copy)]
pub struct Pheromone {
    pub ph_type: PhType,
    pub concentration: i32,
}

impl Pheromone {
    pub fn expl(amount: i32) -> Self {
        Self {
            ph_type: PhType::Exploration,
            concentration: amount,
        }
    }

    pub fn food(amount: i32) -> Self {
        Self {
            ph_type: PhType::FoodTrail,
            concentration: amount,
        }
    }

    pub fn ph_type(&self) -> PhType {
        return self.ph_type.clone();
    }

    pub fn conc(&self) -> i32 {
        return self.concentration;
    }
}

#[derive(Deserialize, Clone, Copy)]
pub enum PhType {
    Exploration,
    FoodTrail,
}
