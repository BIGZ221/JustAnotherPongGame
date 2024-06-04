use std::f32::consts::{FRAC_PI_2, PI};

use console_engine::{pixel::pxl, ConsoleEngine};

use crate::{player::PADDLE_SIZE, vec::Vec2};

const MAX_BOUNCE_ANGLE_RADIANS: f32 = 75.0 * PI / 180.0;

#[derive(Debug, Clone, Copy)]
pub struct Ball {
    pos: Vec2,
    vel: Vec2,
}

impl Ball {
    pub fn new(x: f32, y: f32, vel: Vec2) -> Self {
        Ball {
            pos: Vec2::new(x, y),
            vel,
        }
    }

    pub fn tick(&mut self) {
        self.pos.add(&self.vel);
    }

    pub fn bounce(&mut self, paddle_start: Vec2) {
        let x_dir = self.vel.clone().x.signum(); // +1 if right -1 if left
        let paddle_center = paddle_start.y + PADDLE_SIZE as f32 / 2.0;
        let dist_from_center = self.pos.y - paddle_center;
        let dist_ratio: f32 = (dist_from_center as f32 / (PADDLE_SIZE as f32 / 2.0)).abs();
        let bounce_angle: f32 = dist_ratio.powf(1.5) * MAX_BOUNCE_ANGLE_RADIANS;
        let flipped_angle: f32 =
            FRAC_PI_2 * self.vel.x.signum() + bounce_angle * dist_from_center.signum(); // If moving left, go right and vice versa
        self.vel = Vec2::new(0.0, 1.0).rotate(flipped_angle);
        self.pos.x += -1.0 * x_dir; // Move it away from the paddle to avoid double bounce
        self.vel.y *= -1.0 * x_dir;
    }

    pub fn reverse_vertical(&mut self) {
        self.vel.y *= -1.0;
    }

    pub fn draw(&self, engine: &mut ConsoleEngine, x_offset: i32, y_offset: i32) {
        engine.set_pxl(
            self.pos.x.round() as i32 + x_offset,
            self.pos.y.round() as i32 + y_offset,
            pxl('■'),
        );
    }

    pub fn get_pos(&self) -> Vec2 {
        self.pos.clone()
    }
}
