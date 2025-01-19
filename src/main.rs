// TODO:
// 1. Multiple block placement
// 2. Big Block placement
// 3. 3D textures
// 4. color/ more varied map
// 5. Block Collision Detection

const SCREEN_WIDTH: i32 = 1024;
const SCREEN_HEIGHT: i32 = 1024;
const WORLD_SIZE: usize = 16;

use raylib::prelude::*;
use render::Renderer;
use selector::Selector;
use world::World;

pub mod render;
pub mod selector;
pub mod world;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .build();

    let texture_atlas = rl
        .load_texture(&thread, "./player_sheet.png")
        .expect("Failed to load texture");

    let mut world = World::new(WORLD_SIZE);

    let mut selector = Selector::new();

    let mut renderer = Renderer::new(&selector);

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        selector.mov(&mut world, &mut renderer.camera);
        d.clear_background(Color::BLACK);
        renderer.render(&mut d, &texture_atlas, &world, &selector);
    }
}
