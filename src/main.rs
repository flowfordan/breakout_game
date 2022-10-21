use macroquad::prelude::*;

const PLAYER_SIZE: Vec2 = Vec2::from_array([150.0, 40.0]);

struct Player {
    rect: Rect,
}

impl Player {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(
                screen_width() * 0.5 - PLAYER_SIZE.x*0.5,
                screen_height() - 100.0,
                PLAYER_SIZE.x,
                PLAYER_SIZE.y
            )
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, BLUE)
    }

    pub fn update(&mut self, dt: f32) {
        let mut x_move: f32 = 0.0;

        if is_key_down(KeyCode::Left) {
            x_move -= 1.0;
        }
        if is_key_down(KeyCode::Right) {
            x_move += 1.0;
        }
    }
}

#[macroquad::main("breakout")]
async fn main() {

    //PLAYER
    let player = Player::new();

    loop {
        clear_background(WHITE);

        player.draw();
        player.update(dt)

        next_frame().await
    }
}

//Player::new vs player.draw
//(&self)