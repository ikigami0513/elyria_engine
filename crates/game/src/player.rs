use cgmath::{InnerSpace, Vector2};
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

pub struct PlayerComponent {
    pub speed: f32,
    pub direction: Direction
}

impl Component for PlayerComponent {}

pub struct PlayerSystem;

impl System for PlayerSystem {
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
                
                if ctx.input.is_key_pressed(Key::W) {
                    velocity.y = -1.0;
                    direction = Direction::UP;
                }
                if ctx.input.is_key_pressed(Key::S) {
                    velocity.y = 1.0;
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

                    let position = transform_component.transform.get_local_position_mut();
                    position.x += final_movement.x;
                    position.y += final_movement.y;
                }

                println!("{} {}", transform_component.transform.get_local_position().x, transform_component.transform.get_local_position().y);

                if direction != player_component.direction {
                    player_component.direction = direction;
                    if let Some(animation_comp) = ctx.world.get_component_mut::<AnimationComponent>(entity) {
                        animation_comp.play(format!("player_base_idle_{}", Direction::to_str(direction)).as_str());
                    }
                }
            }
        }
    }
}
