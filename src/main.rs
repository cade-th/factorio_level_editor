// TODO:
// 1. Multiple block placement
// 2. Big Block placement
// 3. Maybe merge into game engine?
// 4. Center Zooms

const SCREEN_WIDTH: u32 = 1024;
const SCREEN_HEIGHT: u32 = 1024;
const WORLD_SIZE: usize = 32;

#[derive(Copy, Clone, Debug)]
pub enum Blocks {
    GRASS = 0,
    STONE = 1,
    PLAYER = 2,
}

impl Blocks {
    fn to_u8(self) -> u8 {
        self as u8
    }
}

mod renderer {
    use crate::Blocks;
    use crate::Selector;
    use crate::World;
    use raylib::prelude::*;

    pub struct Renderer {}

    impl Renderer {
        pub fn new() -> Self {
            Renderer {}
        }

        fn world_to_screen(world_pos: Vector2, camera: &Camera2D) -> Vector2 {
            Vector2::new(
                (world_pos.x - camera.target.x) * camera.zoom + camera.offset.x,
                (world_pos.y - camera.target.y) * camera.zoom + camera.offset.y,
            )
        }

        pub fn render(
            &self,
            d: &mut RaylibDrawHandle,
            texture_atlas: &Texture2D,
            world: &World,
            selector: &Selector,
            camera: &Camera2D,
        ) {
            // Begin 2D camera mode
            let _ = d.begin_mode2D(*camera);

            let tile_size = 64.0 * camera.zoom;

            // Render the world (tiles)
            for i in 0..world.data.len() {
                for j in 0..world.data[0].len() {
                    // Convert grid position to world space
                    let world_pos = Vector2::new(i as f32 * tile_size, j as f32 * tile_size);

                    // Convert world space to screen space using the helper function
                    let screen_pos = Self::world_to_screen(world_pos, camera);

                    let dest_rect = Rectangle {
                        x: screen_pos.x,
                        y: screen_pos.y,
                        width: tile_size * camera.zoom, // Scale the tile with zoom
                        height: tile_size * camera.zoom,
                    };

                    let texture_section = match world.data[i][j] {
                        Blocks::STONE => Rectangle {
                            x: 0.0,
                            y: 32.0,
                            width: 32.0,
                            height: 32.0,
                        },
                        Blocks::GRASS => Rectangle {
                            x: 32.0,
                            y: 32.0,
                            width: 32.0,
                            height: 32.0,
                        },
                        _ => Rectangle {
                            x: 0.0,
                            y: 0.0,
                            width: 32.0,
                            height: 32.0,
                        },
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
            }

            // Render the selector
            let selector_world_pos =
                Vector2::new(selector.x as f32 * tile_size, selector.y as f32 * tile_size);

            let selector_screen_pos = Self::world_to_screen(selector_world_pos, camera);

            let selector_dest_rect = Rectangle {
                x: selector_screen_pos.x,
                y: selector_screen_pos.y,
                width: tile_size * camera.zoom,
                height: tile_size * camera.zoom,
            };

            let selector_texture_section = Rectangle {
                x: 0.0,
                y: 64.0,
                width: 32.0,
                height: 32.0,
            };

            d.draw_texture_pro(
                texture_atlas,
                selector_texture_section,
                selector_dest_rect,
                Vector2::new(0.0, 0.0),
                0.0,
                Color::WHITE,
            );

            // End 2D camera mode

            // Display camera information
            let target_text = format!("Target: ({:.2}, {:.2})", camera.target.x, camera.target.y);
            let offset_text = format!("Offset: ({:.2}, {:.2})", camera.offset.x, camera.offset.y);

            // Draw text on the screen
            d.draw_text(&target_text, 10, 10, 20, Color::WHITE);
            d.draw_text(&offset_text, 10, 40, 20, Color::WHITE);
        }
    }
}

mod world {

    use crate::Blocks;
    use crate::WORLD_SIZE;
    use std::fs::File;
    use std::io::{self, Write};

