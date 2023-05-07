use macroquad::prelude::*;
mod entity;

// WINDOW CONST
const WINDOW: (f32, f32) = (600.0, 400.0);

//PADLE CONSTS
const PADLE_WIDTH: f32 = 20f32;

struct Game {
    player: entity::padle::Padle,
    enemy: entity::enemy::Enemy,
    ball: entity::ball::Ball,
    game_over: bool,
    end_game: bool,
    player_score: u32,
    enemy_score: u32,
}

impl Game {
    pub fn new() -> Self {
        Game {
            player: entity::padle::Padle::new(),
            enemy: entity::enemy::Enemy::new(),
            ball: entity::ball::Ball::new(),
            game_over: false,
            end_game: false,
            player_score: 0,
            enemy_score: 0,
        }
    }

    pub fn init(&mut self) {
        self.player = entity::padle::Padle::new();
        self.player.position.x = 10f32;
        self.player.position.y = 10f32;

        self.enemy = entity::enemy::Enemy::new();
        self.enemy.position.x = WINDOW.0 - PADLE_WIDTH - 10f32;
        self.enemy.position.y = 10f32;

        self.ball = entity::ball::Ball::new();

        self.game_over = false;
        self.end_game = false;
    }

    pub fn update(&mut self) {
        if !self.game_over {
            self.game_loop();
        } else {
            self.init();
        }
    }

    pub fn draw(&mut self) {
        self.player.draw();
        self.enemy.draw();
        self.ball.draw();
        let mut score = String::from(self.player_score.to_string());
        score.push_str("-");
        score.push_str(&self.enemy_score.to_string());

        draw_text(&score, WINDOW.0 / 2.0, 10.0, 20.0, WHITE);
    }

    fn game_loop(&mut self) {
        if is_key_down(KeyCode::Escape) {
            self.end_game = true;
            self.game_over = true;
        } else if self.ball.out_of_bound {
            self.game_over = true;
            if self.ball.position.x < 0.0 {
                self.enemy_score += 1;
            } else {
                self.player_score += 1;
            }
        }

        let dt: f32 = get_frame_time();

        if !self.ball.launched && is_key_down(KeyCode::Space) {
            self.ball.push();
        }

        self.player.update(dt);
        self.enemy.update(dt, &self.ball);
        self.ball.update(&mut self.player, &mut self.enemy, dt);
    }
}

#[macroquad::main("BasicShapes")]
async fn main() {
    request_new_screen_size(WINDOW.0, WINDOW.1);

    let mut game = Game::new();
    game.init();
    while !game.end_game {
        game.update();

        clear_background(BLACK);
        game.draw();
        next_frame().await
    }
}
