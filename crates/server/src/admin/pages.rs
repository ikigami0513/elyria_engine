use std::sync::Arc;
use common::player::PlayerInfo;
use engine::world::components::TransformComponent;

use axum::extract::State;

use askama::Template;
use crate::player::PlayerComponent;
use crate::core::server::ElyriaServer;

#[derive(Template)]
#[template(path="players.html")]
pub struct PlayersTemplate {
    title: String,
    players: Vec<PlayerInfo>
}

pub async fn show_players_page(State(server): State<Arc<ElyriaServer>>) -> PlayersTemplate {
    let mut players_info = Vec::new();
    let world = server.world.lock().await;

    if let Some(player_components) = world.get_components::<PlayerComponent>() {
        for (entity, player_comp) in player_components.iter() {
            if let Some(transform_comp) = world.get_component::<TransformComponent>(*entity) {
                players_info.push(PlayerInfo {
                    id: player_comp.id.to_string(),
                    x: transform_comp.transform.get_local_position().x,
                    y: transform_comp.transform.get_local_position().y,
                    z: transform_comp.transform.get_local_position().z
                });
            }
        }
    }

    PlayersTemplate {
        title: "Liste des joueurs connectés".to_string(),
        players: players_info
    }
}