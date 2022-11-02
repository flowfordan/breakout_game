use macroquad::prelude::*;

const PLAYER_SIZE: Vec2 = Vec2::from_array([150.0, 40.0]);
const PLAYER_SPEED: f32 = 700.0;
const BLOCK_SIZE: Vec2 = Vec2::from_array([100.0, 40.0]);
const BALL_SIZE: f32 = 50.0;
const BALL_SPEED: f32 = 400.0;

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

    //movement
    pub fn update(&mut self, dt: f32) {
        let mut x_move = match (is_key_down(KeyCode::Left), is_key_down(KeyCode::Right)) {
            (true, false) => -1.0,
            (false, true) => 1.0,
            _ => 0.0,
        };

        self.rect.x += x_move * dt * PLAYER_SPEED;

        if self.rect.x < 0.0 {
            self.rect.x = 0.0;
        }
        if self.rect.x > screen_width() - self.rect.w {
            self.rect.x = screen_width() - self.rect.w;
        }


    }
}

pub struct Ball {
    rect: Rect,
    vel: Vec2,
}

impl Ball {
    pub fn new(pos: Vec2) -> Self {
        Self {
            rect: Rect::new(pos.x, pos.y, BALL_SIZE, BALL_SIZE),
            vel: vec2(rand::gen_range(-1f32, 1f32), 1f32).normalize(),
        }
    }

    pub fn update(&mut self, dt: f32){
        
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, DARKGRAY);
    }
}

struct Block {
    rect: Rect,
}

impl Block {
    pub fn new(pos: Vec2) -> Self {
        Self {
            rect: Rect::new(pos.x, pos.y, BLOCK_SIZE.x, BLOCK_SIZE.y)
        }
    }
    pub fn draw(&self){
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, BLUE)
    }
}


#[macroquad::main("breakout")]
async fn main() {

    //PLAYER
    let mut player = Player::new();
    //BLOCKS
    let mut blocks = Vec::new();

    let (width, height) = (6, 6);
    let padding = 5.0;
    let total_block_size = BLOCK_SIZE + vec2(padding, padding);
    let mut board_start_pos = vec2((screen_width() - (total_block_size.x * width as f32)) * 0.5, 50.0);
    for i in 0..width * height {
        let block_x = (i % width) as f32 * total_block_size.x;
        let block_y = (i / width) as f32 * total_block_size.y;
        blocks.push(Block::new(board_start_pos + vec2(block_x, block_y)))
    }

    loop {
        //frame time
        player.update(get_frame_time());
        clear_background(WHITE);

        player.draw();
        for block in blocks.iter() {
            block.draw();
        }

        // board_start_pos = vec2((screen_width() - (total_block_size.x * width as f32)) * 0.5, 50.0);
        
        // println!("{}", board_start_pos);

        next_frame().await
    }
}

//Player::new vs player.draw
//(&self)