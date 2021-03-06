use itertools::izip;
use nalgebra::Point2;
use tui::style::Style;

use crate::{
    config::ConfigVar,
    core::{
        map::{
            self,
            area::AreaType,
            map::{Map, Pos},
        },
        simulation::colony::AntCollection,
    },
    style::Theme,
};
pub trait Renderer {
    fn render(&self, map: &Map, ants: AntCollection, width: i32, height: i32) -> Vec<String>;
}
pub struct PrintRenderer {
    anthill: char,
    food: char,
    obstacle: char,
    ant: char,
}

impl PrintRenderer {
    pub fn new(anthill: char, food: char, obstacle: char, ant: char) -> Self {
        Self {
            anthill,
            food,
            obstacle,
            ant,
        }
    }

    fn match_area_char(&self, area_type: AreaType) -> char {
        match area_type {
            AreaType::Anthill => self.anthill,
            AreaType::Food => self.food,
            AreaType::Obstacle => self.obstacle,
        }
    }

    fn process_string_layers(&self, map_layer: Vec<String>, ant_layer: Vec<String>) -> Vec<String> {
        let mut final_string: Vec<String> = Vec::new();

        for (ml, al) in map_layer.iter().zip(ant_layer.iter()) {
            let mut row = String::new();
            for (mc, ac) in ml.chars().into_iter().zip(al.chars().into_iter()) {
                if ac.is_whitespace() {
                    row.push(mc);
                } else {
                    row.push(ac);
                }
            }
            final_string.push(row);
        }

        return final_string;
    }

    fn render_map(&self, map: &Map, width: i32, height: i32) -> Vec<String> {
        let mut tile_matrix = vec![vec![' '; width as usize]; height as usize];
        let mut rows: Vec<String> = Vec::new();

        for row in &map.tile_matrix {
            for tile in row {
                if pos_inside_area(&tile.pos, width, height) {
                    tile_matrix[tile.pos.y as usize][tile.pos.x as usize] = match tile.tile_type {
                        map::map::TileType::Border => tile.char(),
                        map::map::TileType::Empty => {
                            if let Some(ph) = &tile.pheromone {
                                Theme::grey_scale(ph.conc())
                            } else {
                                tile.char()
                            }
                        }
                        map::map::TileType::Food => tile.char(),
                        map::map::TileType::Nest => tile.char(),
                    };
                }
            }
        }

        for row in tile_matrix {
            rows.push(row.into_iter().collect());
        }

        return rows;
    }

    fn render_ants(&self, ants: AntCollection, width: i32, height: i32) -> Vec<String> {
        let mut tile_matrix = vec![vec![' '; width as usize]; height as usize];
        let mut rows: Vec<String> = Vec::new();

        for (id, (speed, position)) in ants
            .id
            .iter()
            .zip(ants.speed.iter().zip(ants.position.iter()))
        {
            if pos_inside_area(position, width, height) {
                let x = position.x as usize;
                let y = position.y as usize;
                tile_matrix[y][x] = self.ant;
            }
        }

        for row in tile_matrix {
            rows.push(row.into_iter().collect());
        }

        return rows;
    }
}

fn pos_inside_area(pos: &Pos, width: i32, height: i32) -> bool {
    return (pos.x < width - 1 && pos.x >= 0) && (pos.y < height - 1 && pos.y >= 0);
}

#[test]
fn test_pos_inside_area() {
    let p1 = Pos::new(0, -50);
    let p2 = Pos::new(0, -150);
    assert!(pos_inside_area(&p1, 200, 200));
    assert!(!pos_inside_area(&p2, 200, 200));
}

impl Renderer for PrintRenderer {
    fn render(&self, map: &Map, ants: AntCollection, width: i32, height: i32) -> Vec<String> {
        let x_scale = width as f64 / *map.width() as f64;
        let y_scale = height as f64 / *map.height() as f64;

        let map_layer = self.render_map(map, width, height);
        let ant_layer = self.render_ants(ants, width, height);

        let final_string = self.process_string_layers(map_layer, ant_layer);

        return final_string;
    }
}
