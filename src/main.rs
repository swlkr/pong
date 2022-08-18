use ::rand::Rng;
use macroquad::prelude::*;

const PLAYER_SIZE: Vec2 = const_vec2!([20.0, 100.0]);
const PLAYER_SPEED: f32 = 700.0;
const BALL_SIZE: Vec2 = const_vec2!([10.0, 10.0]);
const BALL_SPEED: f32 = 200.0;

struct Player {
    rect: Rect
}

impl Player {
    pub fn new(x: f32) -> Self {
        Self {
            rect: Rect::new(
                x,
                screen_height() / 2.0 - PLAYER_SIZE.y * 0.5,
                PLAYER_SIZE.x,
                PLAYER_SIZE.y
            )
        }
    }

    pub fn update(&mut self, dt: f32) {
        let y_move = match (is_key_down(KeyCode::Up) || is_key_down(KeyCode::K), is_key_down(KeyCode::Down) || is_key_down(KeyCode::J)) {
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

    pub fn update_right(&mut self, dt: f32, target: f32) {
        if self.rect.y != target {
            if target < screen_height() / 2.0 {
                self.rect.y += -1.0 * dt * PLAYER_SPEED;
            } else {
                self.rect.y += 1.0 * dt * PLAYER_SPEED;
            }
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
    vel: Vec2
}

impl Ball {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(
                screen_width() / 2.0 - BALL_SIZE.x * 0.5,
                screen_height() / 2.0 - BALL_SIZE.y * 0.5,
                BALL_SIZE.x,
                BALL_SIZE.y
            ),
            vel: vec2(rand::gen_range(-1f32, 1f32), 1f32).normalize()
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.rect.x += self.vel.x * dt * BALL_SPEED;
        self.rect.y += self.vel.y * dt * BALL_SPEED;
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
    return true
}

#[macroquad::main("pong")]
async fn main() {
    let mut left_player = Player::new(10.0);
    let mut right_player = Player::new(screen_width() - PLAYER_SIZE.x - 10.0);
    let mut ball = Ball::new();
    let mut target: f32 = 0.0;
    let mut collision: bool = false;
    let mut rng = ::rand::thread_rng();

    loop {
        // set target if ball changed directions
        // if there was collision with the left player
        if collision == true {
            target = rng.gen_range(0.0..screen_height());
        }

        left_player.update(get_frame_time());
        right_player.update_right(get_frame_time(), target);
        ball.update(get_frame_time());

        left_player.draw();
        right_player.draw();
        ball.draw();

        collision = resolve_collision(&mut ball.rect, &mut ball.vel, &left_player.rect);
        resolve_collision(&mut ball.rect, &mut ball.vel, &right_player.rect);

        next_frame().await;
    }
}
