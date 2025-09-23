use std::os::raw::c_void;
use gl::types::*;
use crate::glutils::{
    buffer::{VertexArray, VertexBuffer},
    texture::Texture
};
use crate::world::components::Component;

#[derive(Default)]
#[allow(dead_code)]
pub struct PrimitiveRenderComponent {
    pub vao: VertexArray,
    pub vbo: VertexBuffer,
    pub texture: Texture,
}

impl Component for PrimitiveRenderComponent {}

pub struct CuboidCreator;

impl CuboidCreator {
    pub fn new_render_component(texture_path: &str) -> PrimitiveRenderComponent {
        let vertices: [f32; 180] = [
            -0.5, -0.5, -0.5, 0.0, 0.0,
             0.5, -0.5, -0.5, 1.0, 0.0,
             0.5, 0.5, -0.5, 1.0, 1.0,
             0.5, 0.5, -0.5, 1.0, 1.0,
            -0.5, 0.5, -0.5, 0.0, 1.0,
            -0.5, -0.5, -0.5, 0.0, 0.0,

            -0.5, -0.5, 0.5, 0.0, 0.0,
             0.5, -0.5, 0.5, 1.0, 0.0,
             0.5, 0.5, 0.5, 1.0, 1.0,
             0.5, 0.5, 0.5, 1.0, 1.0,
            -0.5, 0.5, 0.5, 0.0, 1.0,
            -0.5, -0.5, 0.5, 0.0, 0.0,

            -0.5, 0.5, 0.5, 1.0, 0.0,
            -0.5, 0.5, -0.5, 1.0, 1.0,
            -0.5, -0.5, -0.5, 0.0, 1.0,
            -0.5, -0.5, -0.5, 0.0, 1.0,
            -0.5, -0.5, 0.5, 0.0, 0.0,
            -0.5, 0.5, 0.5, 1.0, 0.0,

             0.5, 0.5, 0.5, 1.0, 0.0,
             0.5, 0.5, -0.5, 1.0, 1.0,
             0.5, -0.5, -0.5, 0.0, 1.0,
             0.5, -0.5, -0.5, 0.0, 1.0,
             0.5, -0.5, 0.5, 0.0, 0.0,
             0.5, 0.5, 0.5, 1.0, 0.0,

            -0.5, -0.5, -0.5, 0.0, 1.0,
             0.5, -0.5, -0.5, 1.0, 1.0,
             0.5, -0.5, 0.5, 1.0, 0.0,
             0.5, -0.5, 0.5, 1.0, 0.0,
            -0.5, -0.5, 0.5, 0.0, 0.0,
            -0.5, -0.5, -0.5, 0.0, 1.0,

            -0.5, 0.5, -0.5, 0.0, 1.0,
             0.5, 0.5, -0.5, 1.0, 1.0,
             0.5, 0.5, 0.5, 1.0, 0.0,
             0.5, 0.5, 0.5, 1.0, 0.0,
            -0.5, 0.5, 0.5, 0.0, 0.0,
            -0.5, 0.5, -0.5, 0.0, 1.0 
        ];

        let vao = VertexArray::new();
        let vbo = VertexBuffer::new();

        vao.bind();
        vbo.bind();
        vbo.set_data(&vertices);

        let stride = 5 * std::mem::size_of::<GLfloat>() as GLsizei;

        vao.set_attribute(0, 3, gl::FLOAT, stride, std::ptr::null());
        vao.set_attribute(2, 2, gl::FLOAT, stride, (3 * std::mem::size_of::<GLfloat>()) as *const c_void);

        vbo.unbind();
        vao.unbind();

        let texture = Texture::new(texture_path, Some("texture_diffuse1".into()));

        PrimitiveRenderComponent {
            vao,
            vbo,
            texture
        }
    }
}
