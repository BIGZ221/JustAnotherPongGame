use console_engine::{pixel::pxl, ConsoleEngine};

use crate::{ball::Ball, vec::Vec2};

pub static PADDLE_SIZE: i32 = 6;

pub enum Direction {
    Up,
    Down,
}

#[derive(Debug, Clone, Copy)]
pub struct Player {
    paddle: Vec2,
    score: u32,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Player {
            paddle: Vec2::new(x, y),
            score: 0,
        }
    }

    pub fn draw(&self, engine: &mut ConsoleEngine, x_offset: i32, y_offset: i32) {
        engine.line(
            self.paddle.x.round() as i32 + x_offset,
            self.paddle.y.round() as i32 + y_offset,
            self.paddle.x.round() as i32 + x_offset,
            self.paddle.y.round() as i32 + y_offset + PADDLE_SIZE,
            pxl('█'),
        )
    }

    pub fn score(&mut self) {
        self.score += 1;
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }

    pub fn get_paddle(&self) -> Vec2 {
        self.paddle.clone()
    }

    pub fn move_paddle(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.paddle.y -= 1.0,
            Direction::Down => self.paddle.y += 1.0,
        }
    }

    pub fn will_collid_with(&self, ball: &Ball) -> bool {
        let ball_pos = ball.get_pos();
        approx_eq(self.paddle.x.trunc(), ball_pos.x.trunc(), 0.2)
            && self.paddle.y + PADDLE_SIZE as f32 >= ball_pos.y as f32
            && self.paddle.y <= ball_pos.y
    }
}

fn approx_eq(a: f32, b: f32, epsilon: f32) -> bool {
    (a - b).abs() <= epsilon
}
