use serde::Deserialize;
use crate::world::components::Component;

#[derive(Deserialize, Debug)]
pub struct AnimationSerializer {
    pub name: String,
    pub spritesheet: String,
    pub frame_duration: f32,
    pub loops: bool,
    pub flipped: bool,
    pub frames: Vec<String>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Animation {
    pub name: String,
    pub spritesheet_name: String,
    pub frame_duration: f32,
    pub loops: bool,
    pub flipped: bool,
    pub frames: Vec<String>,
}

#[derive(Clone)]
pub struct AnimationComponent {
    pub current_animation: Option<String>,
    pub current_frame_index: usize,
    pub timer: f32,
    pub is_playing: bool
}

impl Component for AnimationComponent {}

#[allow(dead_code)]
impl AnimationComponent {
    pub fn new() -> Self {
        Self {
            current_animation: None,
            current_frame_index: 0,
            timer: 0.0,
            is_playing: false
        }
    }

    pub fn play(&mut self, animation_name: &str) {
        if self.current_animation.as_deref() != Some(animation_name) {
            self.current_animation = Some(animation_name.to_string());
            self.current_frame_index = 0;
            self.timer = 0.0;
        }
        self.is_playing = true;
    }

    pub fn stop(&mut self) {
        self.is_playing = false;
        self.current_frame_index = 0;
        self.timer = 0.0;
    }
}