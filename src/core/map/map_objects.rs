use super::area::AreaType;
use nalgebra::Point2;
use serde::Deserialize;

pub trait MapObj {
    fn on_obj(&self, point: Point2<f64>) -> bool;
}

#[derive(Deserialize)]
pub struct Circle {
    pub radius: f64,
    pub position: Point2<f64>,
    pub area_type: AreaType,
}

impl MapObj for Circle {
    fn on_obj(&self, point: Point2<f64>) -> bool {
        return nalgebra::distance(&self.position, &point) <= self.radius;
    }
}
