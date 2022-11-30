use macroquad::prelude::*;

const BLOCK_SIZE: Vec2 = Vec2::from_array([100.0, 40.0]);

#[derive(PartialEq)]
pub enum BlockKind {
    Regular,
    SpawnBall,
}
pub struct Block {
    pub rect: Rect,
    pub lives: i32,
    pub block_kind: BlockKind,
}

impl Block {
    pub fn new(pos: Vec2, block_kind: BlockKind) -> Self {
        Self {
            rect: Rect::new(pos.x, pos.y, BLOCK_SIZE.x, BLOCK_SIZE.y),
            lives: 2,
            block_kind,
        }
    }
    pub fn draw(&self){
        let color = match self.block_kind {
            BlockKind::Regular => {
                match self.lives {
                    2 => RED,
                    _ => ORANGE,
                }
            },
            BlockKind::SpawnBall => GREEN,
        };
        // let color = match self.lives {
        //     2 => RED,
        //     _ => ORANGE,
        // };
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, color)
    }
}
