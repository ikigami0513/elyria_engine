use cgmath;
use cgmath::{vec3, Matrix4, Vector3};

use crate::core::frame_context::FrameContext;
use crate::world::components::TransformComponent;
use crate::world::entity::Entity;

const SPEED: f32 = 200.0;
const ZOOM_SENSITIVITY: f32 = 0.1;
const INITIAL_ZOOM: f32 = 1.0;

pub struct Camera {
    pub position: Vector3<f32>,
    pub zoom: f32,
    pub movement_speed: f32,
    pub target: Option<Entity>
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            position: Vector3::new(0.0, 0.0, 0.0),
            zoom: INITIAL_ZOOM,
            movement_speed: SPEED,
            target: None
        }
    }
}

impl Camera {
    pub fn get_view_matrix(&self, width: u32, height: u32) -> Matrix4<f32> {
        let center_x = self.position.x - (width as f32 / 2.0) / self.zoom;
        let center_y = self.position.y - (height as f32 / 2.0) / self.zoom;

        let translation = Matrix4::from_translation(vec3(-center_x, -center_y, 0.0));
        let scale = Matrix4::from_scale(self.zoom);

        scale * translation
    }

    pub fn update(&mut self, ctx: &FrameContext) {
        let scroll_delta = ctx.input.get_scroll_delta();
        if scroll_delta != 0.0 {
            self.zoom += scroll_delta * ZOOM_SENSITIVITY;
            self.zoom = self.zoom.clamp(0.1, 5.0);
        }

        if let Some(target_entity) = self.target {
            if let Some(transform_comp) = ctx.world.get_component::<TransformComponent>(target_entity) {
                self.position.x = transform_comp.transform.get_global_position().x;
                self.position.y = transform_comp.transform.get_global_position().y;
            }
        }
    }
}