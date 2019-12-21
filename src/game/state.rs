use super::field::Field;
use super::player::Player;
use super::types::Id;
use super::Game;

pub trait GameState {
    fn get_field(&self) -> &Field;
    fn get_player(&self, id: Id) -> &Player;
    fn get_player_target(&self, id: Id) -> &Player;
    fn get_player_target_id(&self, id: Id) -> Id;
}

fn target_id(id: Id) -> Id {
    let result = if id == 0 { 1 } else { 0 };
    result
}

impl GameState for Game {
    fn get_field(&self) -> &Field {
        &self.field
    }

    fn get_player(&self, id: Id) -> &Player {
        &self.players[id]
    }

    fn get_player_target(&self, id: Id) -> &Player {
        &self.players[target_id(id)]
    }

    fn get_player_target_id(&self, id: Id) -> Id {
        target_id(id)
    }
}
