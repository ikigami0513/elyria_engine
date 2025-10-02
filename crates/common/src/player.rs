use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PlayerInfo {
    pub id: String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}