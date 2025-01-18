// TODO:
// 1. Multiple block placement
// 2. Big Block placement
// 3. 3D textures
// 4. color/ more varied map
// 5. Block Collision Detection

const SCREEN_WIDTH: u32 = 1024;
const SCREEN_HEIGHT: u32 = 1024;

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

    let mut world = World::new();

    let tile_size = 64.0;

    let mut selector = Selector::new();

    let renderer = Renderer::new();

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
        d.clear_background(Color::BLACK);
        renderer.render(&mut d, &texture_atlas, &world, &selector, &camera);
    }
}
