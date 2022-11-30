use macroquad::prelude::*;

pub const BALL_SIZE: f32 = 50.0;
const BALL_SPEED: f32 = 400.0;

pub struct Ball {
    pub rect: Rect,
    pub vel: Vec2,
}

impl Ball {
    pub fn new(pos: Vec2) -> Self {
        Self {
            rect: Rect::new(pos.x, pos.y, BALL_SIZE, BALL_SIZE),
            vel: vec2(rand::gen_range(-1f32, 1f32), 1f32).normalize(),
        }
    }

    pub fn update(&mut self, dt: f32){
        self.rect.x += self.vel.x * dt * BALL_SPEED; //update x coord of ball
        self.rect.y += self.vel.y * dt * BALL_SPEED; //update y coord of ball
        //collision with windows borders
        if self.rect.x < 0.0 {
            self.vel.x = 1.0;
        }
        if self.rect.x > screen_width() - self.rect.w {
            self.vel.x = -1.0;
        }
        if self.rect.y < 0.0 {
            self.vel.y = 1.0;
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, DARKGRAY);
    }
}
