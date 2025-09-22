use cgmath;
use cgmath::vec3;
use cgmath::prelude::*;
use glfw::Key;

use crate::core::frame_context::FrameContext;

type Point3 = cgmath::Point3<f32>;
type Vector3 = cgmath::Vector3<f32>;
type Matrix4 = cgmath::Matrix4<f32>;

const YAW: f32 = -90.0;
const PITCH: f32 = 0.0;
const SPEED: f32 = 2.5;
const SENSITIVTY: f32 = 0.1;
const ZOOM: f32 = 45.0;

pub struct Camera {
    // Camera attributes
    pub position: Point3,
    pub front: Vector3,
    pub up: Vector3,
    pub right: Vector3,
    pub world_up: Vector3,

    // Euler Angles
    pub yaw: f32,
    pub pitch: f32,
    pub constrain_pitch: bool,

    // Camera options
    pub movement_speed: f32,
    pub mouse_sensitivity: f32,
    pub zoom: f32
}

impl Default for Camera {
    fn default() -> Self {
        let mut camera = Camera {
            position: Point3::new(0.0, 0.0, 0.0),
            front: vec3(0.0, 0.0, -1.0),
            up: Vector3::zero(),
            right: Vector3::zero(),
            world_up: Vector3::unit_y(),
            yaw: YAW,
            pitch: PITCH,
            constrain_pitch: true,
            movement_speed: SPEED,
            mouse_sensitivity: SENSITIVTY,
            zoom: ZOOM
        };
        camera.update_camera_vectors();
        camera
    }
}

impl Camera {
    pub fn get_view_matrix(&self) -> Matrix4 {
        Matrix4::look_at(self.position, self.position + self.front, self.up)
    }

    pub fn update(&mut self, ctx: &FrameContext) {
        let velocity = self.movement_speed * ctx.time.delta_time();

        if ctx.input.is_key_pressed(Key::W) {
            self.position += self.front * velocity;
        }
        if ctx.input.is_key_pressed(Key::S) {
            self.position += -(self.front * velocity);
        }

        if ctx.input.is_key_pressed(Key::A) {
            self.position += -(self.right * velocity);
        }
        if ctx.input.is_key_pressed(Key::D) {
            self.position += self.right * velocity;
        }

        if self.zoom >= 1.0 && self.zoom <= 45.0 {
            self.zoom -= ctx.input.get_scroll_delta();
        }
        if self.zoom <= 1.0 {
            self.zoom = 1.0;
        }
        if self.zoom >= 45.0 {
            self.zoom = 45.0;
        }

        let (mut xoffset, mut yoffset) = ctx.input.get_mouse_delta();
        xoffset *= self.mouse_sensitivity;
        yoffset *= self.mouse_sensitivity;

        self.yaw += xoffset;
        self.pitch += yoffset;

        if self.constrain_pitch {
            if self.pitch > 89.0 {
                self.pitch = 89.0;
            }
            if self.pitch < -89.0 {
                self.pitch = -89.0;
            }
        }

        self.update_camera_vectors();
    }

    fn update_camera_vectors(&mut self) {
        let front = Vector3 {
            x: self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            y: self.pitch.to_radians().sin(),
            z: self.yaw.to_radians().sin() * self.pitch.to_radians().cos()
        };
        self.front = front.normalize();
        self.right = self.front.cross(self.world_up).normalize();
        self.up = self.right.cross(self.front).normalize();
    }
}
