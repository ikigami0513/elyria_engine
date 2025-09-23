
use std::path::{Path, PathBuf};

use cgmath::{vec2, vec3};
use tobj;

use crate::glutils::texture::Texture;
use crate::graphics::mesh::{ Mesh, Vertex };
use crate::world::component::Component;

#[derive(Default)]
pub struct Model {
    pub meshes: Vec<Mesh>,
    pub textures_loaded: Vec<Texture>,
    directory: String
}

#[allow(dead_code)]
impl Model {
    pub fn new(path: &str) -> Model {
        let mut model = Model::default();
        model.load_model(path);
        model
    }

    fn load_model(&mut self, path: &str) {
        let path = Path::new(path);

        self.directory = path.parent().unwrap_or_else(|| Path::new("")).to_str().unwrap().into();
        let obj = tobj::load_obj(path);

        let (models, materials) = obj.unwrap();
        for model in models {
            let mesh = &model.mesh;
            let num_vertices = mesh.positions.len() / 3;

            let mut vertices: Vec<Vertex> = Vec::with_capacity(num_vertices);
            let indices: Vec<u32> = mesh.indices.clone();

            let (p, n, t) = (&mesh.positions, &mesh.normals, &mesh.texcoords);
            for i in 0..num_vertices {
                vertices.push(Vertex {
                    position:  vec3(p[i*3], p[i*3+1], p[i*3+2]),
                    normal:    vec3(n[i*3], n[i*3+1], n[i*3+2]),
                    tex_coords: vec2(t[i*2], t[i*2+1]),
                    ..Vertex::default()
                })
            }

            let mut textures = Vec::new();
            if let Some(material_id) = mesh.material_id {
                let material = &materials[material_id];

                // diffuse map
                if !material.diffuse_texture.is_empty() {
                    let texture = self.load_material_texture(&material.diffuse_texture, "texture_diffuse");
                    textures.push(texture);
                }

                // specular map
                if !material.specular_texture.is_empty() {
                    let texture = self.load_material_texture(&material.specular_texture, "texture_specular");
                    textures.push(texture);
                }

                // normal map
                if !material.normal_texture.is_empty() {
                    let texture = self.load_material_texture(&material.normal_texture, "texture_normal");
                    textures.push(texture);
                }
            }

            self.meshes.push(Mesh::new(vertices, indices, textures));
        }
    }

    fn load_material_texture(&mut self, path: &str, typename: &str) -> Texture {
        {
            let texture = self.textures_loaded.iter().find(|t| t.get_path() == path);
            if let Some(texture) = texture {
                return texture.clone();
            }
        }

        let mut full_path = PathBuf::from(&self.directory);
        full_path.push(path);
        let texture = Texture::new(full_path.to_str().unwrap(), typename.into());
        self.textures_loaded.push(texture.clone());
        texture
    }
}

#[allow(unused_variables)]
impl Component for Model {
    fn render(&self, owner: &std::rc::Rc<std::cell::RefCell<crate::world::entity::Entity>>, shader: &crate::glutils::shader::Shader) {
        for mesh in &self.meshes {
            unsafe { mesh.render(shader); }
        }
    }
}
