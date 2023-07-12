use crate::vec::Vec2;
use crate::{
    ball::Ball,
    player::{Direction, Player, PADDLE_SIZE},
};
use console_engine::pixel::pxl;
use console_engine::rect_style::BorderStyle;
use console_engine::{Color, ConsoleEngine};
use rand::{thread_rng, Rng};

#[derive(Debug, Clone, Copy)]
pub struct Pong {
    width: f32,
    players: (Player, Player),
    ball: Ball,
    playable_height: f32,
    header_offset: i32,
}

impl Pong {
    pub fn new(width: f32, height: f32) -> Self {
        let playable_height = height - 1.0;
        let paddle_center = playable_height / 2.0;
        let paddle_top = paddle_center - PADDLE_SIZE as f32 / 2.0;
        Pong {
            players: (
                Player::new(1.0, paddle_top),
                Player::new(width - 1.0, paddle_top),
            ),
            width,
            ball: Ball::new(width / 2.0, paddle_center, Vec2::new(1.0, 0.0)),
            playable_height,
            header_offset: (height - playable_height) as i32,
        }
    }

    pub fn tick(&mut self) {
        self.ball.tick();
        if self.players.0.will_collid_with(&self.ball) {
            self.ball.bounce(self.players.0.get_paddle());
        }
        if self.players.1.will_collid_with(&self.ball) {
            self.ball.bounce(self.players.1.get_paddle());
        }
        let ball_pos = self.ball.get_pos();
        if ball_pos.y <= 1.0 || ball_pos.y >= self.playable_height - 1.0 {
            self.ball.reverse_vertical();
        }
        if ball_pos.x <= 0.0 {
            self.players.1.score();
            self.spawn_ball();
        }
        if ball_pos.x >= self.width - 1.0 {
            self.players.0.score();
            self.spawn_ball();
        }
    }

    pub fn spawn_ball(&mut self) -> Self {
        let vel = Vec2::new(
            if thread_rng().gen_bool(0.5) {
                1.0
            } else {
                -1.0
            },
            0.0,
        );
        self.ball = Ball::new(self.width / 2.0, self.playable_height / 2.0, vel);
        *self
    }

    pub fn move_paddle_player_one(&mut self, direction: Direction) {
        if self.can_move(&self.players.0, &direction) {
            self.players.0.move_paddle(&direction);
        }
    }

    pub fn move_paddle_player_two(&mut self, direction: Direction) {
        if self.can_move(&self.players.1, &direction) {
            self.players.1.move_paddle(&direction);
        }
    }

    fn can_move(&self, player: &Player, direction: &Direction) -> bool {
        let paddle = player.get_paddle();
        match direction {
            Direction::Up => paddle.y > 1.0,
            Direction::Down => (paddle.y + PADDLE_SIZE as f32) < self.playable_height - 1.0,
        }
    }

    pub fn draw(&self, engine: &mut ConsoleEngine, x_offset: i32, y_offset: i32) {
        let player_one_score = format!("Player 1 Score: {}", self.players.0.get_score());
        let player_two_score = format!("Player 2 Score: {}", self.players.1.get_score());
        let screen_center = (self.width / 2.0).round() as i32;
        engine.set_pxl(screen_center, y_offset, pxl('|'));
        engine.print(
            screen_center / 2 - player_one_score.len() as i32 / 2,
            y_offset,
            player_one_score.as_str(),
        );
        engine.print(
            screen_center + screen_center / 2 - player_two_score.len() as i32 / 2,
            y_offset,
            player_two_score.as_str(),
        );
        engine.rect_border(
            x_offset,
            y_offset + self.header_offset,
            self.width.round() as i32 + x_offset,
            self.playable_height.round() as i32 + y_offset + self.header_offset,
            BorderStyle::new_solid().with_colors(Color::DarkGrey, Color::Black),
        );
        self.ball
            .draw(engine, x_offset, y_offset + self.header_offset);
        self.players
            .0
            .draw(engine, x_offset, y_offset + self.header_offset);
        self.players
            .1
            .draw(engine, x_offset, y_offset + self.header_offset);
    }
}
