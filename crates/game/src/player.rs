use cgmath::{vec3, InnerSpace, Vector2, Vector3};
use engine::{graphics::animation::AnimationComponent, world::{components::{Component, TransformComponent}, entity::Entity, system::System}};
use glfw::Key;
use uuid::Uuid;

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

pub struct LocalPlayerComponent {
    pub speed: f32,
    pub direction: Direction,
    pub state: State
}

impl Component for LocalPlayerComponent {}

pub struct LocalPlayerSystem;

impl System for LocalPlayerSystem {
    #[allow(unused_assignments)]
    fn update(&mut self, ctx: &mut engine::core::frame_context::FrameContext) {
        let mut target_entities: Vec<Entity> = Vec::new();
        if let (Some(players), Some(transforms)) = (
            ctx.world.get_components::<LocalPlayerComponent>(),
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
                .get_components_mut_pair::<LocalPlayerComponent, TransformComponent>(entity)
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

pub struct DistantPlayerComponent {
    pub player_id: Uuid,
    pub speed: f32,
    pub direction: Direction,
    pub state: State,
    pub target_position: Option<Vector3<f32>>
}

impl Component for DistantPlayerComponent {}

pub struct DistantPlayerSystem;

#[allow(unused_assignments)]
impl System for DistantPlayerSystem {
    fn update(&mut self, ctx: &mut engine::core::frame_context::FrameContext) {
        let mut target_entities: Vec<Entity> = Vec::new();
        if let Some(distant_players) = ctx.world.get_components::<DistantPlayerComponent>() {
            target_entities.extend(distant_players.keys().copied());
        }

        for entity in target_entities {
            if let Some((distant_player_comp, transform_comp)) = ctx.world.get_components_mut_pair::<DistantPlayerComponent, TransformComponent>(entity) {
                if let Some(target_pos) = distant_player_comp.target_position {
                    let current_pos = transform_comp.transform.get_local_position();
                    let direction_vector = target_pos - current_pos;
                    let distance_to_target = direction_vector.magnitude();

                    let max_move_this_frame = distant_player_comp.speed * ctx.time.delta_time();

                    let mut new_state = distant_player_comp.state;
                    let mut new_direction = distant_player_comp.direction;

                    if distance_to_target <= max_move_this_frame {
                        transform_comp.transform.set_local_position(target_pos);
                        distant_player_comp.target_position = None;
                        new_state = State::IDLE;
                    }
                    else {
                        let movement = direction_vector.normalize() * max_move_this_frame;
                        transform_comp.transform.set_local_position(current_pos + movement);
                        new_state = State::WALK;

                        if movement.x.abs() > movement.y.abs() {
                            new_direction = if movement.x > 0.0 { Direction::RIGHT } else { Direction::LEFT };
                        }
                        else {
                            new_direction = if movement.y > 0.0 { Direction::UP } else { Direction::DOWN };
                        }
                    }

                    if new_state != distant_player_comp.state || new_direction != distant_player_comp.direction {
                        distant_player_comp.state = new_state;
                        distant_player_comp.direction = new_direction;

                        if let Some(animation_comp) = ctx.world.get_component_mut::<AnimationComponent>(entity) {
                            let anim_name = format!("player_base_{}_{}", State::to_str(new_state), Direction::to_str(new_direction));
                            animation_comp.play(&anim_name);
                        }
                    }
                }
            }
        }
    }
}
