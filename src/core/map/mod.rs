use core::fmt::Debug;

pub mod area;
pub mod map;

use serde::de::value::BoolDeserializer;

pub trait Renderer {
    fn render(&self, width: i64, height: i64) -> Vec<String>;

    fn render_throw(&self, width: i64, height: i64) -> Vec<String>;
}
