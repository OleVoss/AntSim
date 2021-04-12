use nalgebra::Point2;

use crate::core::map::{self, area::AreaType};
pub trait Renderer {
    fn render(&self, width: i64, height: i64) -> Vec<String>;
}
pub struct PrintRenderer {
    field: char,
    tee: char,
    basket: char,
    woods: char,
    bunker: char,
    bushes: char,
}

impl PrintRenderer {
    pub fn new(
        field: char,
        tee: char,
        basket: char,
        woods: char,
        bunker: char,
        bushes: char,
    ) -> Self {
        Self {
            field,
            tee,
            basket,
            woods,
            bunker,
            bushes,
        }
    }

    fn match_area_char(&self, area_type: AreaType) -> char {
        match area_type {
            AreaType::Field => self.field,
            AreaType::Woods => self.woods,
            AreaType::Bushes => self.bushes,
            AreaType::Bunker => self.bunker,
        }
    }

    fn process_string_layers(
        &self,
        map_layer: Vec<String>,
        stroke_layer: Vec<String>,
    ) -> Vec<String> {
        let mut final_string: Vec<String> = Vec::new();

        for (ml, sl) in map_layer.iter().zip(stroke_layer.iter()) {
            let mut row = String::new();
            for (mc, sc) in ml.chars().into_iter().zip(sl.chars().into_iter()) {
                // TODO: add player chars as priority
                if mc.is_whitespace() || sc == self.tee || sc == self.basket {
                    row.push(sc);
                } else {
                    row.push(mc);
                }
            }
            final_string.push(row);
        }

        return final_string;
    }

    fn render_throw(&self, width: i64, height: i64) -> Vec<String> {
        let mut rows: Vec<String> = Vec::new();

        // // scaling
        // let x_scale = width as f64 / *hole.width() as f64;
        // let y_scale = height as f64 / *hole.height() as f64;
        // if let Some(path) = path {
        //     let scaled_from = Point2::new(path.from.x * x_scale, path.from.y * y_scale);
        //     for y in (0..height).rev() {
        //         let mut row = String::new();
        //         for x in 0..width {
        //             if path.on_path_scaled(x as f64, y as f64, x_scale, y_scale) {
        //                 row.push('-');
        //             } else {
        //                 row.push(' ');
        //             }
        //         }
        //         rows.push(row);
        //     }
        // } else {
        //     rows.push("asödlkfashdlfksja".to_string());
        // }

        return rows;
    }

    fn render_map(&self, width: i64, height: i64) -> Vec<String> {
        let mut rows: Vec<String> = Vec::new();
        // scaling
        // let x_scale = width as f64 / *hole.width() as f64;
        // let y_scale = height as f64 / *hole.height() as f64;
        // let scaled_tee = Point2::new(hole.tee().x * x_scale, hole.tee().y * y_scale);
        // let scaled_basket = Point2::new(hole.basket().x * x_scale, hole.basket().y * y_scale);

        // // render hole
        // for y in (0..height).rev() {
        //     let mut row = String::new();
        //     for x in 0..width {
        //         let mut tile = ' ';
        //         if x == scaled_tee.x as i64 && y == scaled_tee.y as i64 {
        //             tile = self.tee;
        //         } else if x == scaled_basket.x as i64 && y == scaled_basket.y as i64 {
        //             tile = self.basket;
        //         }
        //         for area in hole.areas() {
        //             if area.inside(x as f64, y as f64, x_scale, y_scale) {
        //                 tile = self.match_area_char(area.area_type);
        //             }
        //         }
        //         row.push(tile);
        //     }
        //     rows.push(row);
        // }
        return rows;
    }
}

impl Renderer for PrintRenderer {
    fn render(&self, width: i64, height: i64) -> Vec<String> {
        let map_layer = self.render_map(width, height);
        let stroke_layer = self.render_throw(width, height);

        return self.process_string_layers(map_layer, stroke_layer);
    }
}
