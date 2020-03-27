use bracket_lib::prelude::*;

#[derive(Clone, Debug)]
pub struct GameCell {
    point: Point,
    symbol: char,
    color: RGB,
    selected: bool,
}

impl GameCell {
    pub fn new(x: i32, y: i32, symbol: char, color: RGB) -> Self {
        Self {
            point: Point::new(x, y),
            symbol,
            color,
            selected: false,
        }
    }

    /// Toggle the selected status of the cell
    pub fn select(&mut self) {
        self.selected = !self.selected;
    }
    /// Deselect the cell
    pub fn deselect(&mut self) {
        self.selected = false
    }

    pub fn point(&self) -> Point {
        self.point
    }
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
    /// Return a black background for the cell, but black if selected
    pub fn bg_color(&self) -> RGB {
        if self.selected {
            RGB::from_u8(255, 255, 255)
        } else {
            RGB::new()
        }
    }
    pub fn selected(&self) -> bool {
        self.selected
    }
}
