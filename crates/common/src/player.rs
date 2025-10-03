use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    DOWN,
    UP,
    RIGHT,
    LEFT
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "down" => Ok(Direction::DOWN),
            "up" => Ok(Direction::UP),
            "right" => Ok(Direction::RIGHT),
            "left" => Ok(Direction::LEFT),
            _ => Err(())
        }
    }
}

impl Direction {
    pub fn to_str(direction: Direction) -> &'static str {
        match direction {
            Direction::DOWN => "down",
            Direction::UP => "up",
            Direction::RIGHT => "right",
            Direction::LEFT => "left",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum State {
    IDLE,
    WALK
}

impl FromStr for State {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "idle" => Ok(State::IDLE),
            "walk" => Ok(State::WALK),
            _ => Err(()),
        }
    }
}

impl State {
    pub fn to_str(state: State) -> &'static str {
        match state {
            State::IDLE => "idle",
            State::WALK => "walk"
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PlayerInfo {
    pub id: String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}