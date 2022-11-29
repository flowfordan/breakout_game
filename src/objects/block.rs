use macroquad::prelude::*;

const BLOCK_SIZE: Vec2 = Vec2::from_array([100.0, 40.0]);

pub struct Block {
    pub rect: Rect,
    pub lives: i32,
}

impl Block {
    pub fn new(pos: Vec2) -> Self {
        Self {
            rect: Rect::new(pos.x, pos.y, BLOCK_SIZE.x, BLOCK_SIZE.y),
            lives: 2,
        }
    }
    pub fn draw(&self){
        let color = match self.lives {
            2 => RED,
            _ => ORANGE,
        };
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, color)
    }
}
