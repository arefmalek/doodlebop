use macroquad::{prelude::*, ui::KeyCode};
use std::cmp;

const XVEL: f32 = 3.5;
const XACCEL: f32 = 0.05;
const PLATFORM_HEIGHT: f32 = 10.0;
const PLATFORM_WIDTH: f32 = 30.0;
const PLATFORM_SPACING: f32 = 2.0;

struct GameState {
    dood: Rect,
    x_vel: f32,
    y_vel: f32,
    platforms: Vec<Rect>,
    dood_color: Color,
    platform_color: Color,
    platform_levels: usize,
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
    }

    fn update(&mut self) {
        // updating state

        // adding state until we've reached the top
        let mut curr_levels = self.platforms.len();

        while curr_levels < self.platform_levels {
            let curr_y =
                screen_height() - (curr_levels as f32) * PLATFORM_HEIGHT * PLATFORM_SPACING;
            let curr_x =
                (screen_width() / 2.0 + (curr_levels as f32) * PLATFORM_WIDTH) % screen_width();
            self.platforms
                .push(Rect::new(curr_x, curr_y, PLATFORM_WIDTH, PLATFORM_HEIGHT));

            curr_levels += 1
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
        //  else if self.x_vel > 0.0 {
        //     self.x_vel = f32::max(0.0, self.x_vel - XACCEL);
        // } else if self.x_vel < 0.0 {
        //     self.x_vel = f32::min(0.0, self.x_vel + XACCEL);
        // }

        let platforms_iter = self.platforms.iter();

        // file screen with platform

        // check collision with rect
        for platform in platforms_iter {
            if self.dood.overlaps(platform) {
                if platform.left() < self.dood.right() && self.dood.right() < platform.right() {
                    self.dood
                        .move_to(Vec2::new(platform.left() - self.dood.w, self.dood.y));
                } else if self.dood.left() > platform.right()
                    && self.dood.right() > platform.right()
                {
                    self.dood.move_to(Vec2::new(platform.right(), self.dood.y));
                }
                if self.dood.top() < platform.bottom() || self.dood.bottom() > platform.top() {
                    self.y_vel = if self.y_vel < 0.0 { -self.y_vel } else { -5.0 };
                }
                dbg!(platform.top(), self.dood.bottom());
            }
        }

        if self.dood.right() < 0.0 {
            self.dood
                .move_to(Vec2::new(screen_width() - self.dood.w, self.dood.y));
        } else if self.dood.left() > screen_width() {
            self.dood.move_to(Vec2::new(0.0, self.dood.y));
        }

        // if dood disappears, start over
        if self.dood.bottom() >= screen_height() {
            self.y_vel = -5.0
        } else {
            // otherwise accel downward
            self.y_vel += yaccel;
        }

        // moving the position of the dude
        self.dood.x += self.x_vel;
        self.dood.y += self.y_vel;
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
