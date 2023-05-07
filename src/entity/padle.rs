use macroquad::{
    prelude::{is_key_down, KeyCode, Rect, WHITE},
    shapes::draw_rectangle,
};

use crate::entity::vector::Vector;

// WINDOW CONST
const WINDOW: (f32, f32) = (600.0, 400.0);
//PADLE CONSTS
const PADLE_SPEED: f32 = 450f32;
const PADLE_HEIGHT: f32 = 100f32;
const PADLE_WIDTH: f32 = 20f32;

#[derive(Debug)]
pub struct Padle {
    pub position: Vector,
    pub velocity: Vector,
}

impl Padle {
    pub fn new() -> Self {
        Padle {
            position: Vector::new(),
            velocity: Vector::new(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        Padle::handle_key_event(self);

        // AVOID OUT OF BOUND
        if (self.position.y <= 0f32 && self.velocity.y == -1f32)
            || (self.position.y + PADLE_HEIGHT >= WINDOW.1 && self.velocity.y == 1f32)
        {
            self.velocity.y = 0f32;
        }

        self.position.y += self.velocity.y * dt * PADLE_SPEED;
    }

    fn handle_key_event(&mut self) {
        if is_key_down(KeyCode::Up) {
            self.velocity.y = -1f32;
        } else if is_key_down(KeyCode::Down) {
            self.velocity.y = 1f32;
        } else {
            self.velocity.y = 0f32;
        }
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
}