    pub struct World {
        pub data: [[Blocks; WORLD_SIZE]; WORLD_SIZE],
    }

    impl World {
        pub fn new() -> Self {
            World {
                data: [[Blocks::STONE; WORLD_SIZE]; WORLD_SIZE],
            }
        }

        pub fn data_to_file(&self, file_name: &str) -> io::Result<()> {
            let mut file = File::create(file_name)?;

            for row in &self.data {
                for &block in row {
                    file.write_all(&[block.to_u8()])?;
                }
            }

            println!("world data saved to {}", file_name);
            Ok(())
        }
    }
}

mod selector {

    use crate::Blocks;
    use crate::World;
    use raylib::prelude::*;
    use std::os::raw::c_int;

    pub struct Selector {
        pub x: usize,
        pub y: usize,
    }

    impl Selector {
        pub fn new() -> Self {
            Selector { x: 0, y: 0 }
        }

        pub fn mov(&mut self, world: &mut World, camera: &mut Camera2D, tile_size: f32) {
            let offset_mov = camera.zoom * 50.0;

            unsafe {
                if ffi::IsKeyPressed(ffi::KeyboardKey::KEY_K as c_int) {
                    if self.y > 0 {
                        self.y = self.y.saturating_sub(1);
                        camera.target.y = self.y as f32;
                        camera.offset.y += offset_mov;
                    }
                }
                if ffi::IsKeyPressed(ffi::KeyboardKey::KEY_H as c_int) {
                    if self.x > 0 {
                        self.x = self.x.saturating_sub(1);
                        camera.target.x = self.x as f32;
                        camera.offset.x += offset_mov;
                    }
                }
                if ffi::IsKeyPressed(ffi::KeyboardKey::KEY_J as c_int) {
                    if self.y + 1 < world.data[0].len() {
                        self.y = self.y + 1;
                        camera.target.y = self.y as f32;
                        camera.offset.y -= offset_mov;
                    }
                }
                if ffi::IsKeyPressed(ffi::KeyboardKey::KEY_L as c_int) {
                    if self.x + 1 < world.data.len() {
                        self.x = self.x + 1;
                        camera.target.x = self.x as f32;
                        camera.offset.x -= offset_mov;
                    }
                }
                if ffi::IsKeyPressed(ffi::KeyboardKey::KEY_W as c_int) {
                    world.data[self.x][self.y] = Blocks::GRASS;
                }

                if ffi::IsKeyPressed(ffi::KeyboardKey::KEY_D as i32) {
                    camera.zoom -= 0.05;
                    println!("Zoom: {:.2}", camera.zoom);
                }

                if ffi::IsKeyPressed(ffi::KeyboardKey::KEY_F as i32) {
                    camera.zoom += 0.05;
                    println!("Zoom: {:.2}", camera.zoom);
                }
                if ffi::IsKeyPressed(ffi::KeyboardKey::KEY_R as i32) {
                    camera.offset.x += 50.0;
                }

                if ffi::IsKeyPressed(ffi::KeyboardKey::KEY_S as c_int) {
                    let _ = world.data_to_file("data.cade");
                }
            }
        }
    }
}

use crate::renderer::*;
use crate::selector::*;
use crate::world::*;
use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .build();

    let texture_atlas = rl
        .load_texture(&thread, "./player_sheet.png")
        .expect("Failed to load texture");

    let mut world = World::new();

    let tile_size = 64.0;

    let mut selector = Selector::new();

    let mut renderer = Renderer::new();

    let mut camera = Camera2D {
        target: Vector2::new(selector.x as f32, selector.y as f32),
        offset: Vector2::new(selector.x as f32 + 512.0, selector.y as f32 + 512.0),
        rotation: 0.0,
        zoom: 1.0,
    };

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        selector.mov(&mut world, &mut camera, tile_size);
        d.clear_background(Color::GRAY);
        renderer.render(&mut d, &texture_atlas, &world, &selector, &camera);
    }
}
