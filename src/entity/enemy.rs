use macroquad::{
    prelude::{Rect, WHITE},
    shapes::draw_rectangle,
};

use crate::entity::ball::Ball;
use crate::entity::vector::Vector;
use rand::prelude::*;

use super::ball::BALL_SIZE;

// WINDOW CONST
const WINDOW: (f32, f32) = (600.0, 400.0);
//PADLE CONSTS
const PADLE_SPEED: f32 = 450f32;
const PADLE_HEIGHT: f32 = 100f32;
const PADLE_WIDTH: f32 = 20f32;

const ENEMY_THRESHOLD: f32 = 0.25;

pub struct Enemy {
    pub position: Vector,
    pub velocity: Vector,
}

impl Enemy {
    pub fn new() -> Self {
        Enemy {
            position: Vector::new(),
            velocity: Vector::new(),
        }
    }

    pub fn update(&mut self, dt: f32, ball: &Ball) {
        let mut rng = rand::thread_rng();
        let random_number: f32 = rng.gen_range(0.0..1.0);
        if random_number > ENEMY_THRESHOLD && ball.launched {
            self.predict_trajectory(ball);
        } else {
            self.velocity.x = 0.0;
            self.velocity.y = 0.0;
        }

        // AVOID OUT OF BOUND
        if (self.position.y <= 0f32 && self.velocity.y == -1f32)
            || (self.position.y + PADLE_HEIGHT >= WINDOW.1 && self.velocity.y == 1f32)
        {
            self.velocity.y = 0f32;
        }

        self.position.y += self.velocity.y * dt * PADLE_SPEED;
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.position.x,
            self.position.y,
            PADLE_WIDTH,
            PADLE_HEIGHT,
            WHITE,
        );
    }

    pub fn to_rect(&mut self) -> Rect {
        Rect {
            x: self.position.x,
            y: self.position.y,
            w: PADLE_WIDTH,
            h: PADLE_HEIGHT,
        }
    }

    fn predict_trajectory(&mut self, ball: &Ball) {
        if ball.position.y + BALL_SIZE > self.position.y + PADLE_HEIGHT / 2.0 {
            self.velocity.y = 1.0;
        } else if ball.position.y + BALL_SIZE < self.position.y + PADLE_HEIGHT / 2.0 {
            self.velocity.y = -1.0;
        }
    }
}
