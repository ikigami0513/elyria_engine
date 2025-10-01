use engine::world::components::Component;
use uuid::Uuid;

pub struct GameStateComponent {
    pub player_id: Option<Uuid>
}

impl Component for GameStateComponent {
    
}
