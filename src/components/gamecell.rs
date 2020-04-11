use bracket_lib::prelude::*;

#[derive(Clone, Debug)]
pub struct GameCell {
    point: Point,
    symbol: char,
    color: RGB,
}

impl GameCell {
    pub fn new(point: Point, symbol: char, color: RGB) -> Self {
        Self {
            point,
            symbol,
            color,
        }
    }

    // pub fn point(&self) -> Point {
    //     self.point
    // }
    pub fn x(&self) -> i32 {
        self.point.x
    }
    pub fn y(&self) -> i32 {
        self.point.y
    }
    pub fn symbol(&self) -> char {
        self.symbol
    }
    pub fn color(&self) -> RGB {
        self.color
    }
    /// Return a brightened version of the cell's color
    pub fn color_bright(&self) -> RGB {
        RGB::from_f32(self.color.r * 1.5, self.color.g * 1.5, self.color.b * 1.5)
    }
    /// Return a black background for the cell
    pub fn bg_color(&self) -> RGB {
        RGB::named(WHITE)
    }
}
