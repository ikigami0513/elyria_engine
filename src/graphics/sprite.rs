use std::os::raw::c_void;
use gl::types::*;
use crate::glutils::{
    buffer::{VertexArray, VertexBuffer},
    texture::Texture
};
use crate::world::components::Component;

#[derive(Default)]
#[allow(dead_code)]
pub struct SpriteRendererComponent {
    pub vao: VertexArray,
    pub vbo: VertexBuffer,
    pub texture: Texture
}

impl Component for SpriteRendererComponent {}

pub struct SpriteCreator;

impl SpriteCreator {
    pub fn new_render_component(texture_path: &str) -> SpriteRendererComponent {
        let vertices: [f32; 24] = [
            //   Positions     TexCoords
            -0.5,  0.5,    0.0, 1.0, // Haut-gauche
            -0.5, -0.5,    0.0, 0.0, // Bas-gauche
             0.5, -0.5,    1.0, 0.0, // Bas-droit

            -0.5,  0.5,    0.0, 1.0, // Haut-gauche
             0.5, -0.5,    1.0, 0.0, // Bas-droit
             0.5,  0.5,    1.0, 1.0, // Haut-droit
        ];

        let vao = VertexArray::new();
        let vbo = VertexBuffer::new();

        vao.bind();
        vbo.bind();
        vbo.set_data(&vertices);

        let stride = 4 * std::mem::size_of::<GLfloat>() as GLsizei;

        vao.set_attribute(0, 2, gl::FLOAT, stride, std::ptr::null());
        vao.set_attribute(1, 2, gl::FLOAT, stride, (2 * std::mem::size_of::<GLfloat>()) as *const c_void);

        vbo.unbind();
        vao.unbind();

        let texture = Texture::new(texture_path);

        SpriteRendererComponent {
            vao,
            vbo,
            texture
        }
    }
}