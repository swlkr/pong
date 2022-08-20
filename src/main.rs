use macroquad::prelude::*;

const PLAYER_SIZE: Vec2 = const_vec2!([20.0, 100.0]);
const PLAYER_SPEED: f32 = 700.0;
const BALL_SIZE: Vec2 = const_vec2!([10.0, 10.0]);

fn draw_title_text(text: &str, font: Font, y: f32) {
    let dims = measure_text(text, Some(font), 50, 1.0);
    draw_text_ex(
        text,
        screen_width() * 0.5 - dims.width * 0.5,
        y,
        TextParams {
            font,
            font_size: 50,
            color: WHITE,
            ..Default::default()
        },
    )
}

struct Player {
    rect: Rect,
}

enum GameState {
    Menu,
    Game,
    Win,
    Lose,
}

impl Player {
    pub fn new(x: f32) -> Self {
        Self {
            rect: Rect::new(
                x,
                screen_height() / 2.0 - PLAYER_SIZE.y * 0.5,
                PLAYER_SIZE.x,
                PLAYER_SIZE.y,
            ),
        }
    }

    pub fn update(&mut self, dt: f32) {
        let y_move = match (
            is_key_down(KeyCode::Up) || is_key_down(KeyCode::K),
            is_key_down(KeyCode::Down) || is_key_down(KeyCode::J),
        ) {
            (true, false) => -1f32,
            (false, true) => 1f32,
            _ => 0f32,
        };
        self.rect.y += y_move * dt * PLAYER_SPEED;

        if self.rect.y < 0f32 {
            self.rect.y = 0f32;
        }
        if self.rect.y > screen_height() - self.rect.h {
            self.rect.y = screen_height() - self.rect.h;
        }
    }

    pub fn move_to(&mut self, dt: f32, target: f32) {
        if self.rect.y == target {
            self.rect.y = target;
            return;
        }
        if self.rect.y < target {
            self.rect.y += 1.0 * dt * PLAYER_SPEED;
        } else {
            self.rect.y += -1.0 * dt * PLAYER_SPEED;
        }
        if self.rect.y < 0f32 {
            self.rect.y = 0f32;
        }
        if self.rect.y > screen_height() - self.rect.h {
            self.rect.y = screen_height() - self.rect.h;
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, WHITE);
    }
}

struct Ball {
    rect: Rect,
    vel: Vec2,
}

impl Ball {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(
                screen_width() / 2.0 - BALL_SIZE.x * 0.5,
                screen_height() / 2.0 - BALL_SIZE.y * 0.5,
                BALL_SIZE.x,
                BALL_SIZE.y,
            ),
            vel: vec2(rand::gen_range(-1f32, 1f32), 1f32).normalize(),
        }
    }

    pub fn update(&mut self, dt: f32, speed: f32) {
        self.rect.x += self.vel.x * dt * speed;
        self.rect.y += self.vel.y * dt * speed;
        if self.rect.y < 0f32 {
            self.vel.y = 1f32;
        }
        if self.rect.y > screen_height() - self.rect.h * 0.5 {
            self.vel.y = -1f32;
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.w, WHITE);
    }
}

// aabb collision with positional correction
fn resolve_collision(a: &mut Rect, vel: &mut Vec2, b: &Rect) -> bool {
    // early exit
    let intersection = match a.intersect(*b) {
        Some(intersection) => intersection,
        None => return false,
    };
    let a_center = a.point() + a.size() * 0.5f32;
    let b_center = b.point() + b.size() * 0.5f32;
    let to = b_center - a_center;
    let to_signum = to.signum();
    match intersection.w > intersection.h {
        true => {
            // bounce on y
            a.y -= to_signum.y * intersection.h;
            vel.y = -to_signum.y * vel.y.abs();
        }
        false => {
            // bounce on x
            a.x -= to_signum.x * intersection.w;
            vel.x = -to_signum.x * vel.x.abs();
        }
    }
    return true;
}

#[macroquad::main("pong")]
async fn main() {
    let font = load_ttf_font("res/Silkscreen-Regular.ttf").await.unwrap();

    let mut game_state = GameState::Menu;
    let mut left_player = Player::new(10.0);
    let mut right_player = Player::new(screen_width() - PLAYER_SIZE.x - 10.0);
    let mut ball = Ball::new();
    let mut score = 0;
    let mut ball_speed = 200.0;

    loop {
        // lose
        if ball.rect.x < 0.0 {
            game_state = GameState::Lose;
        }

        // win
        if ball.rect.x > screen_width() {
            game_state = GameState::Win;
        }

        match game_state {
            GameState::Menu => {
                if is_key_down(KeyCode::Space) {
                    game_state = GameState::Game;
                }
                draw_title_text("Press SPACE to start", font, 100.0);
            }
            GameState::Game => {
                let score_text = format!("score: {}", score);
                let score_text_dim = measure_text(&score_text, Some(font), 30u16, 1.0);
                draw_text_ex(
                    &score_text,
                    screen_width() * 0.5f32 - score_text_dim.width * 0.5f32,
                    40.0,
                    TextParams {
                        font,
                        font_size: 30u16,
                        color: WHITE,
                        ..Default::default()
                    },
                );
                // move right_player if ball is going to the right
                if ball.vel.x > 0.0 {
                    right_player.move_to(get_frame_time(), ball.rect.y)
                }

                left_player.update(get_frame_time());
                ball.update(get_frame_time(), ball_speed);

                left_player.draw();
                right_player.draw();
                ball.draw();

                if resolve_collision(&mut ball.rect, &mut ball.vel, &left_player.rect) {
                    score += 1;
                    ball_speed += 50.0;
                }
                if resolve_collision(&mut ball.rect, &mut ball.vel, &right_player.rect) {
                    ball_speed += 50.0;
                }
            }
            GameState::Win => {
                if is_key_down(KeyCode::Space) {
                    game_state = GameState::Game;
                }
                draw_title_text("You win!", font, 100.0);
                let lose_text = "Press SPACE to play again";
                let lose_text_dim = measure_text(&lose_text, Some(font), 30u16, 1.0);
                draw_text_ex(
                    &lose_text,
                    screen_width() * 0.5f32 - lose_text_dim.width * 0.5f32,
                    screen_height() - 100.0,
                    TextParams {
                        font,
                        font_size: 30u16,
                        color: WHITE,
                        ..Default::default()
                    },
                );
            }
            GameState::Lose => {
                if is_key_down(KeyCode::Space) {
                    game_state = GameState::Game;
                }
                draw_title_text("You lost :(", font, 100.0);

                let lose_text = "Press SPACE to play again";
                let lose_text_dim = measure_text(&lose_text, Some(font), 30u16, 1.0);
                draw_text_ex(
                    &lose_text,
                    screen_width() * 0.5f32 - lose_text_dim.width * 0.5f32,
                    screen_height() - 100.0,
                    TextParams {
                        font,
                        font_size: 30u16,
                        color: WHITE,
                        ..Default::default()
                    },
                );
            }
        }

        next_frame().await;
    }
}
