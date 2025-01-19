use crate::Selector;
use crate::World;
use raylib::prelude::*;

pub struct Renderer {
    pub camera: Camera2D,
}

impl Renderer {
    pub fn new(selector: &Selector) -> Self {
        let offset = unsafe {
            Vector2::new(
                raylib::ffi::GetScreenWidth() as f32 / 2.0,
                raylib::ffi::GetScreenHeight() as f32 / 2.0,
            )
        };

        Renderer {
            camera: Camera2D {
                target: Vector2::new(selector.x as f32, selector.y as f32),
                offset,
                rotation: 0.0,
                zoom: 1.0,
            },
        }
    }

    pub fn render(
        &self,
        d: &mut RaylibDrawHandle,
        texture_atlas: &Texture2D,
        world: &World,
        selector: &Selector,
    ) {
        world.render(d, texture_atlas, &self.camera);

        selector.render(d, texture_atlas, world, &self.camera);

        // Display camera information
        let target_text = format!(
            "Target: ({:.2}, {:.2})",
            self.camera.target.x, self.camera.target.y
        );
        let offset_text = format!(
            "Offset: ({:.2}, {:.2})",
            self.camera.offset.x, self.camera.offset.y
        );
        let zoom_text = format!("Zoom: ({:.2})", self.camera.zoom);

        // Draw text on the screen
        d.draw_text(&target_text, 10, 10, 20, Color::WHITE);
        d.draw_text(&offset_text, 10, 40, 20, Color::WHITE);
        d.draw_text(&zoom_text, 10, 70, 20, Color::WHITE);
    }
}
