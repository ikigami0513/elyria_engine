#[link(name = "shell32")]
unsafe extern "C" {}

mod handler;
mod player;
mod core;
mod admin;

use std::sync::Arc;
use axum::{
    routing::get,
    Router
};
use std::net::SocketAddr;
use crate::player::{PlayerComponent, PlayerMoveHandler};
use crate::core::server::ElyriaServer;
use crate::admin::pages::show_players_page;


#[tokio::main]
async fn main() {
    let server = Arc::new(ElyriaServer::new());
    server.world.lock().await.register_component::<PlayerComponent>();
    server.handlers.lock().await.insert("player_move".to_string(), Box::new(PlayerMoveHandler));

    let game_server_clone = server.clone();

    tokio::spawn(async move {
        game_server_clone.run("127.0.0.1:8080").await;
    });

    println!("Démarrage du serveur web sur http://127.0.0.1:3000");

    let app = Router::new()
        .route("/", get(show_players_page))
        .with_state(server);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
