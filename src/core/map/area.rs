use meval::Expr;
use serde::{de::Error, Deserialize, Deserializer, Serialize};
use std::fmt::Debug;

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum AreaType {
    Field,
    Woods,
    Bushes,
    Bunker,
}

#[derive(Debug, Clone, Deserialize)]
pub enum AreaDirection {
    Above,
    Beneath,
}

#[derive(Deserialize)]
pub struct AreaBorder {
    #[serde(deserialize_with = "deserialize_function")]
    pub func: Box<dyn Fn(f64) -> f64>,
    pub direction: AreaDirection,
}

fn deserialize_function<'de, D>(data: D) -> Result<Box<dyn Fn(f64) -> f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let buf: String = String::deserialize(data)?;
    let expr: Expr = buf.parse().unwrap();
    let func = expr.bind("x");
    match func {
        Ok(f) => Ok(Box::new(f)),
        Err(_) => Err(serde::de::Error::custom("Error deserializing function")),
    }
}

impl AreaBorder {
    pub fn match_direction(&self, f_y: f64, y: f64) -> bool {
        match self.direction {
            AreaDirection::Above => f_y <= y,
            AreaDirection::Beneath => f_y >= y,
        }
    }

    pub fn ff(&self, y: f64) -> f64 {
        (self.func)(y)
    }

    pub fn in_area(&self, x: f64, y: f64) -> bool {
        let f_y = self.ff(x);
        self.match_direction(f_y, y)
    }

    pub fn in_area_scaled(&self, x: f64, y: f64, x_scale: f64, y_scale: f64) -> bool {
        let f_y = self.ff(x / x_scale) * y_scale;
        self.match_direction(f_y, y * y_scale)
    }
}

#[derive(Deserialize)]
pub struct Area {
    pub area_type: AreaType,
    pub borders: Vec<AreaBorder>,
}

impl Area {
    pub fn new(functions: Vec<(&'static str, AreaDirection)>, area_type: AreaType) -> Self {
        let mut borders: Vec<AreaBorder> = Vec::new();
        for (func_str, direction) in functions {
            let expr: Result<Expr, meval::Error> = func_str.parse();
            match expr {
                Ok(e) => match e.bind("x") {
                    Ok(f) => {
                        let border_area = AreaBorder {
                            func: Box::new(f),
                            direction,
                        };
                        borders.push(border_area);
                    }
                    Err(_) => (),
                },
                Err(_) => {}
            };
        }

        Self { area_type, borders }
    }

    pub fn inside(&self, x: f64, y: f64, x_scale: f64, y_scale: f64) -> bool {
        self.borders
            .iter()
            .all(|b| b.in_area_scaled(x, y, x_scale, y_scale))
    }
}

#[cfg(test)]
mod test {
    use meval::Expr;

    use super::{Area, AreaBorder, AreaDirection, AreaType};

    #[test]
    fn point_inside() {
        let expr: Expr = "-0.05*(x-12)^2+6".parse().unwrap();
        let func = expr.bind("x").unwrap();
        let border_1 = AreaBorder {
            func: Box::new(func),
            direction: super::AreaDirection::Beneath,
        };

        let expr: Expr = "0.05*(x-12)^2+1".parse().unwrap();
        let func = expr.bind("x").unwrap();
        let border_2 = AreaBorder {
            func: Box::new(func),
            direction: super::AreaDirection::Above,
        };

        assert!(border_1.in_area(12., 5.));
        assert!(border_2.in_area(12., 5.));

        assert!(!border_1.in_area(12., 10.));
        assert!(border_2.in_area(12., 10.));

        assert!(border_1.in_area(12., 0.));
        assert!(!border_2.in_area(12., 0.));
    }

    #[test]
    fn inside_area() {
        let area = Area::new(
            vec![
                ("-0.05*(x-12)^2+6", AreaDirection::Beneath),
                ("0.05*(x-12)^2+1", AreaDirection::Above),
            ],
            AreaType::Woods,
        );

        assert!(area.inside(12., 5., 1., 1.));
        assert!(!area.inside(12., 10., 1., 1.));
        assert!(!area.inside(12., 0., 1., 1.));
    }
}
