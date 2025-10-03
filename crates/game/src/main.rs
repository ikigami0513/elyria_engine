#[link(name = "shell32")]
unsafe extern "C" {}

mod player;
mod network;
mod tick;
mod gamestate;

use cgmath::vec3;
use common::message::Message;
use common::player::{Direction, State};
use engine::{core::application::Application, graphics::{animation::AnimationComponent, sprite::SpriteCreator}, world::components::TransformComponent};

use player::{LocalPlayerComponent, LocalPlayerSystem};
use crate::{
    gamestate::GameStateComponent, 
    network::{
        client::Client, 
        event::NetworkEvent, 
        handlers::{
            connected_handler::ConnectedHandler, 
            distant_player_moved::{
                DistantPlayerMovedHandler, 
                NewDistantPlayerHandler
            }
        }, 
        system::NetworkEventSystem
    }, 
    player::DistantPlayerSystem, 
    tick::TickSystem
};

use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (game_tx, mut network_rx) = mpsc::channel::<Message>(100);
    let (network_tx, game_rx) = mpsc::channel::<NetworkEvent>(100);

    let server_addr = "127.0.0.1:8080";
    match Client::connect(server_addr).await {
        Ok(mut client) => {
            println!("Connexion réussie !");

            // 2. On lance la tâche réseau en arrière-plan
            // On lui donne le récepteur du canal (rx)
            let network_tx_clone = network_tx.clone();
            tokio::spawn(async move {
                loop {
                    // tokio::select! attend que l'une des deux opérations se termine
                    tokio::select! {
                        // Cas A: On reçoit un message du jeu à envoyer au serveur
                        Some(message_to_send) = network_rx.recv() => {
                            if let Err(e) = client.send(&message_to_send).await {
                                eprintln!("Erreur lors de l'envoi du message: {}", e);
                                break; // On arrête la boucle en cas d'erreur
                            }
                        },

                        // Cas B: On reçoit un message du serveur
                        result = client.receive() => {
                             match result {
                                Ok(message) => {
                                    let event = NetworkEvent { data: message.get_data() };
                                    if let Err(e) = network_tx_clone.send(event).await {
                                        eprintln!("Impossible d'envoyer l'événement au jeu: {}", e);
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Erreur de réception: {}", e);
                                    break; // Sortir de la boucle si la connexion est perdue
                                }
                            }
                        }
                    }
                }
            });
        }
        Err(e) => {
            eprintln!("Impossible de se connecter au serveur: {}", e);
            return;
        }
    }

    let mut app = Application::new(1920, 1200, "Elyria");
    app.camera.zoom = 2.0;

    let mut network_system = Box::new(NetworkEventSystem::new(game_rx));
    network_system.handlers.insert("connected".to_string(), Box::new(ConnectedHandler));
    network_system.handlers.insert("player_moved".to_string(), Box::new(DistantPlayerMovedHandler));
    network_system.handlers.insert("new_distant_player".to_string(), Box::new(NewDistantPlayerHandler));

    app.systems.push(network_system);
    app.systems.push(Box::new(LocalPlayerSystem));
    app.systems.push(Box::new(DistantPlayerSystem));
    app.systems.push(Box::new(TickSystem::new(game_tx.clone())));
    app.world.register_component::<LocalPlayerComponent>();

    app.spritesheet_manager.load("resources/data/spritesheets/player_base.json").unwrap();

    // player base idle animation
    app.animation_manager.load("resources/data/animations/player_base_idle_down.json").unwrap();
    app.animation_manager.load("resources/data/animations/player_base_idle_left.json").unwrap();
    app.animation_manager.load("resources/data/animations/player_base_idle_right.json").unwrap();
    app.animation_manager.load("resources/data/animations/player_base_idle_up.json").unwrap();

    // player base walk animation
    app.animation_manager.load("resources/data/animations/player_base_walk_down.json").unwrap();
    app.animation_manager.load("resources/data/animations/player_base_walk_left.json").unwrap();
    app.animation_manager.load("resources/data/animations/player_base_walk_right.json").unwrap();
    app.animation_manager.load("resources/data/animations/player_base_walk_up.json").unwrap();

    let gamestate_entity = app.world.new_entity();
    app.world.add_component(gamestate_entity, GameStateComponent { player_id: None } );

    let container_entity = app.world.new_entity();
    let mut container_transform = TransformComponent::new();
    container_transform.transform.set_local_position(vec3(400.0, 300.0, 0.0));
    container_transform.transform.set_local_scale(vec3(0.1, 0.1, 0.1));

    app.world.add_component(container_entity, container_transform);
    app.world.add_component(container_entity, SpriteCreator::from_texture("resources/textures/container.jpg"));

    let player_entity = app.world.new_entity();
    let mut player_transform = TransformComponent::new();
    player_transform.transform.set_local_position(vec3(0.0, 0.0, 0.0));
    let mut anim_comp = AnimationComponent::new();
    anim_comp.play("player_base_idle_down"); 

    app.world.add_component(player_entity, player_transform);
    app.world.add_component(player_entity, SpriteCreator::from_sprite(app.spritesheet_manager.get("player_base").unwrap(), "idle_down_0").unwrap());
    app.world.add_component(player_entity, anim_comp);
    app.world.add_component(player_entity, LocalPlayerComponent { 
        speed: 100.0, 
        direction: Direction::DOWN,
        state: State::IDLE
    });

    app.camera.target = Some(player_entity);

    app.run();
}
