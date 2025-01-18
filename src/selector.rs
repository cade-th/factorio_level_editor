use crate::world::Blocks;
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

    // TODO:
    // 1. add remove block function
    // 2. add otther blocks to place

    pub fn mov(&mut self, world: &mut World, camera: &mut Camera2D, tile_size: f32) {
        let offset_mov = tile_size;

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
            if ffi::IsKeyPressed(ffi::KeyboardKey::KEY_S as c_int) {
                let _ = world.data_to_file("data.cade");
            }
            if ffi::IsKeyPressed(ffi::KeyboardKey::KEY_R as i32) {
                world.data[self.x][self.y] = Blocks::STONE;
            }
        }
    }
}
