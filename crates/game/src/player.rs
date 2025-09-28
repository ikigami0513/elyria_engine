use cgmath::{vec3, InnerSpace, Vector2};
use engine::{graphics::animation::AnimationComponent, world::{components::{Component, TransformComponent}, entity::Entity, system::System}};
use glfw::Key;

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    DOWN,
    UP,
    RIGHT,
    LEFT
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

#[derive(Clone, Copy, PartialEq)]
pub enum State {
    IDLE,
    WALK
}

impl State {
    pub fn to_str(state: State) -> &'static str {
        match state {
            State::IDLE => "idle",
            State::WALK => "walk"
        }
    }
}

pub struct PlayerComponent {
    pub speed: f32,
    pub direction: Direction,
    pub state: State
}

impl Component for PlayerComponent {}

pub struct PlayerSystem;

impl System for PlayerSystem {
    #[allow(unused_assignments)]
    fn update(&mut self, ctx: &mut engine::core::frame_context::FrameContext) {
        let mut target_entities: Vec<Entity> = Vec::new();
        if let (Some(players), Some(transforms)) = (
            ctx.world.get_components::<PlayerComponent>(),
            ctx.world.get_components::<TransformComponent>(),
        ) {
            for (entity, _player) in players.iter() {
                if transforms.contains_key(entity) {
                    target_entities.push(*entity);
                }
            }
        }

        for entity in target_entities {
            if let Some((player_component, transform_component)) = ctx
                .world
                .get_components_mut_pair::<PlayerComponent, TransformComponent>(entity)
            {
                let mut velocity = Vector2::new(0.0, 0.0);
                let mut direction = player_component.direction.clone();
                let mut state = player_component.state.clone();
                
                if ctx.input.is_key_pressed(Key::W) {
                    velocity.y = 1.0;
                    direction = Direction::UP;
                }
                if ctx.input.is_key_pressed(Key::S) {
                    velocity.y = -1.0;
                    direction = Direction::DOWN;
                }

                if ctx.input.is_key_pressed(Key::A) {
                    velocity.x = -1.0;
                    direction = Direction::LEFT;
                }
                if ctx.input.is_key_pressed(Key::D) {
                    velocity.x = 1.0;
                    direction = Direction::RIGHT;
                }
                
                if velocity.magnitude2() > 0.0 {
                    let final_movement = velocity.normalize() * player_component.speed * ctx.time.delta_time();

                    let position = transform_component.transform.get_local_position();
                    transform_component.transform.set_local_position(vec3(
                        position.x + final_movement.x, 
                        position.y + final_movement.y, 
                        position.z
                    ));

                    state = State::WALK;
                }
                else {
                    state = State::IDLE;
                }

                if direction != player_component.direction || state != player_component.state {
                    player_component.direction = direction;
                    player_component.state = state;
                    if let Some(animation_comp) = ctx.world.get_component_mut::<AnimationComponent>(entity) {
                        animation_comp.play(format!("player_base_{}_{}", State::to_str(state), Direction::to_str(direction)).as_str());
                    }
                }
            }
        }
    }
}
