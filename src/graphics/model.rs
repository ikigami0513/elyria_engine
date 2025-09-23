use std::path::{Path, PathBuf};
use cgmath::{vec2, vec3};
use tobj;
use crate::glutils::texture::Texture;
use crate::graphics::mesh::{ Mesh, Vertex };
use crate::world::components::Component;

#[derive(Default)]
#[allow(dead_code)]
pub struct ModelRenderComponent {
    pub meshes: Vec<Mesh>,
    pub textures: Vec<Texture>,
}
impl Component for ModelRenderComponent {}

pub struct ModelLoader {
    textures_loaded: Vec<Texture>,
}

impl ModelLoader {
    pub fn new() -> Self {
        ModelLoader {
            textures_loaded: Vec::new(),
        }
    }

    pub fn load_model(&mut self, path: &str) -> ModelRenderComponent {
        let path = Path::new(path);
        let directory = path.parent().unwrap_or_else(|| Path::new("")).to_str().unwrap();
        let obj = tobj::load_obj(path).expect("Failed to load OBJ file");

        let (models, materials) = obj;
        let mut meshes = Vec::new();

        for model in models {
            let mesh = &model.mesh;
            let num_vertices = mesh.positions.len() / 3;

            let mut vertices: Vec<Vertex> = Vec::with_capacity(num_vertices);
            let indices: Vec<u32> = mesh.indices.clone();

            let (p, n, t) = (&mesh.positions, &mesh.normals, &mesh.texcoords);
            for i in 0..num_vertices {
                vertices.push(Vertex {
                    position:   vec3(p[i*3], p[i*3+1], p[i*3+2]),
                    normal:     vec3(n[i*3], n[i*3+1], n[i*3+2]),
                    tex_coords: vec2(t[i*2], t[i*2+1]),
                    ..Vertex::default()
                })
            }

            let mut textures = Vec::new();
            if let Some(material_id) = mesh.material_id {
                let material = &materials[material_id];

                if !material.diffuse_texture.is_empty() {
                    let texture = self.load_material_texture(&material.diffuse_texture, "texture_diffuse", &directory);
                    textures.push(texture);
                }
                if !material.specular_texture.is_empty() {
                    let texture = self.load_material_texture(&material.specular_texture, "texture_specular", &directory);
                    textures.push(texture);
                }
                if !material.normal_texture.is_empty() {
                    let texture = self.load_material_texture(&material.normal_texture, "texture_normal", &directory);
                    textures.push(texture);
                }
            }
            meshes.push(Mesh::new(vertices, indices, textures));
        }

        ModelRenderComponent {
            meshes,
            textures: self.textures_loaded.clone(),
        }
    }

    fn load_material_texture(&mut self, path: &str, typename: &str, directory: &str) -> Texture {
        if let Some(texture) = self.textures_loaded.iter().find(|t| t.get_path() == path) {
            return texture.clone();
        }

        let mut full_path = PathBuf::from(directory);
        full_path.push(path);
        let texture = Texture::new(full_path.to_str().unwrap(), typename.into());
        self.textures_loaded.push(texture.clone());
        texture
    }
}
