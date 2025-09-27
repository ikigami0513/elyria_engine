#[link(name = "shell32")]
unsafe extern "C" {}

use cgmath::vec3;
use engine::{core::application::Application, graphics::{animation::AnimationComponent, sprite::SpriteCreator}, world::components::{Parent, TransformComponent}};

fn main() {
    let mut app = Application::new(1920, 1200, "Elyria");

    let _ = app.spritesheet_manager.load("resources/data/spritesheets/player_base.json");
    let _ = app.animation_manager.load("resources/data/animations/player_base_idle_down.json");

    let root_entity = app.world.new_entity();
    app.world.add_component(root_entity, TransformComponent::new());

    let container_entity = app.world.new_entity();
    let mut container_transform = TransformComponent::new();
    container_transform.transform.set_local_position(vec3(400.0, 300.0, 0.0));
    container_transform.transform.set_local_scale(vec3(0.1, 0.1, 0.1));

    app.world.add_component(container_entity, container_transform);
    app.world.add_component(container_entity, SpriteCreator::from_texture("resources/textures/container.jpg"));
    app.world.add_component(container_entity, Parent(root_entity));

    let player_entity = app.world.new_entity();
    let mut player_transform = TransformComponent::new();
    player_transform.transform.set_local_position(vec3(200., 200., 0.0));
    app.world.add_component(player_entity, player_transform);
    app.world.add_component(player_entity, SpriteCreator::from_sprite(app.spritesheet_manager.get("player_base").unwrap(), "idle_down_0").unwrap());
    let mut anim_comp = AnimationComponent::new();
    anim_comp.play("player_base_idle_down"); 
    app.world.add_component(player_entity, anim_comp);

    app.run();
}
