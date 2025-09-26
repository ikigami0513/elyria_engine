use std::io::Read;

use hashbrown::HashMap;
use crate::graphics::spritesheet::{Spritesheet, SpritesheetSerializer};

pub struct SpritesheetManager {
    spritesheets: HashMap<String, Spritesheet>
}

impl SpritesheetManager {
    pub fn new() -> Self {
        SpritesheetManager {
            spritesheets: HashMap::new()
        }
    }

    pub fn load(&mut self, metadata_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = std::fs::File::open(metadata_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let temp_serializer: SpritesheetSerializer = serde_json::from_str(&contents)?;

        if self.spritesheets.contains_key(&temp_serializer.name) {
            return Ok(())
        }

        let spritesheet = Spritesheet::from_serializer(temp_serializer)?;
        self.spritesheets.insert(spritesheet.name.clone(), spritesheet);
        
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<&Spritesheet> {
        self.spritesheets.get(name)
    }
}