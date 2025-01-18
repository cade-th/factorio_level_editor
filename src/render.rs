use crate::world::Blocks;
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
        let zoom_text = format!("Zoom: ({:.2})", camera.zoom);

        // Draw text on the screen
        d.draw_text(&target_text, 10, 10, 20, Color::WHITE);
        d.draw_text(&offset_text, 10, 40, 20, Color::WHITE);
        d.draw_text(&zoom_text, 10, 70, 20, Color::WHITE);
    }
}
