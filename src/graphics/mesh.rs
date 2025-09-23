use std::ffi::CString;
use std::mem::size_of;
use std::os::raw::c_void;
use std::ptr;
use std::mem::offset_of;

use cgmath::{ Vector3, Vector2 };
use cgmath::prelude::*;
use gl;

use crate::glutils::buffer::{ElementBuffer, VertexArray, VertexBuffer};
use crate::glutils::shader::Shader;
use crate::glutils::texture::Texture;

// NOTE: without repr(C) the compiler may reorder the fields or use different padding/alignment than C.
// Depending on how you pass the data to OpenGL, this may be bad. In this case it's not strictly
// necessary though because of the `offset!` macro used below in setupMesh()
#[repr(C)]
pub struct Vertex {
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub tex_coords: Vector2<f32>,
    pub tangent: Vector3<f32>,
    pub bitangent: Vector3<f32>
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex {
            position: Vector3::zero(),
            normal: Vector3::zero(),
            tex_coords: Vector2::zero(),
            tangent: Vector3::zero(),
            bitangent: Vector3::zero()
        }
    }
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub textures: Vec<Texture>,
    pub vao: VertexArray,
    pub vbo: VertexBuffer,
    pub ebo: ElementBuffer
}

#[allow(dead_code)]
impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, textures: Vec<Texture>) -> Mesh {
        let mut mesh = Mesh {
            vertices,
            indices,
            textures,
            vao: VertexArray::new(),
            vbo: VertexBuffer::new(),
            ebo: ElementBuffer::new()
        };
        unsafe { mesh.setup_mesh() }
        mesh
    }

    pub unsafe fn render(&self, shader: &Shader) {
        let mut diffuse_nr = 0;
        let mut specular_nr = 0;
        let mut normal_nr = 0;
        let mut height_nr = 0;

        for (i, texture) in self.textures.iter().enumerate() {
            texture.active(i as u32);

            let name = &texture.get_type();
            let number = match name.as_str() {
                "texture_diffuse" => {
                    diffuse_nr += 1;
                    diffuse_nr
                },
                "texture_specular" => {
                    specular_nr += 1;
                    specular_nr
                }
                "texture_normal" => {
                    normal_nr += 1;
                    normal_nr
                }
                "texture_height" => {
                    height_nr += 1;
                    height_nr
                }
                _ => panic!("unknown texture type")
            };

            let sampler = CString::new(format!("{}{}", name, number)).unwrap();
            unsafe {
                shader.set_int(&sampler, i as i32);
                texture.bind();
            }
        }

        self.vao.bind();
        unsafe { gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, ptr::null()) }
        self.vao.unbind();

        for i in 0..self.textures.len() {
            unsafe {
                gl::ActiveTexture(gl::TEXTURE0 + i as u32);
                gl::BindTexture(gl::TEXTURE_2D, 0);
            }
        }
    }

    unsafe fn setup_mesh(&mut self) {
        self.vao.bind();

        self.vbo.bind();
        self.vbo.set_data(&self.vertices);

        self.ebo.bind();
        self.ebo.set_data(&self.indices);

        let size = size_of::<Vertex>() as i32;

        // vertex positions
        self.vao.set_attribute(0, 3, gl::FLOAT, size, offset_of!(Vertex, position) as *const c_void);

        // vertex normals
        self.vao.set_attribute(1, 3, gl::FLOAT, size, offset_of!(Vertex, normal) as *const c_void);

        // vertex texture coords
        self.vao.set_attribute(2, 2, gl::FLOAT, size, offset_of!(Vertex, tex_coords) as *const c_void);

        // vertex tangent
        self.vao.set_attribute(3, 3, gl::FLOAT, size, offset_of!(Vertex, tangent) as *const c_void);

        // vertex bitangent
        self.vao.set_attribute(4, 3, gl::FLOAT, size, offset_of!(Vertex, bitangent) as *const c_void);

        self.vao.unbind();
    }
}