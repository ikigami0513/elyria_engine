use hashbrown::HashMap;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;

use crate::core::path::get_path_to_asset;
use crate::glutils::texture::Texture;

#[derive(Deserialize, Debug)]
struct SpriteDataSerializer {
    name: String,
    x: u32,
    y: u32,
    width: u32,
    height: u32
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct SpritesheetSerializer {
    pub name: String,
    texture: String,
    sprites: Vec<SpriteDataSerializer>
}

#[derive(Debug, Clone)]
pub struct Sprite {
    pub tex_coords: [f32; 12],
    pub width: u32,
    pub height: u32
}

#[derive(Debug)]
pub struct Spritesheet {
    pub name: String,
    pub texture: Texture,
    sprites: HashMap<String, Sprite>
}

#[allow(dead_code)]
impl Spritesheet {
    pub fn from_file(metadata_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = File::open(get_path_to_asset(metadata_path))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let serializer: SpritesheetSerializer = serde_json::from_str(&contents)?;

        Self::from_serializer(serializer)
    }

    pub fn from_serializer(serializer: SpritesheetSerializer) -> Result<Self, Box<dyn std::error::Error>> {
        let texture = Texture::new(&serializer.texture);
        let tex_width = texture.width as f32;
        let tex_height = texture.height as f32;

        let mut sprites = HashMap::new();
        for s_data in serializer.sprites {
            let left = s_data.x as f32;
            let right = (s_data.x + s_data.width) as f32;
            let top = s_data.y as f32;
            let bottom = (s_data.y + s_data.height) as f32;

            let u_min = left / tex_width;
            let u_max = right / tex_width;
            let v_min = bottom / tex_height;
            let v_max = top / tex_height;

            let tex_coords = [
                // Triangle 1
                u_min, v_max, // Haut-gauche
                u_min, v_min, // Bas-gauche
                u_max, v_min, // Bas-droit
                // Triangle 2
                u_min, v_max, // Haut-gauche
                u_max, v_min, // Bas-droit
                u_max, v_max, // Haut-droit
            ];

            sprites.insert(
                s_data.name.clone(),
                Sprite {
                    tex_coords,
                    width: s_data.width,
                    height: s_data.height,
                },
            );
        }

        Ok(Spritesheet {
            name: serializer.name,
            texture,
            sprites,
        })
    }

    pub fn get_sprite(&self, name: &str) -> Option<&Sprite> {
        self.sprites.get(name)
    }
}
