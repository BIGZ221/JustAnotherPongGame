mod ball;
mod player;
mod pong;
mod vec;

use player::Direction;
use pong::Pong;

use console_engine::{ConsoleEngine, KeyCode};

fn main() {
    let mut engine = ConsoleEngine::init_fill_require(50, 30, 20).unwrap();
    let width: i32 = engine.get_width().try_into().unwrap();
    let height: i32 = engine.get_height().try_into().unwrap();
    let mut pong = Pong::new(width as f32 - 1.0, height as f32 - 1.0);

    loop {
        engine.wait_frame();
        engine.clear_screen();
        pong.tick();
        if engine.is_key_held(KeyCode::Char('w')) || engine.is_key_pressed(KeyCode::Char('w')) {
            pong.move_paddle_player_one(Direction::Up);
        }
        if engine.is_key_held(KeyCode::Char('s')) || engine.is_key_pressed(KeyCode::Char('s')) {
            pong.move_paddle_player_one(Direction::Down);
        }
        if engine.is_key_held(KeyCode::Up) || engine.is_key_pressed(KeyCode::Up) {
            pong.move_paddle_player_two(Direction::Up);
        }
        if engine.is_key_held(KeyCode::Down) || engine.is_key_pressed(KeyCode::Down) {
            pong.move_paddle_player_two(Direction::Down);
        }
        pong.draw(&mut engine, 0, 0);

        if engine.is_key_pressed(KeyCode::Char('q')) || engine.is_key_pressed(KeyCode::Esc) {
            break;
        }
        engine.draw();
    }
}
