extern crate rand;
use rand::{thread_rng, Rng};

use macroquad::prelude::*;

const XVEL: f32 = 3.5;
const XACCEL: f32 = 0.05;
const PLATFORM_HEIGHT: f32 = 10.0;
const PLATFORM_WIDTH: f32 = 40.0;
const PLATFORM_SPACING: f32 = 5.0;

struct GameState {
    dood: Rect,
    x_vel: f32,
    y_vel: f32,
    platforms: Vec<Rect>,
    dood_color: Color,
    platform_color: Color,
    platform_levels: usize,
    score: f32,
    game_over: bool,
}

impl GameState {
    fn draw(&self) {
        clear_background(GRAY);

        draw_rectangle(
            self.dood.x,
            self.dood.y,
            self.dood.w,
            self.dood.h,
            self.dood_color,
        );

        for platform in &self.platforms {
            draw_rectangle(
                platform.x,
                platform.y,
                platform.w,
                platform.h,
                self.platform_color,
            )
        }

        draw_text(&self.score.to_string(), 10.0, 100.0, 30.0, self.dood_color);
        if self.game_over {
            draw_text(
                "refresh page to play again",
                screen_width() / 4.0,
                screen_height() / 2.0,
                50.0,
                RED,
            )
        }
    }

    fn update(&mut self) {
        // updating state
        let mut rng = thread_rng();

        // adding state until we've reached the top

        while self.platforms.len() < self.platform_levels {
            let curr_levels = self.platforms.len();

            let curr_y =
                screen_height() - (curr_levels as f32) * PLATFORM_HEIGHT * PLATFORM_SPACING;
            let curr_x = rng.gen_range((0.0)..(screen_width() - PLATFORM_WIDTH));
            // let curr_x =
            //     (screen_width() / 2.0 + (curr_levels as f32) * PLATFORM_WIDTH) % screen_width();

            self.platforms
                .push(Rect::new(curr_x, curr_y, PLATFORM_WIDTH, PLATFORM_HEIGHT));
            if curr_levels == 1 {
                dbg!(curr_x, curr_y);
                self.dood
                    .move_to(Vec2::new(curr_x, curr_y - PLATFORM_HEIGHT * 2.0));
            }
        }

        // define jump
        let mut yaccel: f32 = 0.1;

        if is_key_down(macroquad::input::KeyCode::Right) || is_key_down(miniquad::KeyCode::D) {
            self.x_vel = XVEL;
        } else if is_key_down(macroquad::input::KeyCode::Left) || is_key_down(miniquad::KeyCode::A)
        {
            self.x_vel = -XVEL;
        } else {
            self.x_vel = 0.0;
        }

        let platforms_iter = self.platforms.iter();

        // check collision with rect
        for platform in platforms_iter {
            if self.dood.overlaps(platform) {
                let sideways =
                    self.dood.top() < platform.top() && self.dood.bottom() > platform.bottom();
                if self.dood.bottom() < ((platform.bottom() + platform.top()) / 2.0)
                    && self.y_vel >= -0.5
                {
                    self.y_vel = -5.0;
                }
            }
        }

        // move if on other side of screen
        if self.dood.right() < 0.0 {
            self.dood
                .move_to(Vec2::new(screen_width() - self.dood.w, self.dood.y));
        } else if self.dood.left() > screen_width() {
            self.dood.move_to(Vec2::new(0.0, self.dood.y));
        }

        // if dood disappears, start over
        if self.dood.bottom() >= screen_height() {
            self.game_over = true;
            self.dood.move_to(Vec2::new(
                self.dood.left(),
                screen_height() + self.dood.h * 2.0,
            ));
        } else {
            // otherwise accel downward
            self.y_vel += yaccel;
        }

        // moving the position of the dude or screen based on height
        self.dood.x += self.x_vel;
        if self.dood.y < (screen_height() / 2.0) && self.y_vel < 0.0 {
            for platform in self.platforms.iter_mut() {
                platform.move_to(Vec2::new(platform.x, platform.y - self.y_vel));
            }

            self.platforms = self
                .platforms
                .iter()
                .filter(|&x| x.top() < screen_height())
                .cloned()
                .collect();

            // updating score
            self.score -= self.y_vel;
        } else {
            self.dood.y += self.y_vel;
        }
    }
}

#[macroquad::main("DoodleJump")]
async fn main() {
    let mut mainstate = GameState {
        dood: Rect::new(screen_width() / 2.0, screen_height() - 30.0, 20.0, 20.0),
        x_vel: 0.0,
        y_vel: 0.0,
        platforms: Vec::new(),

        platform_color: YELLOW,
        dood_color: GREEN,
        platform_levels: (screen_height() / (PLATFORM_HEIGHT * PLATFORM_SPACING)).floor() as usize,
        score: 0.0,
        game_over: false,
    };
    loop {
        mainstate.update();

        mainstate.draw();
        next_frame().await;

        // quit
        if is_key_pressed(miniquad::KeyCode::Q) {
            break;
        }
    }
}
