use macroquad::prelude::*;

mod objects;
use crate::objects::player::Player;
use crate::objects::block::Block;
use crate::objects::ball::Ball;

const BLOCK_SIZE: Vec2 = Vec2::from_array([100.0, 40.0]);

pub enum GameState {
    Menu,
    Game,
    LevelCompleted,
    Dead,
}

pub fn draw_title_text(text: &str, font: Font) {
    let dims = measure_text(text, Some(font), 50u16, 1.0);
    draw_text_ex(
        text, 
        screen_width() * 0.5 - dims.width * 0.5, 
        screen_height() * 0.5 - dims.height * 0.5, 
        TextParams { font, font_size: 50u16, color: BLACK, ..Default::default() }
    )
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

fn reset_game() {
    
}

#[macroquad::main("breakout")]
async fn main() {
    let font = load_ttf_font("res/Roboto-Medium.ttf").await.unwrap();

    let mut game_state = GameState::Menu;
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
        clear_background(WHITE);

        match game_state {
            GameState::Menu => {
                draw_title_text("Press SPACE to start", font);
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Game;
                }
            },
            GameState::Game => {
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
                let is_last_ball = balls_len == 1;
                balls.retain(|ball| ball.rect.y < screen_height());
                let removed_balls = balls_len - balls.len();
                if removed_balls > 0 && is_last_ball {
                    player_lives -= 1;
                    if player_lives <= 0 {
                        game_state = GameState::Dead;
                    }
                }

                //remove block if block lives = 0
                if blocks.is_empty() {
                    game_state = GameState::LevelCompleted;
                }
                blocks.retain(|block| block.lives > 0);
                //score & lives render
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

                player.draw();
                for block in blocks.iter() {
                    block.draw();
                }
                for ball in balls.iter() {
                    ball.draw();
                }
            },
            GameState::LevelCompleted => {
                draw_title_text(&format!("You win! With {} score", score), font);
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Menu;
                }
            },
            GameState::Dead => {
                draw_title_text(&format!("You died! With {} score", score), font);
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Menu;
                }
            },
        }

        next_frame().await
    }
}
