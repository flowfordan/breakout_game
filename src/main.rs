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

struct Block {
    rect: Rect,
    lives: i32,
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


fn resolve_collision(a: &mut Rect, vel: &mut Vec2, b: &Rect) -> bool {
    //early exit
    let intersection = match a.intersect(*b){
        Some(intersection) => intersection,
        None => return false,
    };

    let a_center = a.center();
    let b_center = b.center();
    let to = b_center - a_center;
    let to_signum = to.signum();
    match intersection.w > intersection.h {
        true => {
            //y
            a.y -= to_signum.y * intersection.h;
            vel.y = -to_signum.y * vel.y.abs();
        }
        false => {
            //x
            a.x -= to_signum.x * intersection.w;
            vel.x = -to_signum.x * vel.x.abs();
        }
    }
    true
}

#[macroquad::main("breakout")]
async fn main() {
    let font = load_ttf_font("res/Roboto-Medium.ttf").await.unwrap();

    let mut score = 0;
    let mut player_lives = 3;

    //PLAYER
    let mut player = Player::new();
    //BLOCKS
    let mut blocks = Vec::new();
    //BALLS
    let mut balls = Vec::new();

    let (width, height) = (6, 6);
    let padding = 5.0;
    let total_block_size = BLOCK_SIZE + vec2(padding, padding);
    let mut board_start_pos = vec2((screen_width() - (total_block_size.x * width as f32)) * 0.5, 50.0);
    for i in 0..width * height {
        let block_x = (i % width) as f32 * total_block_size.x;
        let block_y = (i / width) as f32 * total_block_size.y;
        blocks.push(Block::new(board_start_pos + vec2(block_x, block_y)))
    }

    balls.push(Ball::new(vec2(screen_width() * 0.5, screen_height() * 0.5)));

    loop {
        if is_key_pressed(KeyCode::Space) {
            balls.push(Ball::new(vec2(screen_width() * 0.5, screen_height() * 0.5)));
        }
        //frame time
        player.update(get_frame_time());
        for ball in balls.iter_mut() {
            ball.update(get_frame_time());
        }
        for ball in balls.iter_mut() {
            resolve_collision(&mut ball.rect, & mut ball.vel, &mut player.rect);
            for block in blocks.iter_mut() {
                // println!("{}", resolve_collision(& mut ball.rect, &mut ball.vel, &block.rect));
                if resolve_collision(& mut ball.rect, &mut ball.vel, &block.rect) {
                    block.lives -= 1;
                    if block.lives <= 0 {
                        score += 10;  
                    }
                }
            }
        }

        //remove balls
        let balls_len = balls.len();
        balls.retain(|ball| ball.rect.y < screen_height());
        let removed_balls = balls_len - balls.len();
        if removed_balls > 0 {
            player_lives -= 1;
        }

        //remove block if lives = 0
        blocks.retain(|block| block.lives > 0);


        clear_background(WHITE);

        player.draw();
        for block in blocks.iter() {
            block.draw();
        }
        for ball in balls.iter() {
            ball.draw();
        }

        let score_text = format!("score: {}", score);
        let score_text_dim = measure_text(&score_text, Some(font), 30u16, 1.0);

        draw_text_ex(
            &score_text,
            screen_width() * 0.5 - score_text_dim.width*0.5,
            40.0,
            TextParams { font, font_size: 30u16, font_scale: 1.0, font_scale_aspect: 1.0, color: BLACK }
        );

        draw_text_ex(
            &format!("lives: {}", player_lives),
            30.0,
            40.0,
            TextParams { font, font_size: 30u16, font_scale: 1.0, font_scale_aspect: 1.0, color: BLACK }
        );

        next_frame().await
    }
}

//Player::new vs player.draw
//(&self)