use bracket_lib::prelude::*;

pub struct Room {
    rect: Rect,
}

impl Room {
    pub fn new(rect: Rect) -> Self {
        Self { rect }
    }

    pub fn rect(&self) -> &Rect {
        &self.rect
    }
}
