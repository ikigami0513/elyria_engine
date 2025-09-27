use std::os::raw::c_void;
use gl::types::*;
use crate::{glutils::{
    buffer::{VertexArray, VertexBuffer},
    texture::Texture
}, graphics::spritesheet::Spritesheet};
use crate::world::components::Component;

#[derive(Default)]
#[allow(dead_code)]
pub struct SpriteRendererComponent {
    pub vao: VertexArray,
    pub vbo: VertexBuffer,
    pub texture: Texture,
    pub width: u32,
    pub height: u32
}

impl Component for SpriteRendererComponent {}

pub struct SpriteCreator;

impl SpriteCreator {
    pub fn from_texture(texture_path: &str) -> SpriteRendererComponent {
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

        let width = texture.width;
        let height = texture.height;

        SpriteRendererComponent {
            vao,
            vbo,
            texture,
            width,
            height 
        }
    }

    pub fn from_sprite(spritesheet: &Spritesheet, sprite_name: &str) -> Option<SpriteRendererComponent> {
        let sprite_data = spritesheet.get_sprite(sprite_name)?;

        let positions: [f32; 12] = [
            // Triangle 1
            -0.5,  0.5, // Haut-gauche
            -0.5, -0.5, // Bas-gauche
             0.5, -0.5, // Bas-droit
             // Triangle 2
            -0.5,  0.5, // Haut-gauche
             0.5, -0.5, // Bas-droit
             0.5,  0.5, // Haut-droit
        ];

        let mut vertices = [0.0f32; 24];
        for i in 0..6 {
            vertices[i * 4] = positions[i * 2];         // Pos X
            vertices[i * 4 + 1] = positions[i * 2 + 1]; // Pos Y
            vertices[i * 4 + 2] = sprite_data.tex_coords[i * 2];     // Tex U
            vertices[i * 4 + 3] = sprite_data.tex_coords[i * 2 + 1]; // Tex V
        }

        let vao = VertexArray::new();
        let vbo = VertexBuffer::new();

        vao.bind();
        vbo.bind();
        vbo.set_data(&vertices);

        let stride = 4 * std::mem::size_of::<GLfloat>() as GLsizei;

        // position
        vao.set_attribute(0, 2, gl::FLOAT, stride, std::ptr::null());
        vao.set_attribute(1, 2, gl::FLOAT, stride, (2 * std::mem::size_of::<GLfloat>()) as *const c_void);

        vbo.unbind();
        vao.unbind();

        Some(SpriteRendererComponent {
            vao,
            vbo,
            texture: spritesheet.texture.clone(),
            width: sprite_data.width,
            height: sprite_data.height
        })
    }
}