use macroquad::prelude::{Rect, YELLOW};
use macroquad::shapes::draw_rectangle;
use rand::Rng;

use crate::entity::enemy::Enemy;
use crate::entity::padle::Padle;
use crate::entity::vector::Vector;

use super::utils;

//BALL CONSTS
pub const BALL_SIZE: f32 = 15f32;
const BALL_SPEED: f32 = 500f32;
const BALL_THRESHOLD: f32 = 0.4;

// WINDOW CONST
const WINDOW: (f32, f32) = (600.0, 400.0);

#[derive(Debug)]
pub struct Ball {
    pub position: Vector,
    pub velocity: Vector,
    pub out_of_bound: bool,
    pub launched: bool,
}

impl Ball {
    pub fn new() -> Self {
        let mut ball = Ball {
            position: Vector::new(),
            velocity: Vector::new(),
            out_of_bound: false,
            launched: false,
        };

        ball.position.x = WINDOW.0 / 2.0;
        ball.position.y = WINDOW.1 / 2.0;
        return ball;
    }

    pub fn push(&mut self) {
        let mut rng = rand::thread_rng();
        let random_x: f32 = rng.gen_range(0.0..1.0);
        let random_y: f32 = rng.gen_range(0.0..1.0);

        if random_x >= BALL_THRESHOLD {
            self.velocity.x = 1.0;
        } else {
            self.velocity.x = -1.0;
        }

        if random_y >= BALL_THRESHOLD {
            self.velocity.y = 1.0;
        } else {
            self.velocity.y = -1.0;
        }
        self.launched = true;
    }

    pub fn update(&mut self, player: &mut Padle, enemy: &mut Enemy, dt: f32) {
        let player_rect = player.to_rect();
        let enemy_rect = enemy.to_rect();

        let ball_rect = &self.to_rect();
        let side_player = utils::collides(&player_rect, ball_rect);
        let side_enemy = utils::collides(&enemy_rect, ball_rect);

        match side_player {
            utils::Side::TOP => self.velocity.y = -1.0,
            utils::Side::BOTTOM => self.velocity.y = 1.0,
            utils::Side::RIGHT => self.velocity.x = 1.0,
            utils::Side::LEFT => (),
            utils::Side::NONE => (),
        }

        match side_enemy {
            utils::Side::TOP => self.velocity.y = -1.0,
            utils::Side::BOTTOM => self.velocity.y = 1.0,
            utils::Side::LEFT => self.velocity.x = -1.0,
            utils::Side::RIGHT => (),
            utils::Side::NONE => (),
        }

        if self.position.y <= 0.0 {
            self.velocity.y = 1.0;
        } else if self.position.y + BALL_SIZE >= WINDOW.1 {
            self.velocity.y = -1.0;
        }

        self.position.x += self.velocity.x * dt * BALL_SPEED;
        self.position.y += self.velocity.y * dt * BALL_SPEED;

        // ALLOWS US TO MANAGE THE GAME OVER
        self.out_of_bound = self.position.x + BALL_SIZE <= 0.0 || self.position.x >= WINDOW.0;
    }

    pub fn draw(&mut self) {
        draw_rectangle(
            self.position.x,
            self.position.y,
            BALL_SIZE,
            BALL_SIZE,
            YELLOW,
        );
    }

    fn to_rect(&self) -> Rect {
        Rect::new(self.position.x, self.position.y, BALL_SIZE, BALL_SIZE)
    }
}
