use std::os::raw::c_void;
use gl::types::*;
use std::rc::Rc;
use std::cell::RefCell;

use crate::c_str;
use crate::glutils::{
    shader::Shader,
    buffer::{VertexArray, VertexBuffer},
    texture::Texture
};
use crate::world::component::Component;
use crate::world::entity::Entity;

#[allow(dead_code)]
pub struct CuboidRenderer {
    vao: VertexArray,
    vbo: VertexBuffer,
    texture: Texture
}

impl CuboidRenderer {
    pub fn new(texture_path: &str) -> Self {
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
        vao.set_attribute(1, 2, gl::FLOAT, stride, (3 * std::mem::size_of::<GLfloat>()) as *const c_void);

        vbo.unbind();
        vao.unbind();

        let texture = Texture::new(texture_path, None);

        Self {
            vao,
            vbo,
            texture
        }
    }
}

impl Component for CuboidRenderer {
    fn render(&self, owner: &Rc<RefCell<Entity>>, shader: &Shader) {
        unsafe {
            shader.use_program();

            self.texture.active(0);
            self.texture.bind();
            shader.set_int(c_str!("texture_diffuse1"), 0);

            let owner_entity = owner.borrow();
            let model_matrix = owner_entity.transform.get_model_matrix();
            shader.set_mat4(c_str!("model"), model_matrix);

            self.vao.bind();
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }
    }
}
