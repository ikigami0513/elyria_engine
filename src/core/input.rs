use glfw::{Action, Key, WindowEvent};
use std::collections::HashMap;

#[derive(Default)]
struct KeyState {
    pressed: bool,
    just_pressed: bool,
    just_released: bool
}

pub struct InputHandler {
    key_states: HashMap<Key, KeyState>, // true = pressed, false = released
    pub last_x: f32,
    pub last_y: f32,
    first_mouse: bool,
    scroll_delta: f32,
}

#[allow(dead_code)]
impl InputHandler {
    pub fn new() -> Self {
        InputHandler {
            key_states: HashMap::new(),
            last_x: 0.0,
            last_y: 0.0,
            scroll_delta: 0.0,
            first_mouse: true,
        }
    }

    pub fn update(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::Key(key, _, action, _) => {
                let state = self.key_states.entry(*key).or_default();

                match action {
                    Action::Press | Action::Repeat => {
                        // just_pressed seulement si elle n’était pas déjà enfoncée
                        if !state.pressed {
                            state.just_pressed = true;
                        }
                        state.pressed = true;
                    }
                    Action::Release => {
                        if state.pressed {
                            state.just_released = true;
                        }
                        state.pressed = false;
                    }
                }
            }
            WindowEvent::CursorPos(x, y) => {
                if self.first_mouse {
                    self.last_x = *x as f32;
                    self.last_y = *y as f32;
                    self.first_mouse = false;
                }
            }
            WindowEvent::Scroll(_, y) => {
                self.scroll_delta = *y as f32;
            }
            _ => {}
        }
    }

    pub fn end_frame(&mut self) {
        for state in self.key_states.values_mut() {
            state.just_pressed = false;
            state.just_released = false;
        }
        self.scroll_delta = 0.0;
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.key_states.get(&key).map_or(false, |s| s.pressed)
    }

    pub fn is_key_just_pressed(&self, key: Key) -> bool {
        self.key_states.get(&key).map_or(false, |s| s.just_pressed)
    }

    pub fn is_key_just_released(&self, key: Key) -> bool {
        self.key_states.get(&key).map_or(false, |s| s.just_released)
    }

    pub fn get_mouse_movement(&mut self, xpos: f64, ypos: f64) -> (f32, f32) {
        let x_offset = xpos as f32 - self.last_x;
        let y_offset = self.last_y - ypos as f32;

        self.last_x = xpos as f32;
        self.last_y = ypos as f32;

        (x_offset, y_offset)
    }

    pub fn get_scroll_delta(&self) -> f32 {
        self.scroll_delta
    }

    pub fn reset_scroll_delta(&mut self) {
        self.scroll_delta = 0.0;
    }
}
