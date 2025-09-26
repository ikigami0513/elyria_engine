use hashbrown::HashMap;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;

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
struct SpritesheetSerializer {
    name: String,
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
    pub texture: Texture,
    sprites: HashMap<String, Sprite>
}

impl Spritesheet {
    pub fn new(metadata_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = File::open(metadata_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let serializer: SpritesheetSerializer = serde_json::from_str(&contents)?;

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
            u_min, v_max, // Haut-gauche (doit utiliser v_max, le haut)
            u_min, v_min, // Bas-gauche (doit utiliser v_min, le bas)
            u_max, v_min, // Bas-droit (doit utiliser v_min, le bas)
            // Triangle 2
            u_min, v_max, // Haut-gauche (doit utiliser v_max, le haut)
            u_max, v_min, // Bas-droit (doit utiliser v_min, le bas)
            u_max, v_max, // Haut-droit (doit utiliser v_max, le haut)
            ];

            sprites.insert(s_data.name.clone(), Sprite { 
                tex_coords,
                width: s_data.width,
                height: s_data.height
            });
        }

        Ok(Spritesheet { texture, sprites })
    }

    pub fn get_sprite(&self, name: &str) -> Option<&Sprite> {
        self.sprites.get(name)
    }
}