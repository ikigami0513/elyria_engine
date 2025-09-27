use cgmath;
use cgmath::{vec3, Matrix4, Vector3};
use glfw::Key;

use crate::core::frame_context::FrameContext;

const SPEED: f32 = 200.0;
const ZOOM_SENSITIVITY: f32 = 0.1;
const INITIAL_ZOOM: f32 = 1.0;

pub struct Camera {
    pub position: Vector3<f32>,
    pub zoom: f32,
    pub movement_speed: f32
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            position: Vector3::new(0.0, 0.0, 0.0),
            zoom: INITIAL_ZOOM,
            movement_speed: SPEED
        }
    }
}

impl Camera {
    pub fn get_view_matrix(&self) -> Matrix4<f32> {
        let translation = Matrix4::from_translation(vec3(-self.position.x, -self.position.y, 0.0));
        let scale = Matrix4::from_scale(self.zoom);
        scale * translation
    }

    pub fn update(&mut self, ctx: &FrameContext) {
        let delta_time = ctx.time.delta_time();

        let scroll_delta = ctx.input.get_scroll_delta();
        if scroll_delta != 0.0 {
            self.zoom += scroll_delta * ZOOM_SENSITIVITY;
            self.zoom = self.zoom.clamp(0.1, 5.0);
        }

        let adjusted_speed = self.movement_speed * delta_time;

        if ctx.input.is_key_pressed(Key::W) { // Haut (ou Z sur AZERTY)
            self.position.y += adjusted_speed;
        }
        if ctx.input.is_key_pressed(Key::S) { // Bas
            self.position.y -= adjusted_speed;
        }
        if ctx.input.is_key_pressed(Key::A) { // Gauche (ou Q sur AZERTY)
            self.position.x -= adjusted_speed;
        }
        if ctx.input.is_key_pressed(Key::D) { // Droite
            self.position.x += adjusted_speed;
        }
    }
}