// TODO:
// 1. Don't allow selector to go outside of boundaries
// 2. implement block placement

mod level {

    use raylib::prelude::*;

    #[derive(Copy, Clone)]
    pub enum Blocks {
        GRASS,
        STONE,
        PLAYER,
    }

    pub struct Level {
        pub data: [[Blocks; 16]; 16],
    }

    impl Level {
        pub fn new() -> Self {
            Level {
                data: [[Blocks::STONE; 16]; 16],
            }
        }

        pub fn render(&self, d: &mut RaylibDrawHandle, texture_atlas: &Texture2D) {
            for i in 0..self.data.len() {
                for j in 0..self.data[i].len() {
                    let dest_rect = Rectangle {
                        x: i as f32 * 64.0,
                        y: j as f32 * 64.0,
                        width: 64.0,
                        height: 64.0,
                    };
                    let mut texture_section = Rectangle {
                        x: 0.0,
                        y: 0.0,
                        width: 32.0,
                        height: 32.0,
                    };

                    match self.data[i][j] {
                        Blocks::STONE => {
                            texture_section.x += 0.0;
                            texture_section.y += 32.0;
                        }
                        Blocks::GRASS => {
                            texture_section.x += 32.0;
                            texture_section.y += 32.0;
                        }

                        _ => texture_section.y += 32.0,
                    }

                    d.draw_texture_pro(
                        texture_atlas,
                        texture_section,
                        dest_rect,
                        Vector2::new(0.0, 0.0),
                        0.0,
                        Color::WHITE,
                    );
                }
            }
        }
    }
}

mod selector {

    use crate::level::*;
    use raylib::prelude::*;
    use std::os::raw::c_int;

    pub struct Selector {
        x: f32,
        y: f32,
    }

    impl Selector {
        pub fn new() -> Self {
            Selector { x: 0.0, y: 0.0 }
        }

        pub fn render(&self, d: &mut RaylibDrawHandle, texture_atlas: &Texture2D) {
            let dest_rect = Rectangle {
                x: self.x,
                y: self.y,
                width: 64.0,
                height: 64.0,
            };
            let texture_section = Rectangle {
                x: 0.0,
                y: 64.0,
                width: 32.0,
                height: 32.0,
            };
            d.draw_texture_pro(
                texture_atlas,
                texture_section,
                dest_rect,
                Vector2::new(0.0, 0.0),
                0.0,
                Color::WHITE,
            );
        }

        pub fn mov(&mut self, level: &mut Level) {
            unsafe {
                if ffi::IsKeyPressed(ffi::KeyboardKey::KEY_K as c_int) {
                    self.y -= 64.0;
                }
                if ffi::IsKeyPressed(ffi::KeyboardKey::KEY_H as c_int) {
                    self.x -= 64.0;
                }
                if ffi::IsKeyPressed(ffi::KeyboardKey::KEY_J as c_int) {
                    self.y += 64.0;
                }
                if ffi::IsKeyPressed(ffi::KeyboardKey::KEY_L as c_int) {
                    self.x += 64.0;
                }

                if ffi::IsKeyPressed(ffi::KeyboardKey::KEY_B as c_int) {
                    level.data[self.x as usize / 64][self.y as usize / 64] = Blocks::GRASS;
                }
            }
        }
    }
}

use level::*;
use raylib::prelude::*;
use selector::*;

fn main() {
    let (mut rl, thread) = raylib::init().size(1024, 1024).build();

    let texture_atlas = rl
        .load_texture(&thread, "./player_sheet.png")
        .expect("Failed to load texture");

    let mut level = Level::new();

    let mut selector = Selector::new();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        selector.mov(&mut level);
        d.clear_background(Color::GRAY);
        level.render(&mut d, &texture_atlas);
        selector.render(&mut d, &texture_atlas);
    }
}
